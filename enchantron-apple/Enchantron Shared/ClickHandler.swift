//
//  ClickHandler.swift
//  Enchantron
//
//  Created by Kevin Guthrie on 7/7/18.
//  Copyright © 2018 Rook And Pawn Industries, Inc. All rights reserved.
//

import Foundation

class ClickHandler : Equatable {
  
  let handler : () -> Void
  
  public init(extHandler: ext_click_handler) {
    let handler_self : UnsafeMutableRawPointer = extHandler._self
    handler = { () -> Void in (extHandler.on_click)(handler_self) }
  }
  
  public func onClick() {
    handler()
  }
  
  static func ==(lhs: ClickHandler, rhs: ClickHandler) -> Bool {
    return lhs === rhs
  }
}
