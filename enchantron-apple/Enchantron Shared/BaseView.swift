//
//  BaseView.swift
//  FourFours
//
//  Created by Kevin Guthrie on 8/17/18.
//  Copyright © 2018 Rook And Pawn Industries, Inc. All rights reserved.
//

import Metal
import MetalKit
import simd

class BaseView : SpriteSource {
    
    private var dragHandlers: [MultiTouchHandler] = []
    private var layoutHandlers: [LayoutHandler] = []
    private var magnifyHandlers: [MagnifyHandler] = []
    
    private var presenter : BoxedAny?
    private var ctx : ApplicationContext?
    private var transitionService : TransitionService?
    
    let viewport : Viewport
    let device: MTLDevice
    let rootGroup : SpriteGroup
    
    init(viewport: Viewport, device: MTLDevice) {
        self.viewport = viewport
        self.device = device
        self.rootGroup = SpriteGroup(device: device, parent: nil)
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
    
    func unsetPresenter() {
        self.presenter = nil
    }
    
    func getDragHandlers() -> [MultiTouchHandler] {
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
    
    func addMultiTouchHandler(_ handler: MultiTouchHandler) -> HandlerRegistration {
        DispatchQueue.main.sync {
            self.dragHandlers.append(handler)
        }
        return HandlerRegistration(deregister_callback: {
            self.removeHandler(handler)
        })
    }
    
    func removeHandler(_ handler: MultiTouchHandler) {
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
    
    func getViewport() -> Viewport {
        return viewport
    }
    
    func magnify(scaleChangeAdditive: Float64, centerPoint: SIMD2<Float64>) {
        magnifyHandlers.forEach { (handler) in
            handler.onMagnify(
                scaleChangeAdditive: Float64(scaleChangeAdditive),
                zoomCenterX: centerPoint.x,
                zoomCenterY:centerPoint.y)
        }
    }
    
    func oneDragStarted(id: Int64, globalPoint: SIMD2<Float64>, clickCount: Int64) {
        for dragHandler in self.dragHandlers {
            dragHandler.onOneDragStart(
                dragId: id,
                globalX: globalPoint.x,
                globalY: globalPoint.y,
                clickCount: clickCount)
        }
    }
    
    func oneDragMoved(id: Int64, globalPoint: SIMD2<Float64>, clickCount: Int64) {
        for dragHandler in self.dragHandlers {
            dragHandler.onOneDragMove(
                dragId: id,
                globalX: globalPoint.x,
                globalY: globalPoint.y,
                clickCount: clickCount)
        }
    }
    
    func oneDragEnded(id: Int64, globalPoint: SIMD2<Float64>, clickCount: Int64) {
        for dragHandler in self.dragHandlers {
            dragHandler.onOneDragEnd(
                dragId: id,
                globalX: globalPoint.x,
                globalY: globalPoint.y,
                clickCount: clickCount)
        }
    }
    
    func twoDragsStarted(
        id1: Int64,
        globalPoint1: SIMD2<Float64>,
        clickCount1: Int64,
        id2: Int64,
        globalPoint2: SIMD2<Float64>,
        clickCount2: Int64)
    {
        for dragHandler in self.dragHandlers {
            dragHandler.onTwoDragsStart(
                dragId1: id1,
                globalX1: globalPoint1.x,
                globalY1: globalPoint1.y,
                clickCount1: clickCount1,
                dragId2: id2,
                globalX2: globalPoint2.x,
                globalY2: globalPoint2.y,
                clickCount2: clickCount2)
        }
    }
    
    func twoDragsMoved(
        id1: Int64,
        globalPoint1: SIMD2<Float64>,
        clickCount1: Int64,
        id2: Int64,
        globalPoint2: SIMD2<Float64>,
        clickCount2: Int64)
    {
        for dragHandler in self.dragHandlers {
            dragHandler.onTwoDragsMove(
                dragId1: id1,
                globalX1: globalPoint1.x,
                globalY1: globalPoint1.y,
                clickCount1: clickCount1,
                dragId2: id2,
                globalX2: globalPoint2.x,
                globalY2: globalPoint2.y,
                clickCount2: clickCount2)
        }
    }
    
    func twoDragsEnded(
        id1: Int64,
        globalPoint1: SIMD2<Float64>,
        clickCount1: Int64,
        id2: Int64,
        globalPoint2: SIMD2<Float64>,
        clickCount2: Int64)
    {
        for dragHandler in self.dragHandlers {
            dragHandler.onTwoDragsEnd(
                dragId1: id1,
                globalX1: globalPoint1.x,
                globalY1: globalPoint1.y,
                clickCount1: clickCount1,
                dragId2: id2,
                globalX2: globalPoint2.x,
                globalY2: globalPoint2.y,
                clickCount2: clickCount2)
        }
    }
    
    func createSprite() -> Sprite {
        return createSpriteOn(parent: self.rootGroup)
    }
    
    func createGroup() -> SpriteGroup {
        return createGroupOn(parent: self.rootGroup)
    }
}
