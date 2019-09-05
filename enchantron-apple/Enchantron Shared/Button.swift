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
  
  var clickHandlers: [ClickHandler] = []
  
  private var size: CGSize?
  
  let shapeNode = SKShapeNode()
  let labelNode = SKLabelNode()
  
  override init() {
    super.init()
    self.isUserInteractionEnabled = true
    addChild(shapeNode)
    addChild(labelNode)
    
  }
  
  required init?(coder aDecoder: NSCoder) {
    fatalError("init(coder:) has not been implemented")
  }
  
  func setSize(size: CGSize) {
    
    if self.size == size {
      return
    }
    
    self.size = size
    shapeNode.path = CGPath(
      rect: CGRect(origin: CGPoint(x: -size.width / 2, y: -size.height / 2), size: size),
      transform: nil)
    
    labelNode.fontSize = size.height / 4
    labelNode.fontColor = SKColor.darkGray
    labelNode.fontName = "Indira K"
    labelNode.horizontalAlignmentMode = SKLabelHorizontalAlignmentMode.center
    labelNode.verticalAlignmentMode = SKLabelVerticalAlignmentMode.center
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
    DispatchQueue.main.sync {
      self.clickHandlers.append(handler)
    }
    return HandlerRegistration(deregister_callback: {
      self.removeHandler(handler)
    })
  }
  
  func removeHandler(_ handler: ClickHandler) {
    DispatchQueue.main.sync {
      if let index = self.clickHandlers.firstIndex(of: handler) {
        self.clickHandlers.remove(at: index)
      }
    }
  }
  
  deinit {
    print("Dropping Button")
  }
}


extension Button {

  #if os(iOS) || os(tvOS)
  override func touchesEnded(_ touches: Set<UITouch>, with event: UIEvent?) {
    DispatchQueue.main.async {
      self.clickHandlers.forEach { (handler) in
        handler.onClick()
      }
    }
  }
  #endif
  
  #if os(OSX)
  override func mouseUp(with event: NSEvent) {
    DispatchQueue.main.async {
      self.clickHandlers.forEach { (handler) in
        handler.onClick()
      }
    }
  }
  #endif
}

