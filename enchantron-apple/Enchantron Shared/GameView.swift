//
//  GameView.swift
//  FourFours
//
//  Created by Kevin Guthrie on 8/21/18.
//  Copyright © 2018 Rook And Pawn Industries, Inc. All rights reserved.
//

import Foundation
import SpriteKit

class GameView : BaseView {
  
  var dragHandlers: [DragHandler] = []
  var layoutHandlers: [LayoutHandler] = []
  
  static var z = 1;
  
  override init() {
    super.init()
  }
  
  required init?(coder aDecoder: NSCoder) {
    fatalError("init(coder:) has not been implemented")
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
      if let index = self.layoutHandlers.index(of: handler) {
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
      if let index = self.dragHandlers.index(of: handler) {
        self.dragHandlers.remove(at: index)
      }
    }
  }
  
  override func layout(size: CGSize) {
    layoutHandlers.forEach { (handler) in
      handler.onLayout(width: Int64(size.width), height: Int64(size.height))
    }
  }
  
  func createSprite() -> Sprite {
    let result = Sprite()
    
    let onMain : () -> () = {
      result.zPosition = CGFloat(GameView.z)
      GameView.z += 1
      self.addChild(result)
    }
    
    if Thread.isMainThread {
      onMain()
    }
    else {
      DispatchQueue.main.sync { onMain() }
    }
    return result
  }
  
  deinit {
    print("Dropping GameView")
  }
}


extension GameView {
  
  #if os(iOS) || os(tvOS)
  override func touchesBegan(_ touches: Set<UITouch>, with event: UIEvent?) {
    
    DispatchQueue.main.async {
      self.eventSink?.touchesBegan(touches, with: event)
      
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
      self.eventSink?.touchesMoved(touches, with: event)
      
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
      self.eventSink?.touchesEnded(touches, with: event)
      
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
