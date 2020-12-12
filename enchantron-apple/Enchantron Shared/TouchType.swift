//
//  HasTouchLocation.swift
//  Enchantron
//
//  Created by Kevin Guthrie on 12/9/20.
//  Copyright © 2020 Rook And Pawn Industries, Inc. All rights reserved.
//

protocol HasTouchInfo : Hashable {
    
    func getTouchLocation() -> SIMD2<Float64>
    func getTapCount() -> Int64
    
}


#if os(iOS) || os(tvOS)
import UIKit
typealias TouchType = UITouch

extension UITouch : HasTouchInfo {
    
    func getTouchLocation() -> SIMD2<Float64> {
        let loc = self.location(in: nil)
        return [Float64(loc.x), Float64(loc.y)]
    }
    
    func getTapCount() -> Int {
        return Int64(self.tapCount)
    }
}

#else

import Cocoa

typealias TouchType = MouseTouch

/// There's no tracking needed for a mouse touch, we just want to wrap it to be able to simplify calls
class MouseTouch {
    let event: NSEvent
    
    init(_ event: NSEvent) {
        self.event = event
    }
}

extension MouseTouch : Hashable {
    static func ==(lhs: MouseTouch, rhs: MouseTouch) -> Bool {
        return true
    }
    
    func hash(into hasher: inout Hasher) {
        hasher.combine(1034)
    }
}

extension MouseTouch : HasTouchInfo {
    func getTapCount() -> Int64 {
        Int64(self.event.clickCount)
    }
    
    func getTouchLocation() -> SIMD2<Float64> {
        return [
            Float64(event.locationInWindow.x),
            Float64(
                (event.window?.contentView?.bounds.size.height)!
                    - event.locationInWindow.y)]
    }
}

#endif
