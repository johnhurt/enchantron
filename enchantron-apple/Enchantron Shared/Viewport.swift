//
//  Viewport.swift
//  Enchantron iOS
//
//  Created by Kevin Guthrie on 6/8/19.
//  Copyright © 2019 Rook And Pawn Industries, Inc. All rights reserved.
//

import Foundation
import Metal
import MetalKit
import simd

// The 256 byte aligned size of our uniform structure
fileprivate let alignedUniformsSize = (MemoryLayout<ViewportUniform>.size + 0xFF) & -0x100

public class Viewport {
    
    var screenSize : CGSize
    var topLeftMajor = SIMD2<Float32>()
    var topLeftMinor = SIMD2<Float32>()
    var scale : Float32 = 1.0
    
    var uniformBuffer: MTLBuffer
    var uniforms: UnsafeMutablePointer<ViewportUniform>
    var viewLockedSprites: SpriteGroup
    
    init(screenSize: CGSize, device: MTLDevice) {
        uniformBuffer = device.makeBuffer(
            length: alignedUniformsSize * maxBuffersInFlight,
            options: [])!
        
        uniforms = UnsafeMutableRawPointer(uniformBuffer.contents())
            .bindMemory(to:ViewportUniform.self, capacity:1)
        
        viewLockedSprites = SpriteGroup(device: device, parent: nil)
        self.screenSize = screenSize
    }
    
    func reset() {
        self.scale = 1.0
        self.topLeftMajor = SIMD2<Float32>()
        self.topLeftMinor = SIMD2<Float32>()
    }
    
    
    func setScale(_ newScale: Float64) {
        DispatchQueue.main.async {
            self.scale = Float32(newScale)
        }
    }
    
    
    func setScaleAndLocation(
        _ newScale: Float64,
        _ newTopLeftX: Float64,
        _ newTopLeftY: Float64) {
        
        DispatchQueue.main.async {
            self.scale = Float32(newScale)
            
            let (topLeftMajor, topLeftMinor) = PointUtil.toMajorMinor(
                x: newTopLeftX,
                y: newTopLeftY)
            
            self.topLeftMajor = topLeftMajor
            self.topLeftMinor = topLeftMinor
        }
        
    }
    
    func setLocationAnimated(_ left: Float64, _ top: Float64, _ durationSeconds: Float64) {
        DispatchQueue.main.async {
            let (topLeftMajor, topLeftMinor) = PointUtil.toMajorMinor(
                x: left,
                y: top)
            
            self.topLeftMajor = topLeftMajor
            self.topLeftMinor = topLeftMinor
        }
    }
    
    func configureViewport(encoder: MTLRenderCommandEncoder, uniformBufferIndex: Int) {

        let uniformBufferOffset = alignedUniformsSize * uniformBufferIndex
        
        uniforms = UnsafeMutableRawPointer(uniformBuffer.contents() + uniformBufferOffset)
            .bindMemory(to:ViewportUniform.self, capacity:1)
        
        self.uniforms[0].screenSize = [Float(screenSize.width), Float(screenSize.height)]
        self.uniforms[0].topLeftMajor = self.topLeftMajor
        self.uniforms[0].topLeftMinor = self.topLeftMinor
        self.uniforms[0].scale = Float(scale)
        
        encoder.setVertexBuffer(uniformBuffer, offset: uniformBufferOffset, index: 1)
    }
    
    func render(encoder: MTLRenderCommandEncoder, uniformBufferIndex: Int) {
        self.viewLockedSprites.render(encoder: encoder, uniformBufferIndex: uniformBufferIndex)
    }
    
    func setVisible(_ visible: Bool) {
        self.viewLockedSprites.setVisible(visible)
    }
}

extension Viewport : SpriteSource {
    func createSprite() -> Sprite {
        return createSpriteOn(parent: self.viewLockedSprites)
    }
    
    func createGroup() -> SpriteGroup {
        return createGroupOn(parent: self.viewLockedSprites)
    }
}
