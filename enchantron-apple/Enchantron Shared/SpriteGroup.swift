//
//  SpriteGroup.swift
//  Enchantron
//
//  Created by Kevin Guthrie on 3/4/20.
//  Copyright © 2020 Rook And Pawn Industries, Inc. All rights reserved.
//

import Metal
import MetalKit
import simd

class SpriteGroup {
    
    private var sprites: [Sprite] = []
    private var groups: [SpriteGroup] = []
    private var visible = false
    private var zLevel : Double = 0.0
    
    private let device: MTLDevice
    
    private weak var parent : SpriteGroup?
    
    init(device: MTLDevice, parent: SpriteGroup?) {
        self.device = device
        self.parent = parent
    }
    
    func render(encoder: MTLRenderCommandEncoder, uniformBufferIndex: Int) {
        for s in sprites {
            s.render(encoder: encoder, uniformBufferIndex: uniformBufferIndex)
        }
        
        for sg in groups {
            sg.render(encoder: encoder, uniformBufferIndex: uniformBufferIndex)
        }
    }
    
    func setZLevel(_ zLevel: Float64) {
        DispatchQueue.main.async {
            self.zLevel = zLevel
        }
    }
    
    func setVisible(_ visible: Bool) {
        DispatchQueue.main.async {
            self.visible = visible
        }
    }
    
    func clear() {
        sprites.removeAll()
        groups.removeAll()
    }
    
    func createNewSprite() -> Sprite {
        return Sprite(
            device: self.device,
            container: self,
            texture: nil)
        
    }
    
    func addSprite(sprite: Sprite) {
        self.sprites.append(sprite)
    }
    
    func createNewGroup() -> SpriteGroup {
        return SpriteGroup(device: self.device, parent: self)
    }
    
    func addGroup(group: SpriteGroup ) {
        self.groups.append(group)
    }
    
    func removeChild(sprite: Sprite) {
        if let i = sprites.firstIndex(of: sprite) {
            sprites.remove(at: i)
        }
    }
    
    func removeChild(_ group: SpriteGroup) {
        if let i = groups.firstIndex(of: group) {
            groups.remove(at: i)
        }
    }
    
    func removeFromParent() {
        self.parent?.removeChild(self)
    }
}


extension SpriteGroup : SpriteSource {
    func createSprite() -> Sprite {
        return createSpriteOn(parent: self)
    }
    
    func createGroup() -> SpriteGroup {
        return createGroupOn(parent: self)
    }
}


extension SpriteGroup : Equatable {
    
    static func ==(lhs: SpriteGroup, rhs: SpriteGroup) -> Bool {
        return lhs === rhs
    }
}
