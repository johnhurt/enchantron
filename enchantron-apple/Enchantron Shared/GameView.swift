//
//  GameView.swift
//  Enchantron iOS
//
//  Created by Kevin Guthrie on 7/24/18.
//  Copyright © 2018 Rook And Pawn Industries, Inc. All rights reserved.
//

import Foundation
import SpriteKit

class GameView : SKNode {
  
  let origin : CGPoint = CGPoint(x: 0, y: 0)
  let size : CGSize = CGSize(width: 100, height: 100)
  
  let transitioner : TransitionService
  
  init(transitioner : TransitionService) {
        self.transitioner = transitioner
    
    super.init()
  }
  
  required init?(coder aDecoder: NSCoder) {
    fatalError("init(coder:) has not been implemented")
  }
  
  deinit {
    print("Dropping GameView")
  }
  
}

private func get_width(ref: UnsafeMutableRawPointer?) -> Int64 {
  let _self : GameView = Unmanaged.fromOpaque(UnsafeRawPointer(ref!)).takeUnretainedValue()
  return Int64(_self.size.width)
}

private func get_height(ref: UnsafeMutableRawPointer?) -> Int64 {
  let _self : GameView = Unmanaged.fromOpaque(UnsafeRawPointer(ref!)).takeUnretainedValue()
  return Int64(_self.size.height)
}

private func get_x(ref: UnsafeMutableRawPointer?) -> Int64 {
  let _self : GameView = Unmanaged.fromOpaque(UnsafeRawPointer(ref!)).takeUnretainedValue()
  return Int64(_self.origin.x)
}

private func get_y(ref: UnsafeMutableRawPointer?) -> Int64 {
  let _self : GameView = Unmanaged.fromOpaque(UnsafeRawPointer(ref!)).takeUnretainedValue()
  return Int64(_self.origin.y)
}

private func destroy(ref: UnsafeMutableRawPointer?) {
  let _ : GameView
      = Unmanaged.fromOpaque(UnsafeRawPointer(ref!)).takeRetainedValue()
}
