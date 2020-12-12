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

func createSpriteOn(parent : SpriteGroup) -> Sprite {
    let sprite = parent.createNewSprite()
    
    let onMain : () -> () = {
        parent.addSprite(sprite: sprite)
    }
    
    if Thread.isMainThread {
        onMain()
    }
    else {
        DispatchQueue.main.sync { onMain() }
    }
    
    return sprite
}

func createGroupOn(parent: SpriteGroup) -> SpriteGroup {
    let group = parent.createNewGroup()
    
    let onMain : () -> () = {
        parent.addGroup(group: group)
    }
    
    if Thread.isMainThread {
        onMain()
    }
    else {
        DispatchQueue.main.sync { onMain() }
    }
    
    return group
}

