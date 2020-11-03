//
//  Sprite.swift
//  FourFours
//
//  Created by Kevin Guthrie on 8/23/18.
//  Copyright © 2018 Rook And Pawn Industries, Inc. All rights reserved.
//

import Foundation
import SpriteKit

class Sprite : SKSpriteNode {
    
    var currentTexture: Texture?
    var eventSink: Sprite?
    
    init() {
        super.init(
            texture: nil,
            color: SKColor.clear,
            size: CGSize(width: 0, height: 0))
        self.isHidden = true
    }
    
    required init?(coder aDecoder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }
    
    func animate(_ animation: Animation, _ secsPerFrame: Float64) {
        
        var a = SKAction.animate(with: animation.frames, timePerFrame: secsPerFrame)
        
        if animation.isLoop {
            a = SKAction.repeatForever(a)
        }
        
        self.run(a, withKey: animation.name)
    }
    
    func clearAnimations() {
        self.removeAllActions()
    }
    
    func setShader(_ shader: Shader) {
        DispatchQueue.main.async {
            self.shader = shader.inner
        }
        
    }
    
    func setShaderVariableF64(_ name: String, _ value: Float64) {
        DispatchQueue.main.async {
            self.setValue(SKAttributeValue(float: Float(value)), forAttribute: name)
        }
    }
    
    
    func setShaderVariableVec2F64(_ name: String, _ v0: Float64, _ v1: Float64) {
        DispatchQueue.main.async {
            self.setValue(SKAttributeValue(vectorFloat2: simd_float2(Float(v0), Float(v1))), forAttribute: name)
        }
    }
    
    
    func setShaderVariableVec3F64(_ name: String, _ v0: Float64, _ v1: Float64, _ v2: Float64) {
        DispatchQueue.main.async {
            self.setValue(SKAttributeValue(vectorFloat3: simd_float3(Float(v0), Float(v1), Float(v2))), forAttribute: name)
        }
    }
    
    
    func setShaderVariableVec4F64(_ name: String, _ v0: Float64, _ v1: Float64, _ v2: Float64, _ v3: Float64) {
        DispatchQueue.main.async {
            self.setValue(SKAttributeValue(vectorFloat4: simd_float4(Float(v0), Float(v1), Float(v2), Float(v3))), forAttribute: name)
        }
    }
    
    func clearShader() {
        self.shader = nil
    }
    
    func setVisible(_ visible: Bool) {
        DispatchQueue.main.async {
            self.isHidden = !visible
        }
    }
    
    func setTexture(_ texture: Texture) {
        DispatchQueue.main.async {
            self.removeAllActions();
            self.currentTexture = texture
            self.texture = texture.texture
            self.anchorPoint = texture.anchorPoint
        }
    }
    
    func setZLevel(_ newZLevel: Double) {
        self.zPosition = CGFloat(newZLevel)
    }
    
    func setSizeAnimated(_ width: Float64, _ height: Float64, _ durationSeconds: Float64) {
        
        
        
        if durationSeconds > 0.0 {
            let resize = SKAction.resize(
                toWidth: CGFloat(width),
                height: CGFloat(height),
                duration: durationSeconds)
            run(resize)
        }
        else {
            self.size = CGSize(width: width, height: height)
        }
        
    }
    
    func setLocationAnimated(_ left: Float64, _ top: Float64, _ durationSeconds: Float64) {
        
        if durationSeconds > 0.0 {
            
            let move = SKAction.move(
                to: CGPoint(x: CGFloat(left), y: -CGFloat(top)),
                duration: durationSeconds)
            
            
            run(move)
        }
        else {
            self.position = CGPoint(x: left, y: -top)
        }
    }
    
    func propagateEventsTo(_ sprite: Sprite) {
        DispatchQueue.main.sync {
            self.isUserInteractionEnabled = true
            self.eventSink = sprite
        }
    }
    
    override func removeFromParent() {
        DispatchQueue.main.asyncAfter(deadline: DispatchTime.now() + 1, execute: {
            super.removeFromParent()
        })
    }
    
    deinit {
        print("Dropping Sprite")
    }
}
