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
  let applicationContext : ext_application_context
  let transitioner : TransitionService
  
  init(applictionContext : ext_application_context, transitioner : TransitionService) {
    
    let startGameButton = Button(size: CGSize(width: 400, height: 200))
    startGameButton.setFillColor(fillColor: SKColor.cyan)
    
    self.applicationContext = applictionContext
    self.transitioner = transitioner
    self.startGameButton = startGameButton
    
    super.init()
    
    addChild(startGameButton)
    
  }
  
  required init?(coder aDecoder: NSCoder) {
    fatalError("init(coder:) has not been implemented")
  }
  
  deinit {
    print("Dropping MainMenuView")
  }
}

private func transition_to_game_view(ref: UnsafeMutableRawPointer?) -> Void {
  let _self : MainMenuView
      = Unmanaged.fromOpaque(UnsafeRawPointer(ref!)).takeUnretainedValue()
  
  let applicationContext = _self.applicationContext
  let transitioner = _self.transitioner
  
  let gameView = GameView(
    applictionContext: applicationContext,
    transitioner: transitioner)
  
  let gameViewPointer = UnsafeMutableRawPointer(Unmanaged.passRetained(gameView).toOpaque())
  
  let gamePresenter = bind_game_view(applicationContext, gameViewPointer)
  
  transitioner.transition(view: gameView, viewCleanup: {
    (applicationContext.internal_ui_binding.game_presenter.drop)(gamePresenter)
  })
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
