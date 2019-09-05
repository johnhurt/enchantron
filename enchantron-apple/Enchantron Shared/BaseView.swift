//
//  BaseView.swift
//  FourFours
//
//  Created by Kevin Guthrie on 8/17/18.
//  Copyright © 2018 Rook And Pawn Industries, Inc. All rights reserved.
//

import Foundation
import SpriteKit

class BaseView: SKNode {
    
    private var presenter : AnyObject?
    private var ctx : ApplicationContext?
    private var transitionService : TransitionService?
    private var viewport : Viewport?
    
    private var size: CGSize?
    
    override init() {
        super.init()
    }
    
    required init?(coder aDecoder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }
    
    func getContext() -> ApplicationContext {
        return ctx!
    }
    
    func transitionTo<T : BaseView>(newView: T, presenterBinder : @escaping (T) -> AnyObject ) {
        newView.initializeCtx(ctx: self.ctx!, transitionService: self.transitionService!)
        transitionService?.transition(view: newView, presenterBinder: presenterBinder)
        self.unsetPresenter()
    }
    
    func initializeCtx(ctx : ApplicationContext,
                       transitionService : TransitionService) {
        self.ctx = ctx
        self.transitionService = transitionService
    }
    
    func setPresenter(presenter: AnyObject) {
        self.presenter = presenter
    }
    
    func setViewport(viewport: Viewport) {
        self.viewport = viewport
    }
    
    func getViewport() -> Viewport {
        return self.viewport!
    }
    
    func unsetPresenter() {
        self.presenter = nil
    }
    
    final func setSize(size: CGSize) {
        self.size = size
        layout(size: size)
    }
    
    func layout(size: CGSize) {}
    
}
