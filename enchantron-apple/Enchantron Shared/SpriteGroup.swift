//
//  SpriteGroup.swift
//  Enchantron
//
//  Created by Kevin Guthrie on 3/4/20.
//  Copyright © 2020 Rook And Pawn Industries, Inc. All rights reserved.
//

import SpriteKit

class SpriteGroup : SKNode {
    
    func setZLevel(_ zLevel: Float64) {
        self.zPosition = CGFloat(zLevel)
    }
    
    func setVisible(_ visible: Bool) {
        DispatchQueue.main.async {
            self.isHidden = !visible
        }
    }
    
}


extension SpriteGroup : SpriteSource {
    func createSprite() -> Sprite {
        return createSpriteOn(parent: self)
    }
    
    func createGroup() -> SpriteGroup {
        return createGroupOn(parent: self)
    }
}
