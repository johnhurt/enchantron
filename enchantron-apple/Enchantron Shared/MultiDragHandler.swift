//
//  DragHandler.swift
//  FourFours
//
//  Created by Kevin Guthrie on 8/31/18.
//  Copyright © 2018 Rook And Pawn Industries, Inc. All rights reserved.
//

import Foundation

extension MultiDragHandler : Equatable {
  
  static func ==(lhs: MultiDragHandler, rhs: MultiDragHandler) -> Bool {
    return lhs === rhs
  }
}
