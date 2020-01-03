//
//  TransitionService.swift
//  Enchantron
//
//  Created by Kevin Guthrie on 7/25/18.
//  Copyright © 2018 Rook And Pawn Industries, Inc. All rights reserved.
//

import Foundation
import SpriteKit

class TransitionService {
    
    typealias PresenterBinder<T : BaseView> = (T) -> AnyObject
    typealias TransitionClosure = (BaseView) -> Void
    
    let preBindTransition : TransitionClosure
    let postBindTransition : TransitionClosure
    
    init(preBindTransition: @escaping TransitionClosure
        , postBindTransition: @escaping TransitionClosure ) {
        self.preBindTransition = preBindTransition
        self.postBindTransition = postBindTransition
    }
    
    func preBindTransition<T: BaseView>(view: T) {
        let transitionOp = {
            self.preBindTransition(view)
        }
        
        if Thread.isMainThread {
            transitionOp()
        }
        else {
            DispatchQueue.main.sync { transitionOp() }
        }
    }
    
    func postBindTransition<T: BaseView>(view: T) {
        let transitionOp = {
            self.postBindTransition(view)
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
