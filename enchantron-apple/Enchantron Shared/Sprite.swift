//
//  Sprite.swift
//  FourFours
//
//  Created by Kevin Guthrie on 8/23/18.
//  Copyright © 2018 Rook And Pawn Industries, Inc. All rights reserved.
//

import Foundation
import Metal
import MetalKit
import simd

// The 256 byte aligned size of our uniform structure
fileprivate let alignedUniformsSize = (MemoryLayout<SpriteUniform>.size + 0xFF) & -0x100

class Sprite {
    
    static let indexes : [UInt16] = [
        0, 2, 1,
        1, 2, 3
    ]
    
    static let indexesSize = indexes.count * MemoryLayout<UInt16>.stride
    
    static var vertexBuffer: MTLBuffer?
    static var indexBuffer: MTLBuffer?
    
    static func staticInit(device: MTLDevice) {
        indexBuffer = device.makeBuffer(
            bytes: indexes,
            length: indexesSize,
            options: [])!
    }
    
    
    static func setUpForSpriteRendering(encoder: MTLRenderCommandEncoder) {
        encoder.setVertexBuffer(vertexBuffer, offset: 0, index: 0)

    }
    
    var texture: Texture?
    var size = CGSize()
    var uniformBuffer: MTLBuffer
    var uniforms: UnsafeMutablePointer<SpriteUniform>
    var visible = false
    var topLeftMajor = SIMD2<Float32>()
    var topLeftMinor = SIMD2<Float32>()
    
    weak var container : SpriteGroup?
    
    init(device: MTLDevice, container: SpriteGroup, texture: Texture?) {
        self.texture = texture
        uniformBuffer = device.makeBuffer(
            length: alignedUniformsSize * maxBuffersInFlight,
            options: [])!
        uniforms = UnsafeMutableRawPointer(uniformBuffer.contents())
            .bindMemory(to:SpriteUniform.self, capacity:1)
        self.container = container
    }
    
    required init?(coder aDecoder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }
    
    func animate(_ animation: Animation, _ secsPerFrame: Float64) {
        
    }
    
    func clearAnimations() {
        
    }
    
    func setVisible(_ visible: Bool) {
        DispatchQueue.main.async {
            self.visible = visible
        }
    }
    
    func setTexture(_ texture: Texture) {
        DispatchQueue.main.async {
            self.texture = texture
        }
    }
    
    func setZLevel(_ newZLevel: Double) {
        
    }
    
    func setSizeAnimated(_ width: Float64, _ height: Float64, _ durationSeconds: Float64) {
        
        if durationSeconds > 0.0 {
            
        }
        else {
            DispatchQueue.main.async {
                self.size = CGSize(width: width, height: height)
            }
        }
        
    }
    
    func setLocationAnimated(_ left: Float64, _ top: Float64, _ durationSeconds: Float64) {
        
        if durationSeconds > 0.0 {
            
        }
        else {
            DispatchQueue.main.async {
                
                let (topLeftMajor, topLeftMinor) = PointUtil.toMajorMinor(
                    x: left,
                    y: top)
                
                self.topLeftMajor = topLeftMajor
                self.topLeftMinor = topLeftMinor
            }
        }
    }
    
    func removeFromParent() {
        DispatchQueue.main.async {
            self.container?.removeChild(sprite: self)
        }
    }
    
    private func updateDynamicBufferState(uniformBufferIndex: Int) {
        
        let uniformBufferOffset = alignedUniformsSize * uniformBufferIndex
        
        uniforms = UnsafeMutableRawPointer(uniformBuffer.contents() + uniformBufferOffset)
            .bindMemory(to:SpriteUniform.self, capacity:1)
    }
    
    func render(encoder: MTLRenderCommandEncoder, uniformBufferIndex: Int) {
        updateDynamicBufferState(uniformBufferIndex: uniformBufferIndex)
        
        uniforms[0].topLeftMajor = topLeftMajor
        uniforms[0].topLeftMinor = topLeftMinor
        
        uniforms[0].size = [Float32(self.size.width), Float32(self.size.height)]
        texture?.fillSpriteUniformUvs(uniforms: uniforms)
        let uniformBufferOffset = uniformBufferIndex * alignedUniformsSize
        
        encoder.setVertexBuffer(uniformBuffer, offset: uniformBufferOffset, index: 0)
        encoder.setFragmentTexture(texture!.wrapped, index: 0)
        encoder.drawIndexedPrimitives(
            type: .triangle,
            indexCount: Sprite.indexes.count,
            indexType: .uint16,
            indexBuffer: Sprite.indexBuffer!,
            indexBufferOffset: 0)
    }
    
    deinit {
        print("Dropping Sprite")
    }
}

extension Sprite : Equatable {
    
    static func ==(lhs: Sprite, rhs: Sprite) -> Bool {
        return lhs === rhs
    }
}
