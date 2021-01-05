//
//  Texture.swift
//  FourFours
//
//  Created by Kevin Guthrie on 8/16/18.
//  Copyright © 2018 Rook And Pawn Industries, Inc. All rights reserved.
//

import Foundation
import Metal
import MetalKit
import simd

class Texture {
    
    let wrapped: MTLTexture
    let size: SIMD2<Float64>
    let roiTopLeft: SIMD2<Float64>
    let roiSize: SIMD2<Float64>
    let uvTopLeft: SIMD2<Float32>
    let uvSize: SIMD2<Float32>
    
    
    init(wrapped: MTLTexture, roiTopLeft: SIMD2<Float64>, roiSize: SIMD2<Float64>) {
        self.size = [Float64(wrapped.width), Float64(wrapped.height)]
        self.wrapped = wrapped
        self.roiTopLeft = roiTopLeft
        self.roiSize = roiSize
        
        uvTopLeft = [
            Float32(roiTopLeft.x / size.x),
            Float32(roiTopLeft.y / size.y)]
        uvSize = [
            Float32(roiSize.x / size.x),
            Float32(roiSize.y / size.y)]
    }
    
    convenience init(wrapped: MTLTexture) {
        self.init(
            wrapped: wrapped,
            roiTopLeft: [0.0, 0.0],
            roiSize: [Float64(wrapped.width), Float64(wrapped.height)])
    }
    
    func getSubTexture(
        _ left: Float64,
        _ top: Float64,
        _ width: Float64,
        _ height: Float64) -> Texture {
        return Texture(
            wrapped: wrapped,
            roiTopLeft: [roiTopLeft.x + left, roiTopLeft.y + top],
            roiSize: [width, height])
    }
    
    func getWidth() -> Float64 {
        return Float64(self.roiSize.x)
    }
    
    func getHeight() -> Float64 {
        return Float64(self.roiSize.y)
    }
    
    func fillSpriteUniformUvs(uniforms: UnsafeMutablePointer<SpriteUniform>) {
        uniforms[0].textureUvTopLeft = uvTopLeft
        uniforms[0].textureUvSize = uvSize
    }
}
