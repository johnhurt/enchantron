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
    private var magnifyHandlers: [MagnifyHandler] = []
    
    private var presenter : AnyObject?
    private var ctx : ApplicationContext?
    private var transitionService : TransitionService?
    private var viewport : Viewport?
    
    
    #if os(iOS) || os(tvOS)
    private var touchTracker : TouchTracker?
    #endif
    
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
        #if os(iOS) || os(tvOS)
        self.touchTracker = TouchTracker(self)
        #endif
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
    
    func addMagnifyHandler(_ handler: MagnifyHandler) -> HandlerRegistration {
        DispatchQueue.main.sync {
            self.magnifyHandlers.append(handler)
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
    
    func removeHandler(_ handler: MagnifyHandler) {
        DispatchQueue.main.sync {
            if let index = self.magnifyHandlers.firstIndex(of: handler) {
                self.magnifyHandlers.remove(at: index)
            }
        }
    }
    
    func layout(size: CGSize) {
        layoutHandlers.forEach { (handler) in
            handler.onLayout(width: Int64(size.width), height: Int64(size.height))
        }
    }
    
    func magnify(scaleChangeAdditive: CGFloat, centerPoint: CGPoint) {
        magnifyHandlers.forEach { (handler) in
            handler.onMagnify(
                scaleChangeAdditive: Float64(scaleChangeAdditive),
                zoomCenterX: Float64(centerPoint.x),
                zoomCenterY: Float64(centerPoint.y))
        }
    }
    
    func dragStart(windowPoint: CGPoint, localPoint: CGPoint) {
        self.dragHandlers.forEach { (handler) in
            handler.onDragStart(
                globalX: Float64(windowPoint.x),
                globalY: Float64(windowPoint.y),
                localX: Float64(localPoint.x),
                localY: -Float64(localPoint.y))
        }
    }
    
    func dragMoved(windowPoint: CGPoint, localPoint: CGPoint) {
        self.dragHandlers.forEach { (handler) in
            handler.onDragMove(
                globalX: Float64(windowPoint.x),
                globalY: Float64(windowPoint.y),
                localX: Float64(localPoint.x),
                localY: -Float64(localPoint.y))
        }
    }
    
    func dragEnded(windowPoint: CGPoint, localPoint: CGPoint) {
        self.dragHandlers.forEach { (handler) in
            handler.onDragEnd(
                globalX: Float64(windowPoint.x),
                globalY: Float64(windowPoint.y),
                localX: Float64(localPoint.x),
                localY: -Float64(localPoint.y))
        }
    }
    
    #if os(iOS) || os(tvOS)
    
    override func touchesBegan(_ touches: Set<UITouch>, with event: UIEvent?) {
        self.touchTracker?.touchesStarted(touches)
    }
    
    override func touchesMoved(_ touches: Set<UITouch>, with event: UIEvent?) {
        self.touchTracker?.touchesMoved(touches)
    }
    
    override func touchesCancelled(_ touches: Set<UITouch>, with event: UIEvent?) {
        touchesEnded(touches, with: event)
    }
    
    override func touchesEnded(_ touches: Set<UITouch>, with event: UIEvent?) {
        touchTracker?.touchesEnded(touches)
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
