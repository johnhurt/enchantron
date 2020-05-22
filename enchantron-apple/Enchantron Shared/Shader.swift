//
//  Shader.swift
//  Enchantron
//
//  Created by Kevin Guthrie on 5/21/20.
//  Copyright © 2020 Rook And Pawn Industries, Inc. All rights reserved.
//

import Foundation
import SpriteKit

class Shader {
    
    let inner: SKShader
    
    init(_ fileName: String) {
        self.inner = SKShader(fileNamed: fileName)
    }
    
}
