//
//  Animations.swift
//  Enchantron
//
//  Created by Kevin Guthrie on 1/2/21.
//  Copyright © 2021 Rook And Pawn Industries, Inc. All rights reserved.
//

import Foundation
import simd
import CoreImage

struct TextureAnimation {
    let startTime: Float64
    let animation: Animation
    let secsPerFrame: Float64
    let secsPerCycle: Float64
    let frameCount: Float64
    let loop: Bool
    
    init(startTime: Float64,
         animation: Animation,
         secsPerFrame: Float64)
    {
        self.startTime = startTime
        self.animation = animation
        self.secsPerFrame = secsPerFrame
        self.frameCount = Float64(animation.frames.count)
        self.secsPerCycle = secsPerFrame * frameCount
        self.loop = animation.isLoop
    }
    
    func getTexture(time: Float64) -> (Bool, Texture) {
        let (cycleNumber, scaledDelta) = modf((time - startTime) / secsPerCycle)
        
        if !loop && cycleNumber >= 1.0 {
            return (true, animation.frames.last!)
        }
        
        let frame = Int(floor(scaledDelta * frameCount))
        
        return (false, animation.frames[frame])
    }
}

struct LocationAnimation {
    let startLocation: SIMD2<Float64>
    let vector: SIMD2<Float64>
    let startTime: Float64
    let endTime: Float64
    
    init(
        startLocation: SIMD2<Float64>,
        finalLocation: SIMD2<Float64>,
        startTime: Float64,
        endTime: Float64)
    {
        self.startLocation = startLocation
        self.vector = finalLocation - startLocation
        self.startTime = startTime
        self.endTime = endTime
    }
    
    func getLocation(time: Float64) -> (Bool, SIMD2<Float64>) {
        if time > self.endTime {
            return (true, startLocation + vector)
        }
        
        return (false, vector * ((time - startTime) / (endTime - startTime)) + startLocation)
    }
}

struct SizeAnimation {
    let startSize: SIMD2<Float64>
    let finalSize: SIMD2<Float64>
    let startTime: Float64
    let endTime: Float64
}
