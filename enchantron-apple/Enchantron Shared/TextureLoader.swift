//
//  TextureLoader.swift
//  FourFours
//
//  Created by Kevin Guthrie on 8/16/18.
//  Copyright © 2018 Rook And Pawn Industries, Inc. All rights reserved.
//

import Foundation
import SpriteKit

class TextureLoader {
    
    func loadTexture(_ resourceName: String) -> Texture {
        return Texture(resourceName: resourceName)
    }
    
    func loadTextureFromPngData(_ pngData: CGDataProvider) -> Texture {
        return Texture(
            texture: SKTexture(
                cgImage: CGImage(
                    pngDataProviderSource: pngData,
                    decode: nil,
                    shouldInterpolate: false,
                    intent: .defaultIntent)!))
    }
    
    deinit {
        print("Dropping Texture Loader")
    }
}
