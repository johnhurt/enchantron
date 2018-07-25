//
//  MainMenu.swift
//  Enchantron
//
//  Created by Kevin Guthrie on 7/19/18.
//  Copyright © 2018 Rook And Pawn Industries, Inc. All rights reserved.
//

import Foundation
import SpriteKit

class MainMenuView : SKNode {
  
  public class func get_binding() -> ext_main_menu_view {
    return ext_main_menu_view(
      get_start_game_button: get_start_game_button,
      transition_to_game_view: transition_to_game_view,
      destroy: destroy)
  }
  
  let startGameButton : Button
  
  override init() {
    
    let startGameButton = Button(size: CGSize(width: 400, height: 200))
    startGameButton.setFillColor(fillColor: SKColor.cyan)
    
    self.startGameButton = startGameButton
    
    super.init()
    
    addChild(startGameButton)
    
  }
  
  required init?(coder aDecoder: NSCoder) {
    fatalError("init(coder:) has not been implemented")
  }
  
}

private func transition_to_game_view(ref: UnsafeMutableRawPointer?) -> Void {
  
}

private func get_start_game_button(ref: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
  let _self : MainMenuView
      = Unmanaged.fromOpaque(UnsafeRawPointer(ref!)).takeUnretainedValue()
  return UnsafeMutableRawPointer(Unmanaged.passRetained(_self.startGameButton).toOpaque())
}

private func destroy(ref: UnsafeMutableRawPointer?) {
  let _ : MainMenuView
      = Unmanaged.fromOpaque(UnsafeRawPointer(ref!)).takeRetainedValue()
}
