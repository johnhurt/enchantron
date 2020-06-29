//
//  LoadingView.swift
//  FourFours
//
//  Created by Kevin Guthrie on 8/16/18.
//  Copyright © 2018 Rook And Pawn Industries, Inc. All rights reserved.
//

import Foundation
import SpriteKit

class LoadingView : BaseView {
  private static let MAX_WIDTH_FRAC : CGFloat = 0.5
  private static let HEIGHT_FRAC : CGFloat = 0.2
  private static let BUTTON_ASPECT_RATIO : CGFloat = 1.618
  
  let progressIndicator : ProgressBar
  
  override init() {
   
    self.progressIndicator = ProgressBar()
    super.init()
    addChild(progressIndicator)
  }
  
  required init?(coder aDecoder: NSCoder) {
    fatalError("init(coder:) has not been implemented")
  }
  
  override func localLayout(size: CGSize) {
    let maxHeight = size.height * LoadingView.HEIGHT_FRAC
    let maxWidth = size.width * LoadingView.MAX_WIDTH_FRAC
    
    let width = min(maxWidth, maxHeight * LoadingView.BUTTON_ASPECT_RATIO)
    let height = width / LoadingView.BUTTON_ASPECT_RATIO
    
    progressIndicator.setSize(size: CGSize(
      width: width,
      height: height))
    progressIndicator.position = CGPoint(x: size.width, y: -size.height)
    //progressIndicator.position = CGPoint(x: size.width / 2.0, y: -size.height / 2.0)
  }
  
  func transitionToMainMenuView() {
    transitionTo(newView: MainMenuView(), binder: getContext().transitionToMainMenuView)
  }
  
  deinit {
    print("Dropping Loading view")
  }
}
