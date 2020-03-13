//
//  TouchLookup.swift
//  Enchantron iOS
//
//  Created by Kevin Guthrie on 3/5/20.
//  Copyright © 2020 Rook And Pawn Industries, Inc. All rights reserved.
//

import UIKit

/// Class for keeping track of active touches.  Rust only supports 2 finger gestures, so this will keep up with which two touches are the ones being sent to rust and what id to assign to them. This class is written to only be used on the main thread, so it is not safe to use from multiple threads at the same time
class TouchLookup {
    
    var activeTouches : [UITouch : Int64] = [:]
    var nextId : Int64 = 0
    
    func filterForNewActiveTouches(newTouches: Set<UITouch>) -> [(Int64, UITouch)] {
        var result = [(Int64, UITouch)]();
        
        for touch in newTouches {
            if let _ = self.activeTouches[touch] {
                continue
            }
            else if activeTouches.count < 2 {
                activeTouches[touch] = nextId
                result.append((nextId, touch))
                nextId += 1
            }
        }
        
        return result
    }
    
    func filterForMovedActiveTouches(movedTouches: Set<UITouch>) -> [(Int64, UITouch)] {
        var result = [(Int64, UITouch)]();
        
        for touch in movedTouches {
            if let id = self.activeTouches[touch] {
                result.append((id, touch))
            }
        }
        
        return result
    }
    
    func filterForEndedActiveTouches(endedTouches: Set<UITouch>) -> [(Int64, UITouch)] {
        var result = [(Int64, UITouch)]();
        
        for touch in endedTouches {
            if let id = self.activeTouches[touch] {
                self.activeTouches.removeValue(forKey: touch)
                result.append((id, touch))
            }
        }
        
        return result
    }
    
}

