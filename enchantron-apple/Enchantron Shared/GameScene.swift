//
//  GameScene.swift
//  Enchanter Shared
//
//  Created by Kevin Guthrie on 6/12/18.
//  Copyright © 2018 Rook And Pawn Industries, Inc. All rights reserved.
//

import SpriteKit

class GameScene: SKScene {
  
  var startGameButton : Button?
  var mainMenuPresenter : ext_main_menu_presenter?
  
    class func newGameScene() -> GameScene {
        // Load 'GameScene.sks' as an SKScene.
        guard let scene = SKScene(fileNamed: "MainMenuScene") as? GameScene else {
            print("Failed to load MainMenuScene.sks")
            abort()
        }
        
        // Set the scale mode to scale to fit the window
        scene.scaleMode = .aspectFill
        
        return scene
    }
  
  func toExt() -> ext_main_menu_view {
    let ownedPointer = UnsafeMutableRawPointer(Unmanaged.passRetained(self).toOpaque())
    
    return ext_main_menu_view(
      _self: ownedPointer,
      start_game_button: startGameButton!.toExt()
    )
  }
  
    func setUpScene() {
      let startGameButton = Button(size: CGSize(width: 400, height: 200))
      startGameButton.setFillColor(fillColor: SKColor.cyan)
      
      addChild(startGameButton)
      
      self.startGameButton = startGameButton
      
      mainMenuPresenter = bind_main_menu_view(toExt())
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

    override func update(_ currentTime: TimeInterval) {
        // Called before each frame is rendered
    }
}

#if os(iOS) || os(tvOS)
// Touch-based event handling
extension GameScene {

    override func touchesBegan(_ touches: Set<UITouch>, with event: UIEvent?) {
    }
    
    override func touchesMoved(_ touches: Set<UITouch>, with event: UIEvent?) {
    }
    
    override func touchesEnded(_ touches: Set<UITouch>, with event: UIEvent?) {
    }
    
    override func touchesCancelled(_ touches: Set<UITouch>, with event: UIEvent?) {
    }
    
   
}
#endif

#if os(OSX)
// Mouse-based event handling
extension GameScene {

    override func mouseDown(with event: NSEvent) {
    }
    
    override func mouseDragged(with event: NSEvent) {
    }
    
    override func mouseUp(with event: NSEvent) {
    }

}
#endif
