//
//  GameScene.swift
//  Enchanter Shared
//
//  Created by Kevin Guthrie on 6/12/18.
//  Copyright © 2018 Rook And Pawn Industries, Inc. All rights reserved.
//

import SpriteKit

class GameScene: SKScene {
  
  var activeView: SKNode?
  
  class func get_ui_binding() -> ext_ui_binding {
    return ext_ui_binding(
      main_menu_view: MainMenuView.get_binding(),
      game_view: GameView.get_binding(),
      button: Button.get_binding(),
      handler_registration: HandlerRegistration.get_binding(),
      texture: Texture.get_binding())
  }
  
  class func get_native_binding() -> ext_native_binding {
    return ext_native_binding(
      texture_loader: TextureLoader.get_binding())
  }
  
  class func newGameScene() -> GameScene {
    // Load 'GameScene.sks' as an SKScene.
    guard let scene = SKScene(fileNamed: "EmptyScene") as? GameScene else {
      print("Failed to load MainMenuScene.sks")
      abort()
    }
  
    // Set the scale mode to scale to fit the window
    scene.scaleMode = .aspectFill
  
    return scene
  }

  var applicationContext : ext_application_context?
  
  func setUpScene() {
    let textureLoader = TextureLoader()
    
    self.applicationContext = create_application_context(
      UnsafeMutableRawPointer(Unmanaged.passRetained(textureLoader).toOpaque()),
      GameScene.get_ui_binding(),
      GameScene.get_native_binding())
    
    // Establish the bindings between rust types accessed by pointer
    ClickHandler.set_binding(int_binding: applicationContext!.internal_ui_binding.click_handler)
    RustString.set_binding(int_binding: applicationContext!.internal_ui_binding.rust_string)
    
    let mainMenuView = MainMenuView()
    
    bind_main_menu_view(
        applicationContext!,
        UnsafeMutableRawPointer(Unmanaged.passRetained(mainMenuView).toOpaque()))
    
    addChild(mainMenuView)
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
