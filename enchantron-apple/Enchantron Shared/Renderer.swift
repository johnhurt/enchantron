//
//  Renderer.swift
//  Enchantron
//
//  Created by Kevin Guthrie on 11/30/20.
//  Copyright © 2020 Rook And Pawn Industries, Inc. All rights reserved.
//

import Metal
import MetalKit
import simd

let maxBuffersInFlight = 3

enum RendererError: Error {
    case badVertexDescriptor
}

class Renderer: NSObject, MTKViewDelegate {
    
    public let device: MTLDevice
    let commandQueue: MTLCommandQueue
    var pipelineState: MTLRenderPipelineState
    var depthState: MTLDepthStencilState
    
    let inFlightSemaphore = DispatchSemaphore(value: maxBuffersInFlight)
    
    var uniformBufferIndex = 0
    
    private var currentView : BaseView
    private var viewport : Viewport
    var screenHeight : Float64 = 0
    
    init?(metalKitView: MTKView) {
        self.device = metalKitView.device!
        guard let queue = self.device.makeCommandQueue() else { return nil }
        self.commandQueue = queue
        
        
        metalKitView.depthStencilPixelFormat = MTLPixelFormat.depth32Float_stencil8
        metalKitView.colorPixelFormat = MTLPixelFormat.bgra8Unorm_srgb
        metalKitView.sampleCount = 1
        
        let mtlVertexDescriptor = Renderer.buildMetalVertexDescriptor()
        
        do {
            pipelineState = try Renderer.buildRenderPipelineWithDevice(device: device,
                                                                       metalKitView: metalKitView,
                                                                       mtlVertexDescriptor: mtlVertexDescriptor)
        } catch {
            print("Unable to compile render pipeline state.  Error info: \(error)")
            return nil
        }
        
        let depthStateDesciptor = MTLDepthStencilDescriptor()
        depthStateDesciptor.depthCompareFunction = MTLCompareFunction.lessEqual
        depthStateDesciptor.isDepthWriteEnabled = true
        guard let state = device.makeDepthStencilState(descriptor:depthStateDesciptor) else { return nil }
        depthState = state
        
        Sprite.staticInit(device: device)
        
        let systemView = SystemView(
            resourceLoader: ResourceLoader(loader: MTKTextureLoader(device: device)))
        
        self.viewport = Viewport(device: device)
        
        let ctx = RustBinder.bindToRust(systemView)
        
        let loadingView = LoadingView(viewport: viewport, device: device)
        currentView = loadingView
        
        super.init()
        
        let transitioner = TransitionService(preBindTransition: { (view) in
            self.viewport.reset()
        }, postBindTransition: { (view) in
            self.currentView = view
        })
        
        loadingView.initializeCtx(ctx: ctx, transitionService: transitioner)
        ctx.transitionToLoadingView(view: loadingView)
        
        
        
    }
    
    class func buildMetalVertexDescriptor() -> MTLVertexDescriptor {
        let mtlVertexDescriptor = MTLVertexDescriptor()
        
        
        return mtlVertexDescriptor
    }
    
    class func buildRenderPipelineWithDevice(device: MTLDevice,
                                             metalKitView: MTKView,
                                             mtlVertexDescriptor: MTLVertexDescriptor) throws -> MTLRenderPipelineState {
        /// Build a render state pipeline object
        
        let library = device.makeDefaultLibrary()
        
        let vertexFunction = library?.makeFunction(name: "vertexShader")
        let fragmentFunction = library?.makeFunction(name: "fragmentShader")
        
        let pipelineDescriptor = MTLRenderPipelineDescriptor()
        pipelineDescriptor.label = "RenderPipeline"
        pipelineDescriptor.sampleCount = metalKitView.sampleCount
        pipelineDescriptor.vertexFunction = vertexFunction
        pipelineDescriptor.fragmentFunction = fragmentFunction
        pipelineDescriptor.vertexDescriptor = mtlVertexDescriptor
        
        pipelineDescriptor.colorAttachments[0].pixelFormat = metalKitView.colorPixelFormat
        pipelineDescriptor.colorAttachments[0].isBlendingEnabled = true
        pipelineDescriptor.colorAttachments[0].rgbBlendOperation = .add
        pipelineDescriptor.colorAttachments[0].alphaBlendOperation = .add
        pipelineDescriptor.colorAttachments[0].sourceAlphaBlendFactor = .sourceAlpha
        pipelineDescriptor.colorAttachments[0].sourceRGBBlendFactor = .sourceAlpha
        pipelineDescriptor.colorAttachments[0].destinationRGBBlendFactor = .oneMinusSourceAlpha
        pipelineDescriptor.colorAttachments[0].destinationAlphaBlendFactor = .oneMinusSourceAlpha
        
        pipelineDescriptor.depthAttachmentPixelFormat = metalKitView.depthStencilPixelFormat
        pipelineDescriptor.stencilAttachmentPixelFormat = metalKitView.depthStencilPixelFormat
        
        return try device.makeRenderPipelineState(descriptor: pipelineDescriptor)
    }
    
    class func loadTexture(device: MTLDevice,
                           textureName: String) throws -> MTLTexture {
        /// Load texture data with optimal parameters for sampling

        let url = Bundle.main.url(forResource: "gist", withExtension: "png")!
        
        let textureLoader = MTKTextureLoader(device: device)
        
        let textureLoaderOptions = [
            MTKTextureLoader.Option.textureUsage: NSNumber(value: MTLTextureUsage.shaderRead.rawValue),
            MTKTextureLoader.Option.textureStorageMode: NSNumber(value: MTLStorageMode.`private`.rawValue)
        ]
        
        
        let result = try textureLoader.newTexture(URL: url, options: textureLoaderOptions)
        
        return result;
    }
    
    private func updateDynamicBufferState() {
        /// Update the state of our uniform buffers before rendering
        
        uniformBufferIndex = (uniformBufferIndex + 1) % maxBuffersInFlight
    }
    
    
    func draw(in view: MTKView) {
        /// Per frame updates hare
        
        _ = inFlightSemaphore.wait(timeout: DispatchTime.distantFuture)
        
        if let commandBuffer = commandQueue.makeCommandBuffer() {
            
            let semaphore = inFlightSemaphore
            commandBuffer.addCompletedHandler { (_ commandBuffer)-> Swift.Void in
                semaphore.signal()
            }
            
            
            /// Delay getting the currentRenderPassDescriptor until we absolutely need it to avoid
            ///   holding onto the drawable and blocking the display pipeline any longer than necessary
            let renderPassDescriptor = view.currentRenderPassDescriptor
            //renderPassDescriptor?.colorAttachments[0].loadAction = .clear
            renderPassDescriptor?.colorAttachments[0].clearColor = MTLClearColorMake(0.1, 0.1, 0.12, 1.0)
            //renderPassDescriptor?.colorAttachments[0].storeAction = .store
            
            if let renderPassDescriptor = renderPassDescriptor, let renderEncoder = commandBuffer.makeRenderCommandEncoder(descriptor: renderPassDescriptor) {
                
                
                /// Final pass rendering code here
                renderEncoder.label = "Primary Render Encoder"
                
                renderEncoder.pushDebugGroup("Draw Box")
                
                renderEncoder.setCullMode(.back)
                
                updateDynamicBufferState()
                //updateGameState()
                
                
                renderEncoder.setFrontFacing(.counterClockwise)
                
                renderEncoder.setRenderPipelineState(pipelineState)
                
                renderEncoder.setDepthStencilState(depthState)
                

                Sprite.setUpForSpriteRendering(encoder: renderEncoder)
                currentView.rootGroup.render(
                    encoder: renderEncoder,
                    uniformBufferIndex: uniformBufferIndex)
                
                renderEncoder.popDebugGroup()
                
                renderEncoder.endEncoding()
                
                if let drawable = view.currentDrawable {
                    commandBuffer.present(drawable)
                }
            }
            
            commandBuffer.commit()
        }
    }
    
    func mtkView(_ view: MTKView, drawableSizeWillChange size: CGSize) {
        self.viewport.screenSize = size
        self.screenHeight = Float64(size.height)
    }
    
    func magnify(scaleChangeAdditive: Float64, centerPoint: SIMD2<Float64>) {
        DispatchQueue.main.async {
            self.currentView.magnify(
                scaleChangeAdditive: scaleChangeAdditive,
                centerPoint: centerPoint)
        }
    }
    
    func dragsStart(touches: [(Int64, TouchType)]) {
        switch touches.count {
        case 1:
            let (id, touch) = touches[0]
            let globalPoint = touch.getTouchLocation()
            currentView.oneDragStarted(
                id: id,
                globalPoint: globalPoint,
                clickCount: touch.getTapCount())
        
        case 2:
            let (id1, touch1) = touches[0]
            let (id2, touch2) = touches[1]
            
            let globalPoint1 = touch1.getTouchLocation()
            let globalPoint2 = touch2.getTouchLocation()
            
            currentView.twoDragsStarted(
                id1: id1,
                globalPoint1: globalPoint1,
                clickCount1: touch1.getTapCount(),
                id2: id2,
                globalPoint2: globalPoint2,
                clickCount2: touch2.getTapCount())
            
        default: do {}
        }
    }
    
    func dragsMoved(touches: [(Int64, TouchType)]) {
        switch touches.count {
        case 1:
            let (id, touch) = touches[0]
            let globalPoint = touch.getTouchLocation()
            currentView.oneDragMoved(
                id: id,
                globalPoint: globalPoint,
                clickCount: touch.getTapCount())
        
        case 2:
            let (id1, touch1) = touches[0]
            let (id2, touch2) = touches[1]
            
            let globalPoint1 = touch1.getTouchLocation()
            let globalPoint2 = touch2.getTouchLocation()
            
            currentView.twoDragsMoved(
                id1: id1,
                globalPoint1: globalPoint1,
                clickCount1: touch1.getTapCount(),
                id2: id2,
                globalPoint2: globalPoint2,
                clickCount2: touch2.getTapCount())
            
        default: do {}
        }
    }
    
    func dragsEnded(touches: [(Int64, TouchType)]) {
        switch touches.count {
            
        case 1:
            let (id, touch) = touches[0]
            let globalPoint = touch.getTouchLocation()
            currentView.oneDragEnded(
                id: id,
                globalPoint: globalPoint,
                clickCount: touch.getTapCount())
        
        case 2:
            let (id1, touch1) = touches[0]
            let (id2, touch2) = touches[1]
            
            let globalPoint1 = touch1.getTouchLocation()
            let globalPoint2 = touch2.getTouchLocation()
            
            currentView.twoDragsEnded(
                id1: id1,
                globalPoint1: globalPoint1,
                clickCount1: touch1.getTapCount(),
                id2: id2,
                globalPoint2: globalPoint2,
                clickCount2: touch2.getTapCount())
            
        default: do {}
        }
    }
    
}

