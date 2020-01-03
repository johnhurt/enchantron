//
//  MainMenu.swift
//  Enchantron
//
//  Created by Kevin Guthrie on 7/19/18.
//  Copyright © 2018 Rook And Pawn Industries, Inc. All rights reserved.
//

import Foundation
import SpriteKit

class MainMenuView : BaseView {
  private static let MAX_WIDTH_FRAC : CGFloat = 0.5
  private static let HEIGHT_FRAC : CGFloat = 0.2
  private static let BUTTON_ASPECT_RATIO : CGFloat = 1.618
  
  
  let startNewGameButton : Button
  
  
  override init() {
    
    let startNewGameButton = Button()
    startNewGameButton.setFillColor(fillColor: SKColor.cyan)
    
    self.startNewGameButton = startNewGameButton
    
    super.init()
    
    addChild(startNewGameButton)
  }
  
  required init?(coder aDecoder: NSCoder) {
    fatalError("init(coder:) has not been implemented")
  }
  
  func transitionToGameView() {
    transitionTo(newView: GameView(), binder: getContext().transitionToGameView)
  }
  
  override func layout(size: CGSize) {
    let maxHeight = size.height * MainMenuView.HEIGHT_FRAC
    let maxWidth = size.width * MainMenuView.MAX_WIDTH_FRAC
    
    let width = min(maxWidth, maxHeight * MainMenuView.BUTTON_ASPECT_RATIO)
    let height = width / MainMenuView.BUTTON_ASPECT_RATIO
    
    startNewGameButton.setSize(size: CGSize(
      width: width,
      height: height))
    startNewGameButton.position = CGPoint(x: size.width / 2.0, y: -size.height / 2.0)
  }
  
  deinit {
    print("Dropping MainMenuView")
  }
}

