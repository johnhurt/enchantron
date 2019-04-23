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
  
  typealias TransitionClosure = (BaseView) -> Void
  
  let transitionClosure : TransitionClosure
  
  init(transitionClosure: @escaping TransitionClosure ) {
    self.transitionClosure = transitionClosure
  }
  
  func transition(view: BaseView) {
    self.transitionClosure(view)
  }
  
  deinit{
    print("Dropping Transition Service")
  }
}
