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
    
    labelNode.fontSize = size.height / 2
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
  
  public func toExt() -> ext_button {
    let ownedPointer = UnsafeMutableRawPointer(Unmanaged.passRetained(self).toOpaque())
    
    return ext_button(
      click_handlers: ext_has_click_handlers(
        _self: ownedPointer,
        add_handler: addClickHandler
      ),
      text: ext_has_text(
        _self: ownedPointer
        , get_text: getText
        , set_text: setText
      )
    )
  }
  
  func removeHandler(handler: ClickHandler) {
    objc_sync_enter(click_handlers)
    if let index = click_handlers.index(of: handler) {
      click_handlers.remove(at: index)
    }
    objc_sync_exit(click_handlers)
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


private func getText(ref: UnsafeMutableRawPointer?) -> ext_text {
  let button : Button = Unmanaged.fromOpaque(UnsafeRawPointer(ref!)).takeUnretainedValue()
  let data = button.labelNode.text?.data(using: String.Encoding.utf8, allowLossyConversion: false)!
  let length = data!.count
  let targetContent = allocate_string(Int32(length))!
  data!.copyBytes(to: targetContent, count: length)
  return ext_text(length: Int32(length), content: targetContent)
}

private func setText(ref: UnsafeMutableRawPointer?, text: ext_text) {
  let button : Button = Unmanaged.fromOpaque(UnsafeRawPointer(ref!)).takeUnretainedValue()
  let data = Data(bytes: UnsafeRawPointer(text.content!), count: Int(text.length))
  
  DispatchQueue.main.async {
    button.labelNode.text = String(data: data, encoding: String.Encoding.utf8)
  }
}

private func addClickHandler(ref: UnsafeMutableRawPointer?, extHandler: ext_click_handler)
    -> ext_handler_registration {
  let button : Button = Unmanaged.fromOpaque(UnsafeRawPointer(ref!)).takeUnretainedValue()
  let handler = ClickHandler(extHandler: extHandler)
  button.click_handlers.append(handler)
  let handlerRegistration = HandlerRegistration(deregister: {
        () -> Void in button.removeHandler(handler: handler)
  })
      
  return handlerRegistration.toExt()
}
