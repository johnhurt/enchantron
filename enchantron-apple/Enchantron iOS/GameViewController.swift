//
//  GameViewController.swift
//  FourFours iOS
//
//  Created by Kevin Guthrie on 8/9/18.
//  Copyright © 2018 Rook And Pawn Industries, Inc. All rights reserved.
//

import UIKit
import Metal
import MetalKit
import simd
import GameplayKit

class GameViewController: UIViewController {
    
    var renderer: Renderer!
    var mtkView: MTKView!
    private let touchTracker = TouchLookup()
    
    override func viewDidLoad() {
        super.viewDidLoad()
        
        guard let mtkView = self.view as? MTKView else {
            print("View of Gameview controller is not an MTKView")
            return
        }

        // Select the device to render with.  We choose the default device
        guard let defaultDevice = MTLCreateSystemDefaultDevice() else {
            print("Metal is not supported")
            return
        }

        mtkView.device = defaultDevice
        mtkView.backgroundColor = UIColor.black

        guard let newRenderer = Renderer(metalKitView: mtkView, screenScale: Float64(UIScreen.main.scale)) else {
            print("Renderer cannot be initialized")
            return
        }

        renderer = newRenderer

        renderer.mtkView(mtkView, drawableSizeWillChange: mtkView.drawableSize)

        
        
        mtkView.delegate = renderer
    }
    
    override var shouldAutorotate: Bool {
        return true
    }
    
    override var supportedInterfaceOrientations: UIInterfaceOrientationMask {
        if UIDevice.current.userInterfaceIdiom == .phone {
            return .allButUpsideDown
        } else {
            return .all
        }
    }
    
    override func didReceiveMemoryWarning() {
        super.didReceiveMemoryWarning()
        // Release any cached data, images, etc that aren't in use.
    }
    
    override var prefersStatusBarHidden: Bool {
        return true
    }
    
//    override func viewWillTransition(to size: SIMD2<Float64>, with coordinator: UIViewControllerTransitionCoordinator) {
//        super.viewWillTransition(to: size, with: coordinator)
//
//        self.scene?.setSize(size: scaleSize(nativeSize: size))
//    }
    
//    override var acceptsFirstResponder:  Bool {
//        return true
//    }
//
    override func touchesBegan(_ rawTouches: Set<UITouch>, with event: UIEvent?) {
        let touches = touchTracker.filterForNewActiveTouches(newTouches: rawTouches)
        
        self.renderer.dragsStart(touches: touches)
        
    }
    
    override func touchesMoved(_ rawTouches: Set<UITouch>, with event: UIEvent?) {
        let touches = touchTracker.filterForMovedActiveTouches(movedTouches: rawTouches)
        self.renderer.dragsMoved(touches: touches)
        
    }
    
    override func touchesCancelled(_ rawTouches: Set<UITouch>, with event: UIEvent?) {
        let touches = touchTracker.filterForEndedActiveTouches(endedTouches: rawTouches)
        
        self.renderer.dragsEnded(touches: touches)
        
    }
    
    override func touchesEnded(_ touches: Set<UITouch>, with event: UIEvent?) {
        self.touchesCancelled(touches, with: event)
    }
    
}
