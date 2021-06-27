//
//  SpritePipeline.swift
//  Enchantron
//
//  Created by Kevin Guthrie on 1/9/21.
//  Copyright © 2021 Rook And Pawn Industries, Inc. All rights reserved.
//

import Metal
import MetalKit
import simd

class SpritePipeline {
    
    class func buildMetalVertexDescriptor() -> MTLVertexDescriptor {
        let mtlVertexDescriptor = MTLVertexDescriptor()
        
        
        return mtlVertexDescriptor
    }
    
    
    class func buildRenderPipelineWithDevice(
        device: MTLDevice,
        metalKitView: MTKView,
        mtlVertexDescriptor: MTLVertexDescriptor) throws -> MTLRenderPipelineState {
        
        let libraryFile = Bundle.main.path(forResource: "SpriteShaders", ofType: "metallib")!
        
        let library = try device.makeLibrary(filepath: libraryFile)
        
        let vertexFunction = library.makeFunction(name: "spriteVertexShader")
        let fragmentFunction = library.makeFunction(name: "spriteFragmentShader")
        
        let pipelineDescriptor = MTLRenderPipelineDescriptor()
        pipelineDescriptor.label = "SpriteRenderPipeline"
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
    
    let device: MTLDevice
    let pipelineState: MTLRenderPipelineState
    
    init(device: MTLDevice, view: MTKView) {
        self.device = device
        self.pipelineState = try! SpritePipeline.buildRenderPipelineWithDevice(
            device: device,
            metalKitView: view,
            mtlVertexDescriptor: SpritePipeline.buildMetalVertexDescriptor())
        
    }
    
    func encode(
        encoder: MTLRenderCommandEncoder,
        view: NativeView,
        uniformBufferIndex: Int,
        time: Float64
    ) {
        encoder.setRenderPipelineState(pipelineState)
        view.render(
            encoder: encoder,
            uniformBufferIndex: uniformBufferIndex,
            time: time)
    }
}
