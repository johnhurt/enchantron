//
//  TextureLoader.swift
//  FourFours
//
//  Created by Kevin Guthrie on 8/16/18.
//  Copyright © 2018 Rook And Pawn Industries, Inc. All rights reserved.
//

import Foundation
import Metal
import MetalKit
import simd
import CoreImage

class ResourceLoader {

    static let textureLoaderOptions : [ MTKTextureLoader.Option : Any] = [
        MTKTextureLoader.Option.textureUsage: NSNumber(value: MTLTextureUsage.shaderRead.rawValue),
        MTKTextureLoader.Option.textureStorageMode: NSNumber(value: MTLStorageMode.`private`.rawValue)
    ]
    
    let loader: MTKTextureLoader
    
    init(loader: MTKTextureLoader) {
        self.loader = loader
    }
    
    func loadTexture(_ resourceName: String) -> Texture {
        
        let url = Bundle.main.url(forResource: resourceName, withExtension: nil)!
        
        let result = try! loader.newTexture(
            URL: url,
            options: ResourceLoader.textureLoaderOptions)
        
        return Texture(wrapped: result);
    }
    
    func loadTextureFromPngData(_ pngData: CGDataProvider) -> Texture {
        let image = CGImage(
            pngDataProviderSource: pngData,
            decode: nil,
            shouldInterpolate: false,
            intent: .defaultIntent)!
        
        return cgImageToTexture(image)
    }
    
    private func cgImageToTexture(_ image: CGImage) -> Texture {
        let rawTexture = try! loader.newTexture(
            cgImage: image,
            options: ResourceLoader.textureLoaderOptions)
        
        return Texture(wrapped: rawTexture)
    }
    
    func createAnimation() -> Animation {
        return Animation()
    }
    
    deinit {
        print("Dropping Texture Loader")
    }
}
