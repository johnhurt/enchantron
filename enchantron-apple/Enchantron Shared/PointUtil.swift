//
//  PointUtil.swift
//  Enchantron
//
//  Created by Kevin Guthrie on 12/11/20.
//  Copyright © 2020 Rook And Pawn Industries, Inc. All rights reserved.
//

class PointUtil {
    
    static let MAJOR_SIZE : Float64 = 512
    
    /// method for doing euler div that for some reason is branchless :P
    class func eulerDivF(num: Float64, denom: Float64) -> (Float32, Float32) {
        
        let (whole, frac) = modf(num / denom)
        
        let s = sign(frac)
        let s2 = s * s
        let sp = (s - s2) / 2
        
        let resultWhole = (whole + sp) * denom
        
        return (Float32(resultWhole / denom), Float32(num - resultWhole))
        
    }
    
    /// Convert the given double-precision coordinate in to a pair of major-minor single-precision coordinates
    class func toMajorMinor(x: Float64, y: Float64) -> (SIMD2<Float32>, SIMD2<Float32>){
        let (xMajor, xMinor) = eulerDivF(num: x, denom: MAJOR_SIZE)
        let (yMajor, yMinor) = eulerDivF(num: y, denom: MAJOR_SIZE)
        
        return ([xMajor, yMajor], [xMinor, yMinor])
    }
}

