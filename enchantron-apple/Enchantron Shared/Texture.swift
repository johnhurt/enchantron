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
    let size: CGSize
    let roi: CGRect
    let uvTopLeft: SIMD2<Float32>
    let uvSize: SIMD2<Float32>
    
    
    init(wrapped: MTLTexture, roi: CGRect) {
        self.size = CGSize(width: wrapped.width, height: wrapped.height)
        self.wrapped = wrapped
        self.roi = roi
        uvTopLeft = [Float32(roi.origin.x / size.width), Float32(roi.origin.y / size.height)]
        uvSize = [Float32(roi.size.width / size.width), Float32(roi.size.height / size.height)]
    }
    
    convenience init(wrapped: MTLTexture) {
        self.init(
            wrapped: wrapped,
            roi: CGRect(x: 0, y: 0, width: wrapped.width, height: wrapped.height))
    }
    
    func getSubTexture(
        _ left: Float64,
        _ top: Float64,
        _ width: Float64,
        _ height: Float64) -> Texture {
        return Texture(
            wrapped: wrapped,
            roi: CGRect(
                x: self.roi.origin.x + CGFloat(left),
                y: self.roi.origin.y + CGFloat(top),
                width: CGFloat(width),
                height: CGFloat(height)))
    }
    
    func getWidth() -> Float64 {
        return Float64(self.roi.size.width)
    }
    
    func getHeight() -> Float64 {
        return Float64(self.roi.size.height)
    }
    
    func fillSpriteUniformUvs(uniforms: UnsafeMutablePointer<SpriteUniform>) {
        uniforms[0].textureUvTopLeft = uvTopLeft
        uniforms[0].textureUvSize = uvSize
    }
}
