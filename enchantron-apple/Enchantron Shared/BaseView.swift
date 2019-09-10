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
    
    private var dragHandlers: [DragHandler] = []
    private var layoutHandlers: [LayoutHandler] = []
    
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
    
    func getDragHandlers() -> [DragHandler] {
        return self.dragHandlers
    }
    
    func addLayoutHandler(_ handler: LayoutHandler) -> HandlerRegistration {
        DispatchQueue.main.sync {
            self.layoutHandlers.append(handler)
        }
        
        return HandlerRegistration(deregister_callback: {
            self.removeHandler(handler)
        })
        
    }
    
    func removeHandler(_ handler: LayoutHandler) {
        DispatchQueue.main.sync {
            if let index = self.layoutHandlers.firstIndex(of: handler) {
                self.layoutHandlers.remove(at: index)
            }
        }
    }
    
    func addDragHandler(_ handler: DragHandler) -> HandlerRegistration {
        DispatchQueue.main.sync {
            self.isUserInteractionEnabled = true
            self.dragHandlers.append(handler)
        }
        return HandlerRegistration(deregister_callback: {
            self.removeHandler(handler)
        })
    }
    
    func removeHandler(_ handler: DragHandler) {
        DispatchQueue.main.sync {
            if let index = self.dragHandlers.firstIndex(of: handler) {
                self.dragHandlers.remove(at: index)
            }
        }
    }
    
    func layout(size: CGSize) {
        layoutHandlers.forEach { (handler) in
            handler.onLayout(width: Int64(size.width), height: Int64(size.height))
        }
    }
    
}


extension BaseView {
    
    #if os(iOS) || os(tvOS)
    override func touchesBegan(_ touches: Set<UITouch>, with event: UIEvent?) {
        
        DispatchQueue.main.async {
            
            let firstTouch = touches.first!
            
            let localPoint = firstTouch.location(in: self)
            let windowPoint = firstTouch.location(in: nil)
            
            self.dragHandlers.forEach { (handler) in
                handler.onDragStart(
                    globalX: Float64(windowPoint.x),
                    globalY: Float64(windowPoint.y),
                    localX: Float64(localPoint.x),
                    localY: -Float64(localPoint.y))
            }
        }
    }
    
    override func touchesMoved(_ touches: Set<UITouch>, with event: UIEvent?) {
        
        DispatchQueue.main.async {
            
            let firstTouch = touches.first!
            
            let localPoint = firstTouch.location(in: self)
            let windowPoint = firstTouch.location(in: nil)
            
            self.dragHandlers.forEach { (handler) in
                handler.onDragMove(
                    globalX: Float64(windowPoint.x),
                    globalY: Float64(windowPoint.y),
                    localX: Float64(localPoint.x),
                    localY: -Float64(localPoint.y))
            }
        }
    }
    
    override func touchesEnded(_ touches: Set<UITouch>, with event: UIEvent?) {
        
        DispatchQueue.main.async {
            
            let firstTouch = touches.first!
            
            let localPoint = firstTouch.location(in: self)
            let windowPoint = firstTouch.location(in: nil)
            
            self.dragHandlers.forEach { (handler) in
                handler.onDragEnd(
                    globalX: Float64(windowPoint.x),
                    globalY: Float64(windowPoint.y),
                    localX: Float64(localPoint.x),
                    localY: -Float64(localPoint.y))
            }
        }
    }
    
    #endif
    
    #if os(OSX)
    
    override func mouseDown(with event: NSEvent) {
        DispatchQueue.main.async {
            let localPoint = event.location(in: self)
            
            self.dragHandlers.forEach { (handler) in
                handler.onDragStart(
                    globalX: Float64(event.locationInWindow.x),
                    globalY: Float64((event.window?.contentView?.bounds.size.height)!
                        - event.locationInWindow.y),
                    localX: Float64(localPoint.x),
                    localY: -Float64(localPoint.y))
            }
        }
    }
    
    override func mouseDragged(with event: NSEvent) {
        DispatchQueue.main.async {
            let localPoint = event.location(in: self)
            self.dragHandlers.forEach { (handler) in
                handler.onDragMove(
                    globalX: Float64(event.locationInWindow.x),
                    globalY: Float64((event.window?.contentView?.bounds.size.height)!
                        - event.locationInWindow.y),
                    localX: Float64(localPoint.x),
                    localY: -Float64(localPoint.y))
            }
        }
    }
    
    override func mouseUp(with event: NSEvent) {
        DispatchQueue.main.async {
            let localPoint = event.location(in: self)
            self.dragHandlers.forEach { (handler) in
                handler.onDragEnd(
                    globalX: Float64(event.locationInWindow.x),
                    globalY: Float64((event.window?.contentView?.bounds.size.height)!
                        - event.locationInWindow.y),
                    localX: Float64(localPoint.x),
                    localY: -Float64(localPoint.y))
            }
        }
    }
    #endif
}
