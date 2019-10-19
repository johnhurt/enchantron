//
//  GameScene.swift
//  FourFours Shared
//
//  Created by Kevin Guthrie on 8/9/18.
//  Copyright © 2018 Rook And Pawn Industries, Inc. All rights reserved.
//

import SpriteKit

class GameScene: SKScene {
    
    
    class func newGameScene(size: CGSize) -> GameScene {
        // Load 'GameScene.sks' as an SKScene.
        guard let scene = SKScene(fileNamed: "Empty.sks") as? GameScene else {
            print("Failed to load GameScene.sks")
            abort()
        }
        
        // Set the scale mode to scale to fit the window
        scene.size = size
        
        return scene
    }
    
    private var currentView : BaseView?
    private var viewport : Viewport?
    
    func setUpScene() {
        let systemView = SystemView(textureLoader: TextureLoader())
        self.viewport = Viewport()
        self.camera = self.viewport
        
        let ctx = RustBinder.bindToRust(systemView)
        
        let transitioner = TransitionService(preBindTransition: { (view) in
            self.removeAllChildren()
            self.removeAllActions()
            self.viewport!.reset(size: self.size)
            self.addChild(self.viewport!)
            view.setViewport(viewport: self.viewport!)
        }, postBindTransition: { (view) in
            self.addChild(view)
            self.currentView = view
            self.setSize(size: self.size)
        })
        
        let loadingView = LoadingView()
        
        loadingView.initializeCtx(ctx: ctx, transitionService: transitioner)
        transitioner.transition(view: loadingView, presenterBinder: ctx.bindToLoadingView )
    }
    
    #if os(watchOS)
    override func sceneDidLoad() {
        self.setUpScene()
    }
    #else
    override func didMove(to view: SKView) {
        self.setUpScene()
    }
    #endif
    
    func setSize(size: CGSize) {
        let setSizeOp : () -> () = {
            self.size = size
            self.viewport?.resize(size: self.size)
            self.currentView?.setSize(size: self.size)
        }
        
        if Thread.isMainThread {
            setSizeOp()
        }
        else {
            DispatchQueue.main.sync { setSizeOp() }
        }
        
    }
    
    override func update(_ currentTime: TimeInterval) {
        // Called before each frame is rendered
        
    }
    
    func magnify(scaleChangeAdditive: CGFloat, centerPoint: CGPoint) {
        DispatchQueue.main.async {
            self.currentView!.magnify(
                scaleChangeAdditive: scaleChangeAdditive,
                centerPoint: centerPoint)
        }
    }
}

extension GameScene {
    
    
    #if os(iOS) || os(tvOS)
    override func touchesBegan(_ touches: Set<UITouch>, with event: UIEvent?) {
        
        self.currentView?.touchesBegan(touches, with: event)
        
    }
    
    override func touchesMoved(_ touches: Set<UITouch>, with event: UIEvent?) {
        self.currentView?.touchesMoved(touches, with: event )
    }
    
    override func touchesCancelled(_ touches: Set<UITouch>, with event: UIEvent?) {
        self.currentView?.touchesEnded(touches, with: event)
    }
    
    override func touchesEnded(_ touches: Set<UITouch>, with event: UIEvent?) {
        self.currentView?.touchesEnded(touches, with: event)
    }
    
    #endif
    
    #if os(OSX)
    
    
    override func mouseDown(with event: NSEvent) {
        DispatchQueue.main.async {
            let localPoint = event.location(in: self)
            
            self.currentView?.getDragHandlers().forEach { (handler) in
                handler.onDragStart(
                    globalX: Float64(event.locationInWindow.x),
                    globalY: Float64((event.window?.contentView?.bounds.size.height)!
                        - event.locationInWindow.y),
                    localX: Float64(localPoint.x),
                    localY: -Float64(localPoint.y))
            }
        }
    }
    
    override func mouseDragged(with event: NSEvent) {
        DispatchQueue.main.async {
            let localPoint = event.location(in: self)
            self.currentView?.getDragHandlers().forEach { (handler) in
                handler.onDragMove(
                    globalX: Float64(event.locationInWindow.x),
                    globalY: Float64((event.window?.contentView?.bounds.size.height)!
                        - event.locationInWindow.y),
                    localX: Float64(localPoint.x),
                    localY: -Float64(localPoint.y))
            }
        }
    }
    
    override func mouseUp(with event: NSEvent) {
        DispatchQueue.main.async {
            let localPoint = event.location(in: self)
            self.currentView?.getDragHandlers().forEach { (handler) in
                handler.onDragEnd(
                    globalX: Float64(event.locationInWindow.x),
                    globalY: Float64((event.window?.contentView?.bounds.size.height)!
                        - event.locationInWindow.y),
                    localX: Float64(localPoint.x),
                    localY: -Float64(localPoint.y))
            }
        }
    }
    #endif
}

