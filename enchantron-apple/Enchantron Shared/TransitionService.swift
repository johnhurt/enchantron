//
//  TransitionService.swift
//  Enchantron
//
//  Created by Kevin Guthrie on 7/25/18.
//  Copyright © 2018 Rook And Pawn Industries, Inc. All rights reserved.
//

import Foundation
import Metal
import MetalKit
import simd

class TransitionService {
    
    typealias TransitionClosure = (NativeView) -> NativeView
    
    var transiation : TransitionClosure?
    
    func setTransitioner(transiation: @escaping TransitionClosure ) {
        self.transiation = transiation
    }
    
    func transitionTo(_ view: NativeView, _ dropCurrent: Bool) {
        let transitionOp = {
            let oldView = self.transiation!(view)
            
            if dropCurrent {
                oldView.unsetPresenter()
            }
        }
        
        if Thread.isMainThread {
            transitionOp()
        }
        else {
            DispatchQueue.main.sync { transitionOp() }
        }
    }
    
    deinit{
        print("Dropping Transition Service")
    }
}
