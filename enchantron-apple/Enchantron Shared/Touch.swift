//
//  Touch.swift
//  Enchantron
//
//  Created by Kevin Guthrie on 10/18/19.
//  Copyright © 2019 Rook And Pawn Industries, Inc. All rights reserved.
//

import SpriteKit

class Touch {
    
    static func average(_ p1: CGPoint, _ p2: CGPoint) ->  CGPoint {
        CGPoint(x: (p1.x + p2.x) / 2.0, y: (p1.y + p2.y) / 2.0)
    }
    
    let original : UITouch?
    let container : SKNode?
    var windowPoint: CGPoint
    var localPoint: CGPoint
    
    convenience init(_ touch: UITouch, _ container: SKNode) {
        self.init(touch,
                  container: container,
                  windowPoint: touch.location(in: nil),
                  localPoint: touch.location(in: container))
    }
    
    init(_ original : UITouch?,
         container: SKNode?,
         windowPoint: CGPoint,
         localPoint: CGPoint) {
        self.original = original
        self.container = container
        self.windowPoint = windowPoint
        self.localPoint = localPoint
    }
    
    func equals(_ touch: UITouch) -> Bool {
        touch === self.original
    }
    
    func distanceTo(_ other: Touch) -> Double {
        return Double(hypot(self.windowPoint.x - other.windowPoint.x,
                            self.windowPoint.y - other.windowPoint.y))
    }
    
    func midpoint(_ other: Touch) -> Touch {
        Touch(nil,
              container: nil,
              windowPoint: Touch.average(self.windowPoint, other.windowPoint),
              localPoint: Touch.average(self.localPoint, other.localPoint))
    }
    
    func update() -> Touch {
        self.windowPoint = self.original!.location(in: nil)
        self.localPoint = self.original!.location(in: container!)
        return self
    }
}


extension Touch : Equatable {
    
    static func ==(lhs: Touch, rhs: Touch) -> Bool {
        return lhs === rhs
    }
}

extension Touch: Hashable {
    
    func hash(into hasher: inout Hasher) {
        hasher.combine(self.original)
    }
}
