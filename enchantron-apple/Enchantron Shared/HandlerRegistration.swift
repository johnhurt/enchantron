//
//  ExtHandlerRegistration.swift
//  Enchantron
//
//  Created by Kevin Guthrie on 7/7/18.
//  Copyright © 2018 Rook And Pawn Industries, Inc. All rights reserved.
//

import Foundation

public class HandlerRegistration {
  
  let deregister: () -> Void
  
  init(deregister: @escaping () -> Void) {
    self.deregister = deregister
  }
  
  public func toExt() -> ext_handler_registration {
    let ownedPointer = UnsafeMutableRawPointer(Unmanaged.passRetained(self).toOpaque())
    return ext_handler_registration(
        _self: ownedPointer,
        deregister: deregister_callback 
    )
  }
}

private func deregister_callback(registration: UnsafeMutableRawPointer?) {
  let obj: HandlerRegistration = Unmanaged.fromOpaque(UnsafeRawPointer(registration!)).takeUnretainedValue()
  (obj.deregister)()
}
