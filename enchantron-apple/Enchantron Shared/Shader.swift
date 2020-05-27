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
 
    func addShaderVariable(_ name: String, _ varType: String) {
        switch varType {
        case "FLOAT": self.inner.attributes.append(SKAttribute(name: name, type: SKAttributeType.float))
        case "VEC4_FLOAT": self.inner.attributes.append(SKAttribute(name: name, type: SKAttributeType.vectorFloat4))
        default: return
        }
        
    }

    func addShaderConstantF64(_ name: String, _ value: Float64) {
        self.inner.addUniform(SKUniform(name: name, float: Float(value)))
    }
    
    func addShaderConstantVec4F64(_ name: String, _ v0: Float64, _ v1: Float64, _ v2: Float64, _ v3: Float64) {
        self.inner.addUniform(SKUniform(name: name, vectorFloat4: simd_float4(Float(v0), Float(v1), Float(v2), Float(v3))))
    }
}
