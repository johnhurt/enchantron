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
    var topLeft = SIMD2<Float64>()
    var topLeftMajor = SIMD2<Float32>()
    var topLeftMinor = SIMD2<Float32>()
    var color = SIMD4<Float>()
    var textureAnimation: TextureAnimation?
    var locationAnimation: LocationAnimation?
    var sizeAnimation: SizeAnimation?
    
    weak var container : SpriteGroup?
    
    init(device: MTLDevice, container: SpriteGroup) {
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
        DispatchQueue.main.async {
            self.textureAnimation = TextureAnimation(
                startTime: CACurrentMediaTime(),
                animation: animation,
                secsPerFrame: secsPerFrame)
        }
    }
    
    func clearAnimations() {
        DispatchQueue.main.async {
            self.textureAnimation = nil
        }
    }
    
    func setVisible(_ visible: Bool) {
        DispatchQueue.main.async {
            self.visible = visible
        }
    }
    
    func setTexture(_ texture: Texture) {
        DispatchQueue.main.async {
            self.texture = texture
            self.textureAnimation = nil
        }
    }
    
    func setZLevel(_ newZLevel: Double) {
        
    }
    
    func setSizeAnimated(_ width: Float64, _ height: Float64, _ durationSeconds: Float64) {
        
        if durationSeconds > 0.0 {
            let now = CACurrentMediaTime()
            
            DispatchQueue.main.async {
                self.sizeAnimation = SizeAnimation(
                    startSize: self.size,
                    finalSize: CGSize(width: CGFloat(width), height: CGFloat(height)),
                    startTime: now,
                    endTime: now + durationSeconds
                )
            }
        }
        else {
            DispatchQueue.main.async {
                self.size = CGSize(width: width, height: height)
            }
        }
        
    }
    
    private func setLocation(_ location: SIMD2<Float64>) {
        let (topLeftMajor, topLeftMinor) = PointUtil.toMajorMinor(
            x: location.x,
            y: location.y)
        self.topLeft = location
        self.topLeftMajor = topLeftMajor
        self.topLeftMinor = topLeftMinor
    }
    
    func setLocationAnimated(_ left: Float64, _ top: Float64, _ durationSeconds: Float64) {

        if durationSeconds > 0.0 {
            let now = CACurrentMediaTime()
            
            DispatchQueue.main.async {
                self.locationAnimation = LocationAnimation(
                    startLocation: self.topLeft,
                    finalLocation: [left, top],
                    startTime: now,
                    endTime: now + durationSeconds
                )
            }
        }
        else {
            DispatchQueue.main.async {
                self.setLocation([left, top])
            }
        }
    }
    
    func setColor(_ color: UInt32) {
        DispatchQueue.main.async {
            let r = Float((color >> 24) & 255) / 255;
            let g = Float((color >> 16) & 255) / 255;
            let b = Float((color >> 8) & 255) / 255;
            let a = Float((color >> 0) & 255) / 255;
            let float_color : SIMD4<Float> = [r, g, b, a];
            self.color = float_color
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
    
    func applyAnimations(time: Float64) {
        if let animation = self.locationAnimation {
            let (animationDone, location) = animation.getLocation(time: time)
            if animationDone {
                self.locationAnimation = nil
            }
            self.setLocation(location)
        }
        
        if let animation = self.textureAnimation {
            let (animationDone, texture) = animation.getTexture(time: time)
            if animationDone {
                self.textureAnimation = nil
            }
            self.texture = texture
        }
    }
    
    func render(encoder: MTLRenderCommandEncoder, uniformBufferIndex: Int, time: Float64) {
        let uniformBufferOffset = alignedUniformsSize * uniformBufferIndex
        
        applyAnimations(time: time)
        
        uniforms = UnsafeMutableRawPointer(uniformBuffer.contents() + uniformBufferOffset)
            .bindMemory(to:SpriteUniform.self, capacity:1)
        
        uniforms[0].topLeftMajor = topLeftMajor
        uniforms[0].topLeftMinor = topLeftMinor
        uniforms[0].color = self.color
        uniforms[0].hasTexture = texture != nil
        
        uniforms[0].size = [Float32(self.size.width), Float32(self.size.height)]
        texture?.fillSpriteUniformUvs(uniforms: uniforms)
        
        encoder.setVertexBuffer(uniformBuffer, offset: uniformBufferOffset, index: 0)
        if (texture != nil) {
            encoder.setFragmentTexture(texture!.wrapped, index: 0)
        }
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
