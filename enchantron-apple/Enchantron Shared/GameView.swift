//
//  GameView.swift
//  FourFours
//
//  Created by Kevin Guthrie on 8/21/18.
//  Copyright © 2018 Rook And Pawn Industries, Inc. All rights reserved.
//

import Foundation
import Metal
import MetalKit
import simd

class GameView : BaseView {
    
    override init(viewport: Viewport, device: MTLDevice) {
        super.init(viewport: viewport, device: device)
    }
    
    deinit {
        print("Dropping GameView")
    }
}


