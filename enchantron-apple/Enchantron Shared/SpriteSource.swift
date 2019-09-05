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
}

func createSpriteOn(parent : SKNode) -> Sprite {
    let sprite = Sprite()
    
    let onMain : () -> () = {
        sprite.zPosition = CGFloat(GameView.z)
        GameView.z += 1
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

