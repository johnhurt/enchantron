//
//  Animation.swift
//  Enchantron
//
//  Created by Kevin Guthrie on 5/16/20.
//  Copyright © 2020 Rook And Pawn Industries, Inc. All rights reserved.
//

import Foundation
import Metal
import MetalKit
import simd

class Animation {
    
    var frames: [Texture] = []
    var isLoop = false
    var name = "Un-named animation"
    
    func addTexture(_ texture: Texture) {
        frames.append(texture)
    }
    
    func setIsLoop(_ isLoop: Bool) {
        self.isLoop = isLoop
    }
    
    func setName(_ name: String) {
        self.name = name
    }
}
