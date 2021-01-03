//
//  GameViewController.swift
//  FourFours macOS
//
//  Created by Kevin Guthrie on 8/9/18.
//  Copyright © 2018 Rook And Pawn Industries, Inc. All rights reserved.
//

import Cocoa
import MetalKit

class GameViewController: NSViewController {
    
    static let TouchId : Int64 = 0
    
    var renderer: Renderer!
    var mtkView: MTKView!
    var screenScale = Float64()
    
    private let touchTracker = TouchLookup()
    
    override func viewDidLoad() {
        super.viewDidLoad()

        guard let mtkView = self.view as? MTKView else {
            print("View attached to GameViewController is not an MTKView")
            return
        }

        // Select the device to render with.  We choose the default device
        guard let defaultDevice = MTLCreateSystemDefaultDevice() else {
            print("Metal is not supported on this device")
            return
        }

        mtkView.device = defaultDevice
        screenScale = Float64(NSScreen.main!.backingScaleFactor)

        guard let newRenderer = Renderer(metalKitView: mtkView, screenScale: screenScale) else {
            print("Renderer cannot be initialized")
            return
        }

        renderer = newRenderer

        renderer.mtkView(mtkView, drawableSizeWillChange: mtkView.drawableSize)

        mtkView.delegate = renderer
    }
    
    override func magnify(with event: NSEvent) {
        let rawMouseLocation = NSEvent.mouseLocation
        let mouseLocationRect = NSRect(
            origin: rawMouseLocation,
            size: CGSize())
        let rawWindowLocation = self.view.window!.convertFromScreen(mouseLocationRect).origin
        
        let mouseLocation : SIMD2<Float64> = [
            Float64(rawWindowLocation.x) * screenScale,
            renderer.screenHeight - Float64(rawWindowLocation.y) * screenScale
        ]
        
        self.renderer.magnify(
            scaleChangeAdditive: Float64(event.magnification),
            centerPoint: mouseLocation)
    }
    
    
    override func mouseDown(with event: NSEvent) {
        let touches = [ (GameViewController.TouchId , MouseTouch(event)) ]
        
        self.renderer.dragsStart(touches: touches)
    }
    
    override func mouseDragged(with event: NSEvent) {
        let touches = [ (GameViewController.TouchId , MouseTouch(event)) ]
        
        self.renderer.dragsMoved(touches: touches)
    }
    
    override func mouseUp(with event: NSEvent) {
        let touches = [ (GameViewController.TouchId , MouseTouch(event)) ]
        
        self.renderer.dragsEnded(touches: touches)
    }
    
}

