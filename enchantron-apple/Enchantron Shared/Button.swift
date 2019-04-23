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
  
  func setText(_ newText: String) {
    DispatchQueue.main.async {
      self.labelNode.text = newText
    }
  }
  
  func getText() -> String {
    return self.labelNode.text!
  }
  
  func addClickHandler(_ handler: ClickHandler) -> HandlerRegistration {
    objc_sync_enter(click_handlers)
    click_handlers.append(handler)
    objc_sync_exit(click_handlers)
    return HandlerRegistration(deregister_callback: {
      self.removeHandler(handler)
    })
  }
  
  func removeHandler(_ handler: ClickHandler) {
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
    objc_sync_enter(click_handlers)
    click_handlers.forEach { (handler) in
      handler.onClick()
    }
    objc_sync_exit(click_handlers)
  }
  #endif
  
  #if os(OSX)
  override func mouseUp(with event: NSEvent) {
    objc_sync_enter(click_handlers)
    click_handlers.forEach { (handler) in
      handler.onClick()
    }
    objc_sync_exit(click_handlers)
  }
  #endif
}

