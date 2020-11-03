//
//  DragHandler.swift
//  FourFours
//
//  Created by Kevin Guthrie on 8/31/18.
//  Copyright © 2018 Rook And Pawn Industries, Inc. All rights reserved.
//

import Foundation

extension MultiTouchHandler : Equatable {
  
  static func ==(lhs: MultiTouchHandler, rhs: MultiTouchHandler) -> Bool {
    return lhs === rhs
  }
}
