//
//  ExtHandlerRegistration.swift
//  Enchantron
//
//  Created by Kevin Guthrie on 7/7/18.
//  Copyright © 2018 Rook And Pawn Industries, Inc. All rights reserved.
//

import Foundation

public class HandlerRegistration {
  
  let deregister_callback: () -> Void
  
  init(deregister_callback: @escaping () -> Void) {
    self.deregister_callback = deregister_callback
  }
  
  func deregister() {
    (self.deregister_callback)()
  }
  
  deinit {
    print("Dropping Handler Registration")
  }
}
