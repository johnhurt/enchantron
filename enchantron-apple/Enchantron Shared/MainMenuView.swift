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
  
  let startNewGameButton : Button
  let transitioner : TransitionService
  
  init(transitioner : TransitionService) {
    
    let startNewGameButton = Button(size: CGSize(width: 400, height: 200))
    startNewGameButton.setFillColor(fillColor: SKColor.cyan)
    
    self.transitioner = transitioner
    self.startNewGameButton = startNewGameButton
    
    super.init()
    
    addChild(startNewGameButton)
    
  }
  
  required init?(coder aDecoder: NSCoder) {
    fatalError("init(coder:) has not been implemented")
  }
  
  func transitionToGameView() {
//    
//    let transitioner = self.transitioner
//    
//    let gameView = GameView(
//      transitioner: transitioner)
//    
//    let gameViewPointer = UnsafeMutableRawPointer(Unmanaged.passRetained(gameView).toOpaque())
//    
//    let gamePresenter = bind_game_view(applicationContext, gameViewPointer)
//    
//    transitioner.transition(view: gameView, viewCleanup: {
//      (applicationContext.internal_ui_binding.game_presenter.drop)(gamePresenter)
//    })
  }
  
  deinit {
    print("Dropping MainMenuView")
  }
}

