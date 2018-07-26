//
//  TransitionService.swift
//  Enchantron
//
//  Created by Kevin Guthrie on 7/25/18.
//  Copyright © 2018 Rook And Pawn Industries, Inc. All rights reserved.
//

import Foundation
import SpriteKit

class TransitionService {
  
  let transitionClosure: (SKNode, @escaping () -> Void) -> Void
  
  init(transitionClosure: @escaping (SKNode, @escaping () -> Void) -> Void ) {
    self.transitionClosure = transitionClosure
  }
  
  func transition(view: SKNode, viewCleanup: @escaping () -> Void) {
    transitionClosure(view, viewCleanup)
  }
  
  deinit{
    print("Dropping Transition Service")
  }
}
