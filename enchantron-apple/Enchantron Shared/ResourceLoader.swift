//
//  TextureLoader.swift
//  FourFours
//
//  Created by Kevin Guthrie on 8/16/18.
//  Copyright © 2018 Rook And Pawn Industries, Inc. All rights reserved.
//

import Foundation
import SpriteKit

class ResourceLoader {
    
    func loadTexture(_ resourceName: String) -> Texture {
        return Texture(resourceName: resourceName)
    }
    
    func loadTextureFromPngData(_ pngData: CGDataProvider) -> Texture {
        let image = CGImage(
            pngDataProviderSource: pngData,
            decode: nil,
            shouldInterpolate: false,
            intent: .defaultIntent)
        
        let result = Texture(
            texture: SKTexture(
                cgImage: image!))
        
        print("loaded texture")
        
        return result
    }
    
    func loadShader(_ shaderName: String) -> Shader {
        return Shader(shaderName)
    }
    
    func createAnimation() -> Animation {
        return Animation()
    }
    
    deinit {
        print("Dropping Texture Loader")
    }
}
