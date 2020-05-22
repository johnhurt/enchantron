//
//  Animation.swift
//  Enchantron
//
//  Created by Kevin Guthrie on 5/16/20.
//  Copyright © 2020 Rook And Pawn Industries, Inc. All rights reserved.
//

import Foundation
import SpriteKit

class Animation {
    
    var frames: [SKTexture] = []
    var isLoop = false
    var name = "Un-named animation"
    
    func addTexture(_ texture: Texture) {
        frames.append(texture.texture)
    }
    
    func setIsLoop(_ isLoop: Bool) {
        self.isLoop = isLoop
    }
    
    func setName(_ name: String) {
        self.name = name
    }
}
