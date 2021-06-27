//
//  TerrainPipeline.swift
//  Enchantron
//
//  Created by Kevin Guthrie on 1/9/21.
//  Copyright © 2021 Rook And Pawn Industries, Inc. All rights reserved.
//

import Metal
import MetalKit
import simd

class TerrainPipeline {
    
    class func buildMetalVertexDescriptor() -> MTLVertexDescriptor {
        let mtlVertexDescriptor = MTLVertexDescriptor()

        return mtlVertexDescriptor
    }
    
    
    class func buildRenderPipelineWithDevice(
        device: MTLDevice,
        metalKitView: MTKView,
        mtlVertexDescriptor: MTLVertexDescriptor) throws -> MTLRenderPipelineState {
        
        let libraryName : String
        
        // The metal function fma (Fused Multiply Add) does not seem to work on Radeon
        // graphics chips, so there are two different terrain shaders. One with and one
        // without
        if (device.name.contains("adeon")) {
            libraryName = "TerrainWithoutFma"
        } else {
            libraryName = "TerrainWithFma"
        }
        
        let libraryFile = Bundle.main.path(forResource: libraryName, ofType: "metallib")!
        
        let library = try device.makeLibrary(filepath: libraryFile)
        
        let vertexFunction = library.makeFunction(name: "vertexShader")
        let fragmentFunction = library.makeFunction(name: "fragmentShader")
        
        let pipelineDescriptor = MTLRenderPipelineDescriptor()
        pipelineDescriptor.label = "TerrainRenderPipeline"
        pipelineDescriptor.sampleCount = metalKitView.sampleCount
        pipelineDescriptor.vertexFunction = vertexFunction
        pipelineDescriptor.fragmentFunction = fragmentFunction
        pipelineDescriptor.vertexDescriptor = mtlVertexDescriptor
        
        pipelineDescriptor.colorAttachments[0].pixelFormat = metalKitView.colorPixelFormat
        pipelineDescriptor.colorAttachments[0].isBlendingEnabled = false
        
        pipelineDescriptor.depthAttachmentPixelFormat = metalKitView.depthStencilPixelFormat
        pipelineDescriptor.stencilAttachmentPixelFormat = metalKitView.depthStencilPixelFormat
        
        return try device.makeRenderPipelineState(descriptor: pipelineDescriptor)
    }
    
    let device: MTLDevice
    let pipelineState: MTLRenderPipelineState
    
    init(device: MTLDevice, view: MTKView) {
        self.device = device
        self.pipelineState = try! TerrainPipeline.buildRenderPipelineWithDevice(
            device: device,
            metalKitView: view,
            mtlVertexDescriptor: TerrainPipeline.buildMetalVertexDescriptor())
        
    }
    
    func encode(
        encoder: MTLRenderCommandEncoder,
        viewport: Viewport,
        uniformBufferIndex: Int,
        time: Float64
    ) {
        encoder.setRenderPipelineState(pipelineState)
        
        Sprite.setUpForSpriteRendering(encoder: encoder)
        
        viewport.bindToFragmentShader(
            encoder: encoder,
            uniformBufferIndex: uniformBufferIndex,
            bufferIndex: 1)
        
        encoder.drawIndexedPrimitives(
            type: .triangle,
            indexCount: Sprite.indexes.count,
            indexType: .uint16,
            indexBuffer: Sprite.indexBuffer!,
            indexBufferOffset: 0)
    }
}
