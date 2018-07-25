//
//  GameView.swift
//  Enchantron iOS
//
//  Created by Kevin Guthrie on 7/24/18.
//  Copyright © 2018 Rook And Pawn Industries, Inc. All rights reserved.
//

import Foundation
import SpriteKit

class GameView : SKTexture {
  
  let origin : CGPoint = CGPoint(x: 0, y: 0)
  
  public class func get_binding() -> ext_game_view {
    return ext_game_view(
      get_width: get_width,
      get_height: get_height,
      get_x: get_x,
      get_y: get_y,
      destroy: destroy)
  }
  
}

private func get_width(ref: UnsafeMutableRawPointer?) -> Int64 {
  let _self : GameView = Unmanaged.fromOpaque(UnsafeRawPointer(ref!)).takeUnretainedValue()
  return Int64(_self.size().width)
}

private func get_height(ref: UnsafeMutableRawPointer?) -> Int64 {
  let _self : GameView = Unmanaged.fromOpaque(UnsafeRawPointer(ref!)).takeUnretainedValue()
  return Int64(_self.size().height)
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
