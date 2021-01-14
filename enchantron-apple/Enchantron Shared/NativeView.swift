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

class NativeView : SpriteSource {
    
    private var dragHandlers: [MultiTouchHandler] = []
    private var layoutHandlers: [LayoutHandler] = []
    private var magnifyHandlers: [MagnifyHandler] = []
    
    private var presenter : BoxedAny?
    
    private let viewport : Viewport
    let device: MTLDevice
    let rootGroup : SpriteGroup
    
    init(screenSize: SIMD2<Float64>, device: MTLDevice) {
        self.viewport = Viewport(screenSize: screenSize, device: device)
        self.device = device
        self.rootGroup = SpriteGroup(device: device, parent: nil)
    }
    
    func render(encoder: MTLRenderCommandEncoder, uniformBufferIndex: Int, time: Float64) {
        viewport.configureViewport(encoder: encoder, uniformBufferIndex: uniformBufferIndex)
        viewport.bindToVertexShader(
            encoder: encoder,
            uniformBufferIndex: uniformBufferIndex,
            bufferIndex: 1)
        rootGroup.render(encoder: encoder, uniformBufferIndex: uniformBufferIndex, time: time)
        viewport.render(encoder: encoder, uniformBufferIndex: uniformBufferIndex, time: time)
    }
    
    func setPresenter(_ presenter: BoxedAny) {
        DispatchQueue.main.async {
            self.presenter = presenter
        }
    }
    
    func unsetPresenter() {
        DispatchQueue.main.async {
            self.presenter = nil
        }
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
        DispatchQueue.main.async {
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
    
    final func layout(size: SIMD2<Float64>) {
        viewport.screenSize = size
        layoutHandlers.forEach { (handler) in
            handler.onLayout(width: size.x, height: size.y)
        }
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
    
    deinit {
        print("Dropping Native View")
    }
}
