//
//  ExtHandlerRegistration.swift
//  Enchantron
//
//  Created by Kevin Guthrie on 7/7/18.
//  Copyright © 2018 Rook And Pawn Industries, Inc. All rights reserved.
//

import Foundation

public class HandlerRegistration {
  
  public class func get_binding() -> ext_handler_registration {
    return ext_handler_registration(
      deregister: deregister_callback,
      destroy: destroy
    )
  }
  
  let deregister: () -> Void
  
  init(deregister: @escaping () -> Void) {
    self.deregister = deregister
  }
  
  deinit {
    print("Dropping Handler Registration")
  }
}

private func deregister_callback(registration: UnsafeMutableRawPointer?) {
  let obj: HandlerRegistration = Unmanaged.fromOpaque(UnsafeRawPointer(registration!)).takeUnretainedValue()
  (obj.deregister)()
}

private func destroy(ref: UnsafeMutableRawPointer?) {
  let _ : HandlerRegistration
      = Unmanaged.fromOpaque(UnsafeRawPointer(ref!)).takeRetainedValue()
}
