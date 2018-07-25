//
//  ClickHandler.swift
//  Enchantron
//
//  Created by Kevin Guthrie on 7/7/18.
//  Copyright © 2018 Rook And Pawn Industries, Inc. All rights reserved.
//

import Foundation

class ClickHandler : Equatable {
  
  private static var binding : ext_click_handler?
  
  public class func set_binding(int_binding: ext_click_handler?) {
    ClickHandler.binding = int_binding
  }
  
  let handler : () -> Void
  let extHandler: UnsafeMutableRawPointer?
  
  public init(extHandler: UnsafeMutableRawPointer?) {
    self.extHandler = extHandler
    handler = { () -> Void in (ClickHandler.binding!.on_click)(extHandler) }
  }
  
  public func onClick() {
    handler()
  }
  
  static func ==(lhs: ClickHandler, rhs: ClickHandler) -> Bool {
    return lhs === rhs
  }
  
  deinit {
    ClickHandler.binding?.drop(extHandler)
  }
}
