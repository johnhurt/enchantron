//
//  Button.swift
//  Enchantron
//
//  Created by Kevin Guthrie on 7/7/18.
//  Copyright © 2018 Rook And Pawn Industries, Inc. All rights reserved.
//

import Foundation
import SpriteKit

class Button : SKNode {
  
  public class func get_binding() -> ext_button {
    return ext_button(
      add_click_handler: add_click_handler,
      get_text: get_text,
      set_text: set_text,
      destroy: destroy)
  }
  
  var click_handlers: [ClickHandler] = []
  let shapeNode = SKShapeNode()
  let labelNode = SKLabelNode()
  
  init(size: CGSize) {
    super.init()
    shapeNode.path = CGPath(
      rect: CGRect(origin: CGPoint(x: -size.width / 2, y: -size.height / 2), size: size),
      transform: nil)
    self.isUserInteractionEnabled = true
    addChild(shapeNode)
    addChild(labelNode)
    
    labelNode.fontSize = size.height / 4
    labelNode.fontColor = SKColor.darkGray
    labelNode.horizontalAlignmentMode = SKLabelHorizontalAlignmentMode.center
    labelNode.verticalAlignmentMode = SKLabelVerticalAlignmentMode.center
  }
  
  required init?(coder aDecoder: NSCoder) {
    fatalError("init(coder:) has not been implemented")
  }
  
  public func setFillColor(fillColor: SKColor) {
    shapeNode.fillColor = fillColor
  }
  
  func removeHandler(handler: ClickHandler) {
    objc_sync_enter(click_handlers)
    if let index = click_handlers.index(of: handler) {
      click_handlers.remove(at: index)
    }
    objc_sync_exit(click_handlers)
  }
  
  deinit {
    print("Dropping Button")
  }
}


extension Button {

  #if os(iOS) || os(tvOS)
  override func touchesEnded(_ touches: Set<UITouch>, with event: UIEvent?) {
    click_handlers.forEach { (handler) in
      handler.onClick()
    }
  }
  #endif
  
  #if os(OSX)
  override func mouseUp(with event: NSEvent) {
    click_handlers.forEach { (handler) in
      handler.onClick()
    }
  }
  #endif
}

private func get_text(ref: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
  let button : Button = Unmanaged.fromOpaque(UnsafeRawPointer(ref!)).takeUnretainedValue()
  let result = SwiftString(source: button.labelNode.text!)
  return UnsafeMutableRawPointer(Unmanaged.passRetained(result).toOpaque())
}

private func set_text(ref: UnsafeMutableRawPointer?, textPointer: UnsafeMutableRawPointer?) {
  let button : Button = Unmanaged.fromOpaque(UnsafeRawPointer(ref!)).takeUnretainedValue()
  let text = RustString(rawPointer: textPointer)
  let content = text.toString()
  DispatchQueue.main.async {
    button.labelNode.text = content
  }
}

private func add_click_handler(ref: UnsafeMutableRawPointer?, extHandler: UnsafeMutableRawPointer?)
    -> UnsafeMutableRawPointer? {
  let button : Button = Unmanaged.fromOpaque(UnsafeRawPointer(ref!)).takeUnretainedValue()
  let handler = ClickHandler(extHandler: extHandler)
  button.click_handlers.append(handler)
  let handlerRegistration = HandlerRegistration(deregister: {
        () -> Void in button.removeHandler(handler: handler)
  })
      
  return UnsafeMutableRawPointer(Unmanaged.passRetained(handlerRegistration).toOpaque())
}


private func destroy(ref: UnsafeMutableRawPointer?) {
  let _ : Button
      = Unmanaged.fromOpaque(UnsafeRawPointer(ref!)).takeRetainedValue()
}

