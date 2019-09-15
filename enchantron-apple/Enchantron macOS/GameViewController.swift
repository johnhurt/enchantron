//
//  GameViewController.swift
//  FourFours macOS
//
//  Created by Kevin Guthrie on 8/9/18.
//  Copyright © 2018 Rook And Pawn Industries, Inc. All rights reserved.
//

import Cocoa
import SpriteKit
import GameplayKit

class GameViewController: NSViewController, NSWindowDelegate {
    
    var skView : SKView?
    var scene : GameScene?
    var screenScale : CGFloat?
    
    override func viewDidLoad() {
        super.viewDidLoad()
        
        // Present the scene
        self.skView = self.view as? SKView
        
        self.scene = GameScene.newGameScene(size: (self.skView?.bounds.size)!)
        self.skView!.presentScene(self.scene)
        
        self.skView!.ignoresSiblingOrder = true
        
        self.skView!.showsFPS = true
        self.skView!.showsNodeCount = true
    }
    
    override func viewDidAppear() {
        self.view.window?.delegate = self
        handleLayout()
    }
    
    func windowDidResize(_ notification: Notification) {
        handleLayout()
    }
    
    func handleLayout() {
        let size = self.view.window?.contentView?.bounds.size;
        if (size != nil) {
            self.scene?.setSize(size: size!)
        }
        
    }
    
    override func magnify(with event: NSEvent) {
        self.scene?.magnify(with: event)
    }
    
}

