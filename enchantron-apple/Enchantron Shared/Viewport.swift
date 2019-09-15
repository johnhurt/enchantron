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
    var size = CGSize(width: 0, height: 0)
    var zeroPosition = CGPoint(x: 0, y: 0)
    
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
        self.zeroPosition = CGPoint(x: size.width / 2.0 , y: -size.height / 2.0)
        self.position = self.zeroPosition
        resize(size: size)
    }
    
    func resize(size: CGSize) {
        self.container.position = CGPoint(x: -size.width / 2.0, y : size.height / 2.0)
        let positionShift = CGPoint(x: (self.size.width - size.width) / 2.0, y: (self.size.height - size.height) / 2.0 )
        self.zeroPosition = CGPoint(x: self.zeroPosition.x - positionShift.x,
                                    y: self.zeroPosition.y + positionShift.y)
        let newPosition = CGPoint(x: self.position.x - positionShift.x,
                                  y: self.position.y + positionShift.y);
        self.size = size
        self.position = newPosition
    }
    
    func setVisible(_ visible: Bool) {
        let action = visible ? SKAction.unhide() : SKAction.hide() 
        run(action)
    }
    
    func setScaleAnimated(_ newScale: Float64, _ durationSeconds: Float64) {
        
        let rescale = SKAction.scale(
            to: CGFloat(newScale),
            duration: durationSeconds)
        
        if durationSeconds > 0.0 {
            rescale.timingMode = .easeInEaseOut
        }
        
        run(rescale)
    }
    
    func setLocationAnimated(_ left: Float64, _ top: Float64, _ durationSeconds: Float64) {
        let move = SKAction.move(
            to: CGPoint(
                x: CGFloat(left) + zeroPosition.x,
                y: -CGFloat(top) + zeroPosition.y),
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
