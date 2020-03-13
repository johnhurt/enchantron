//
//  BaseView.swift
//  FourFours
//
//  Created by Kevin Guthrie on 8/17/18.
//  Copyright © 2018 Rook And Pawn Industries, Inc. All rights reserved.
//


import SpriteKit

class BaseView: SKNode {
    
    private var dragHandlers: [MultiDragHandler] = []
    private var layoutHandlers: [LayoutHandler] = []
    private var magnifyHandlers: [MagnifyHandler] = []
    
    private var presenter : BoxedAny?
    private var ctx : ApplicationContext?
    private var transitionService : TransitionService?
    private var viewport : Viewport?
    
    #if os(iOS) || os(tvOS)
    private var touchTracker = TouchLookup()
    #endif
    
    var size: CGSize?
    
    override init() {
        super.init()
    }
    
    required init?(coder aDecoder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }
    
    func getContext() -> ApplicationContext {
        return ctx!
    }
    
    func transitionTo<T : BaseView>(newView: T, binder : @escaping (T) -> ()) {
        newView.initializeCtx(ctx: self.ctx!, transitionService: self.transitionService!)
        binder(newView)
        self.unsetPresenter()
    }
    
    func initializeCtx(ctx : ApplicationContext,
                       transitionService : TransitionService) {
        self.ctx = ctx
        self.transitionService = transitionService
    }
    
    func initializePreBind() {
        self.transitionService?.preBindTransition(self)
    }
    
    func initializePostBind(_ presenter: BoxedAny) {
        self.presenter = presenter
        self.transitionService?.postBindTransition(self)
        
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
    
    func getDragHandlers() -> [MultiDragHandler] {
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
    
    func addMultiDragHandler(_ handler: MultiDragHandler) -> HandlerRegistration {
        DispatchQueue.main.sync {
            self.isUserInteractionEnabled = true
            self.dragHandlers.append(handler)
        }
        return HandlerRegistration(deregister_callback: {
            self.removeHandler(handler)
        })
    }
    
    func removeHandler(_ handler: MultiDragHandler) {
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
    
    final func layout(size: CGSize) {
        layoutHandlers.forEach { (handler) in
            handler.onLayout(width: Int64(size.width), height: Int64(size.height))
        }
        
        localLayout(size: size)
    }
    
    func localLayout(size: CGSize) {
        
    }
    
    func magnify(scaleChangeAdditive: CGFloat, centerPoint: CGPoint) {
        magnifyHandlers.forEach { (handler) in
            handler.onMagnify(
                scaleChangeAdditive: Float64(scaleChangeAdditive),
                zoomCenterX: Float64(centerPoint.x),
                zoomCenterY: Float64(self.size!.height - centerPoint.y))
        }
    }
    
    func oneDragStarted(id: Int64, globalPoint: CGPoint, localPoint: CGPoint) {
        for dragHandler in self.dragHandlers {
            dragHandler.onOneDragStart(
                dragId: id,
                globalX: Float64(globalPoint.x),
                globalY: Float64(globalPoint.y),
                localX: Float64(localPoint.x),
                localY: -Float64(localPoint.y))
        }
    }
    
    func oneDragMoved(id: Int64, globalPoint: CGPoint, localPoint: CGPoint) {
        for dragHandler in self.dragHandlers {
            dragHandler.onOneDragMove(
                dragId: id,
                globalX: Float64(globalPoint.x),
                globalY: Float64(globalPoint.y),
                localX: Float64(localPoint.x),
                localY: -Float64(localPoint.y))
        }
    }
    
    func oneDragEnded(id: Int64, globalPoint: CGPoint, localPoint: CGPoint) {
        for dragHandler in self.dragHandlers {
            dragHandler.onOneDragEnd(
                dragId: id,
                globalX: Float64(globalPoint.x),
                globalY: Float64(globalPoint.y),
                localX: Float64(localPoint.x),
                localY: -Float64(localPoint.y))
        }
    }
    
    func twoDragsStarted(id1: Int64, globalPoint1: CGPoint, localPoint1: CGPoint,
                        id2: Int64, globalPoint2: CGPoint, localPoint2: CGPoint) {
        for dragHandler in self.dragHandlers {
            dragHandler.onTwoDragsStart(
                dragId1: id1,
                globalX1: Float64(globalPoint1.x),
                globalY1: Float64(globalPoint1.y),
                localX1: Float64(localPoint1.x),
                localY1: -Float64(localPoint1.y),
                dragId2: id2,
                globalX2: Float64(globalPoint2.x),
                globalY2: Float64(globalPoint2.y),
                localX2: Float64(localPoint2.x),
                localY2: -Float64(localPoint2.y))
        }
    }
    
    func twoDragsMoved(id1: Int64, globalPoint1: CGPoint, localPoint1: CGPoint,
                      id2: Int64, globalPoint2: CGPoint, localPoint2: CGPoint) {
        for dragHandler in self.dragHandlers {
            dragHandler.onTwoDragsMove(
                dragId1: id1,
                globalX1: Float64(globalPoint1.x),
                globalY1: Float64(globalPoint1.y),
                localX1: Float64(localPoint1.x),
                localY1: -Float64(localPoint1.y),
                dragId2: id2,
                globalX2: Float64(globalPoint2.x),
                globalY2: Float64(globalPoint2.y),
                localX2: Float64(localPoint2.x),
                localY2: -Float64(localPoint2.y))
        }
    }
    
    func twoDragsEnded(id1: Int64, globalPoint1: CGPoint, localPoint1: CGPoint,
                      id2: Int64, globalPoint2: CGPoint, localPoint2: CGPoint) {
        for dragHandler in self.dragHandlers {
            dragHandler.onTwoDragsEnd(
                dragId1: id1,
                globalX1: Float64(globalPoint1.x),
                globalY1: Float64(globalPoint1.y),
                localX1: Float64(localPoint1.x),
                localY1: -Float64(localPoint1.y),
                dragId2: id2,
                globalX2: Float64(globalPoint2.x),
                globalY2: Float64(globalPoint2.y),
                localX2: Float64(localPoint2.x),
                localY2: -Float64(localPoint2.y))
        }
    }
    
    #if os(iOS) || os(tvOS)
    
    override func touchesBegan(_ touches: Set<UITouch>, with event: UIEvent?) {
        self.dragsStart(touches:
            self.touchTracker.filterForNewActiveTouches(newTouches: touches))
    }
    
    override func touchesMoved(_ touches: Set<UITouch>, with event: UIEvent?) {
        self.dragsMoved(touches:
            self.touchTracker.filterForMovedActiveTouches(movedTouches: touches))
    }
    
    override func touchesCancelled(_ touches: Set<UITouch>, with event: UIEvent?) {
        self.dragsEnded(touches:
            self.touchTracker.filterForEndedActiveTouches(endedTouches: touches))
    }
    
    override func touchesEnded(_ touches: Set<UITouch>, with event: UIEvent?) {
        self.touchesCancelled(touches, with: event)
    }
    
    func dragsStart(touches: [(Int64, UITouch)]) {
        switch touches.count {
        case 1:
            let (id, touch) = touches[0]
            let globalPoint = touch.location(in: nil)
            let localPoint = touch.location(in: self)
            oneDragStarted(id: id, globalPoint: globalPoint, localPoint: localPoint)
        
        case 2:
            let (id1, touch1) = touches[0]
            let (id2, touch2) = touches[1]
            
            let globalPoint1 = touch1.location(in: nil)
            let localPoint1 = touch1.location(in: self)
            let globalPoint2 = touch2.location(in: nil)
            let localPoint2 = touch2.location(in: self)
            
            twoDragsStarted(
                id1: id1,
                globalPoint1: globalPoint1,
                localPoint1: localPoint1,
                id2: id2,
                globalPoint2: globalPoint2,
                localPoint2: localPoint2)
        default: do {}
        }
    }
    
    func dragsMoved(touches: [(Int64, UITouch)]) {
        switch touches.count {
        case 1:
            let (id, touch) = touches[0]
            let globalPoint = touch.location(in: nil)
            let localPoint = touch.location(in: self)
            oneDragMoved(id: id, globalPoint: globalPoint, localPoint: localPoint)
        
        case 2:
            let (id1, touch1) = touches[0]
            let (id2, touch2) = touches[1]
            
            let globalPoint1 = touch1.location(in: nil)
            let localPoint1 = touch1.location(in: self)
            let globalPoint2 = touch2.location(in: nil)
            let localPoint2 = touch2.location(in: self)
            
            twoDragsMoved(
                id1: id1,
                globalPoint1: globalPoint1,
                localPoint1: localPoint1,
                id2: id2,
                globalPoint2: globalPoint2,
                localPoint2: localPoint2)
        default: do {}
        }
    }
    
    func dragsEnded(touches: [(Int64, UITouch)]) {
        switch touches.count {
        case 1:
            let (id, touch) = touches[0]
            let globalPoint = touch.location(in: nil)
            let localPoint = touch.location(in: self)
            oneDragEnded(id: id, globalPoint: globalPoint, localPoint: localPoint)
        
        case 2:
            let (id1, touch1) = touches[0]
            let (id2, touch2) = touches[1]
            
            let globalPoint1 = touch1.location(in: nil)
            let localPoint1 = touch1.location(in: self)
            let globalPoint2 = touch2.location(in: nil)
            let localPoint2 = touch2.location(in: self)
            
            twoDragsEnded(
                id1: id1,
                globalPoint1: globalPoint1,
                localPoint1: localPoint1,
                id2: id2,
                globalPoint2: globalPoint2,
                localPoint2: localPoint2)
        default: do {}
        }
    }
    
    #endif
    
    #if os(OSX)
    
    override func mouseDown(with event: NSEvent) {
        DispatchQueue.main.async {
            let localPoint = event.location(in: self)
            
            self.dragHandlers.forEach { (handler) in
                handler.onOneDragStart(
                    dragId: 0,
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
                handler.onOneDragMove(
                    dragId: 0,
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
                handler.onOneDragEnd(
                    dragId: 0,
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
