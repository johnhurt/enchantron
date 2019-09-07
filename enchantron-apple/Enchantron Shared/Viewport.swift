//
//  Viewport.swift
//  Enchantron iOS
//
//  Created by Kevin Guthrie on 6/8/19.
//  Copyright © 2019 Rook And Pawn Industries, Inc. All rights reserved.
//

import Foundation
import SpriteKit

public class Viewport : SKCameraNode {
    
    let container = SKNode()
    
    public override init() {
        super.init()
        self.addChild(container)
    }
    
    required init?(coder aDecoder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }
    
    func reset(size: CGSize) {
        self.zRotation = 0.0;
        self.xScale = 1.0;
        self.yScale = 1.0;
        self.container.removeAllActions()
        resize(size: size)
    }
    
    func resize(size: CGSize) {
        self.position = CGPoint(x: size.width / 2.0 , y: -size.height / 2.0)
        self.container.position = CGPoint(x: -size.width / 2.0, y : size.height / 2.0)
    }
    
    func setVisible(_ visible: Bool) {
        let action = visible ? SKAction.unhide() : SKAction.hide() 
        run(action)
    }
    
    func setSizeAnimated(_ width: Float64, _ height: Float64, _ durationSeconds: Float64) {
        let resize = SKAction.resize(
            toWidth: CGFloat(width),
            height: CGFloat(height),
            duration: durationSeconds)
        
        if durationSeconds > 0.0 {
            resize.timingMode = .easeInEaseOut
        }
        
        run(resize)
    }
    
    func setLocationAnimated(_ left: Float64, _ top: Float64, _ durationSeconds: Float64) {
        let move = SKAction.move(
            to: CGPoint(x: CGFloat(left), y: -CGFloat(top)),
            duration: durationSeconds)
        
        if durationSeconds > 0.0 {
            move.timingMode = .easeInEaseOut
        }
        
        run(move)
    }
}


extension Viewport : SpriteSource {
    func createSprite() -> Sprite {
        return createSpriteOn(parent: self.container)
    }
}
