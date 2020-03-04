//
//  GameView.swift
//  FourFours
//
//  Created by Kevin Guthrie on 8/21/18.
//  Copyright © 2018 Rook And Pawn Industries, Inc. All rights reserved.
//

import Foundation
import SpriteKit

class GameView : BaseView {
  
  override init() {
    super.init()
  }
  
  required init?(coder aDecoder: NSCoder) {
    fatalError("init(coder:) has not been implemented")
  }
  
  deinit {
    print("Dropping GameView")
  }
}

extension GameView : SpriteSource {
    func createSprite() -> Sprite {
        return createSpriteOn(parent: self)
    }
    
    func createGroup() -> SpriteGroup {
        return createGroupOn(parent: self)
    }
}


