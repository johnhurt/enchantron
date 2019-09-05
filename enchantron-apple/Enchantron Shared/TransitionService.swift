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
    
    func transition<T: BaseView>(view: T, presenterBinder: @escaping PresenterBinder<T>) {
        let transitionOp = {
            self.preBindTransition(view)
            view.setPresenter(presenter: presenterBinder(view))
            self.postBindTransition(view)
        }
        
        if Thread.isMainThread {
            transitionOp()
        }
        else {
            DispatchQueue.main.sync { self.preBindTransition(view) }
            view.setPresenter(presenter: presenterBinder(view))
            DispatchQueue.main.sync { self.postBindTransition(view) }
        }
        
    }
    
    deinit{
        print("Dropping Transition Service")
    }
}
