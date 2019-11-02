//
//  TouchTracker.swift
//  Enchantron
//
//  Created by Kevin Guthrie on 10/18/19.
//  Copyright © 2019 Rook And Pawn Industries, Inc. All rights reserved.
//

import SpriteKit

/// Class for tracking raw touches and converting them into what the application expects which is a distinction between dragging and scaling.  Notice there is no thread safety in this class.  The idea is that this class will only be used from the main thread
class TouchTracker {
    var view: BaseView
    var count : uint_fast8_t = 0
    var touch1: Touch?
    var touch2: Touch?
    var distanceBetweenTouches: Double?
    var midpointTouch: Touch?
    
    init(_ view: BaseView) {
        self.view = view
    }
    
    /// Handles the start of an arbitraty number of touches
    func touchesStarted(_ rawTouches: Set<UITouch>) {
        let touches = rawTouches.map { Touch($0, self.view) }
        
        if touches.count == 1 {
            oneTouchStarted(touches.first!)
        }
        else if touches.count > 1 {
            twoTouchesStarted(touches[0], touches[1])
        }
    }
    
    /// handles the end of an arbitrary number of touches
    func touchesEnded(_ rawTouches: Set<UITouch>) {
        let touches = rawTouches
            .map { self.getTouchFor($0) }
            .filter { $0 != nil }
            .map { $0!.update() }
        
        switch touches.count {
        case 0: print("No meaningful touches ended")
        case 1:
            if touches[0] == self.touch1! {
                touchOneEnded()
            }
            else {
                touchTwoEnded()
            }
        case 2:
            pinchAndDragEnded()
        default: fatalError("Invalid touch state")
        }
    }
    
    /// Called on touches moved events
    func touchesMoved(_ rawTouches: Set<UITouch>) {
        let touches = rawTouches
            .map { self.getTouchFor($0) }
            .filter { $0 != nil }
            .map { $0!.update() }
            
        
        
        switch touches.count {
        case 0: print("No meaningful touches moved")
        case 1: fallthrough
        case 2: touchesMoved()
        default: fatalError("Invalid touch state")
        }
    }
    
    private func startDrag(_ touch: Touch) {
        view.dragStart(
            windowPoint: touch.windowPoint,
            localPoint: touch.localPoint)
    }

    private func moveDrag(_ touch: Touch) {
        view.dragMoved(
            windowPoint: touch.windowPoint,
            localPoint: touch.localPoint)
    }
    
    private func endDrag(_ touch: Touch) {
        view.dragEnded(
            windowPoint: touch.windowPoint,
            localPoint: touch.localPoint)
    }
    
    private func magnify(_ scaleChange: Double, _ touch: Touch) {
        view.magnify(
            scaleChangeAdditive: CGFloat(scaleChange),
            centerPoint: touch.windowPoint)
    }
    
    private func getTouchFor(_ touch: UITouch) -> Touch? {
        (touch1?.equals(touch) ?? false) ? self.touch1 :
            (touch2?.equals(touch) ?? false) ? self.touch2 : nil
    }
    
    /// This method is called when a single touch starts
    private func oneTouchStarted(_ touch: Touch) {
        switch count {
        case 0: dragStarted(touch)
        case 1: pinchStartedAfterDrag(touch)
        default: print("Ignoring additional touches after 2")
        }
    }
    
    /// This method is called when two touches are started at the same time
    private func twoTouchesStarted(_ touch1: Touch, _ touch2: Touch) {
        switch count {
        case 0: pinchAndDragStarted(touch1: touch1, touch2: touch2)
        case 1: pinchStartedAfterDrag(touch1)
        default: print("Ignoring additional touches after 2")
        }
    }
    
    /// Signals that touch1 and only touch1 ended
    private func touchOneEnded() {
        switch count {
        case 1: dragOnlyEnded()
        case 2: pinchEnded(self.touch1!)
        default: fatalError("Invalid touch state?")
        }
    }
    
    /// Signals that touch2 and only touch2 ended
    private func touchTwoEnded() {
        switch count {
        case 2: pinchEnded(self.touch2!)
        default: fatalError("Invalid touch state?")
        }
    }
    
    /// Indicates that touch1 or touch2 moved
    private func touchesMoved() {
        switch count {
        case 1: moveDrag(self.touch1!)
        case 2: pinchDragMoved()
        default: fatalError("Invalid touch state?")
        }
    }
    
    /// Signals the start of a simple drag gesture meaning only one touch is starting and there are no touches in progress
    private func dragStarted(_ touch: Touch) {
        self.count = 1
        self.touch1 = touch
        self.startDrag(self.touch1!)
    }
    
    /// Inidcates that the given touch is being added to an existing drag.  The second touch (and its relation to the first touch) control the scaling
    private func pinchStartedAfterDrag(_ touch2: Touch) {
        self.count = 2
        self.touch2 = touch2
        self.distanceBetweenTouches = self.touch1!.distanceTo(self.touch2!)
        self.midpointTouch = self.touch1!.midpoint(self.touch2!)
        
        self.endDrag(self.touch1!)
        self.startDrag(self.midpointTouch!)
    }
    
    /// Indicates that the pinch and drag gesture started at the same time
    private func pinchAndDragStarted(touch1: Touch, touch2: Touch) {
        self.count = 2
        self.touch1 = touch1
        self.touch2 = touch2
        self.distanceBetweenTouches = self.touch1!.distanceTo(self.touch2!)
        self.midpointTouch = self.touch1!.midpoint(self.touch2!)
        
        self.startDrag(self.midpointTouch!)
    }
    
    /// Signals that the simple drag gesture is ended
    private func dragOnlyEnded() {
        count = 0
        let touch = touch1!
        touch1 = nil
        
        endDrag(touch)
    }
    
    /// Signals that the current pinch gesture ended by ending the given touch.  The given touch is
    /// guaranteed to be either self.touch1 or self.touch2
    private func pinchEnded(_ touch: Touch) {
        count = 1
        distanceBetweenTouches = nil
        midpointTouch = self.touch1!.midpoint(self.touch2!)
        endDrag(midpointTouch!)
        
        if touch == touch1 {
            touch1 = touch2!
        }

        touch2 = nil
        midpointTouch = nil
        self.startDrag(touch1!)
    }
    
    /// Signals that both touch1 and touch2 are ending at the same time
    private func pinchAndDragEnded() {
        midpointTouch = self.touch1!.midpoint(self.touch2!)
        
        endDrag(midpointTouch!)
        
        touch1 = nil
        touch2 = nil
        distanceBetweenTouches = nil
        midpointTouch = nil
        count = 0
    }
    
    /// Signals that we are in pinch/drag moved and at least one of the touches moved
    private func pinchDragMoved() {
        midpointTouch = touch1!.midpoint(touch2!)
        let oldDistance = distanceBetweenTouches!
        distanceBetweenTouches = touch1!.distanceTo(touch2!)
        
        let scaleChange = distanceBetweenTouches! / oldDistance - 1.0
        self.magnify(scaleChange, midpointTouch!)
        
    }
    
}
