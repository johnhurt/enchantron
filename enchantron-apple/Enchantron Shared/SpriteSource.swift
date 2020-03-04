//
//  File.swift
//  Enchantron iOS
//
//  Created by Kevin Guthrie on 8/29/19.
//  Copyright © 2019 Rook And Pawn Industries, Inc. All rights reserved.
//

import SpriteKit

protocol SpriteSource {
    func createSprite() -> Sprite
    func createGroup() -> SpriteGroup
}

func createSpriteOn(parent : SKNode) -> Sprite {
    let sprite = Sprite()
    
    let onMain : () -> () = {
        sprite.zPosition = 0.0
        parent.addChild(sprite)
    }
    
    if Thread.isMainThread {
        onMain()
    }
    else {
        DispatchQueue.main.sync { onMain() }
    }
    
    return sprite
}

func createGroupOn(parent: SKNode) -> SpriteGroup {
    let group = SpriteGroup()
    
    let onMain : () -> () = {
        group.zPosition = 0.0
        parent.addChild(group)
    }
    
    if Thread.isMainThread {
        onMain()
    }
    else {
        DispatchQueue.main.sync { onMain() }
    }
    
    return group
}

