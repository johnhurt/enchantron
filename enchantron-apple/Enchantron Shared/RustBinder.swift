
// This is an automatically generated file that lets a swift application
// bind to the rust library with a single method call

import Foundation

class RustBinder {
  // Bind all the swift functions into rust
  class func bindToRust(_ systemView: SystemView) -> ApplicationContext {

    // Type SwiftString

    // Impl Drop
    set_swift_string__drop(swift_string__drop)

    // Impl 
    set_swift_string__get_length(swift_string__get_length)
    set_swift_string__get_content(swift_string__get_content)

    // Type HandlerRegistration

    // Impl Drop
    set_handler_registration__drop(handler_registration__drop)

    // Impl ui::HandlerRegistration
    set_handler_registration__deregister(handler_registration__deregister)

    // Type Button

    // Impl HasText
    set_button__get_text(button__get_text)
    set_button__set_text(button__set_text)

    // Impl crate::ui::Button

    // Impl Drop
    set_button__drop(button__drop)

    // Impl HasClickHandlers
    set_button__add_click_handler(button__add_click_handler)

    // Type TextArea

    // Impl HasText
    set_text_area__get_text(text_area__get_text)
    set_text_area__set_text(text_area__set_text)

    // Impl Drop
    set_text_area__drop(text_area__drop)

    // Type ProgressBar

    // Impl HasText
    set_progress_bar__get_text(progress_bar__get_text)
    set_progress_bar__set_text(progress_bar__set_text)

    // Impl Drop
    set_progress_bar__drop(progress_bar__drop)

    // Impl HasIntValue
    set_progress_bar__get_int_value(progress_bar__get_int_value)
    set_progress_bar__set_int_value(progress_bar__set_int_value)

    // Impl ui::ProgressBar

    // Type Texture

    // Impl Drop
    set_texture__drop(texture__drop)

    // Impl native::Texture
    set_texture__get_sub_texture(texture__get_sub_texture)

    // Impl HasIntSize
    set_texture__get_width(texture__get_width)
    set_texture__get_height(texture__get_height)

    // Type Sprite

    // Impl HasMutableLocation
    set_sprite__set_location_animated(sprite__set_location_animated)

    // Impl Drop
    set_sprite__drop(sprite__drop)

    // Impl ui::Sprite
    set_sprite__set_texture(sprite__set_texture)
    set_sprite__propagate_events_to(sprite__propagate_events_to)
    set_sprite__remove_from_parent(sprite__remove_from_parent)

    // Impl HasMutableSize
    set_sprite__set_size_animated(sprite__set_size_animated)

    // Impl HasMutableVisibility
    set_sprite__set_visible(sprite__set_visible)

    // Impl HasDragHandlers
    set_sprite__add_drag_handler(sprite__add_drag_handler)

    // Type LoadingView

    // Impl Drop
    set_loading_view__drop(loading_view__drop)

    // Impl ui::LoadingView
    set_loading_view__get_progress_indicator(loading_view__get_progress_indicator)
    set_loading_view__transition_to_main_menu_view(loading_view__transition_to_main_menu_view)

    // Type MainMenuView

    // Impl Drop
    set_main_menu_view__drop(main_menu_view__drop)

    // Impl ui::MainMenuView
    set_main_menu_view__get_start_new_game_button(main_menu_view__get_start_new_game_button)
    set_main_menu_view__transition_to_game_view(main_menu_view__transition_to_game_view)

    // Type GameView

    // Impl ui::GameView

    // Impl Drop
    set_game_view__drop(game_view__drop)

    // Impl HasLayoutHandlers
    set_game_view__add_layout_handler(game_view__add_layout_handler)

    // Impl HasDragHandlers
    set_game_view__add_drag_handler(game_view__add_drag_handler)

    // Impl ui::SpriteSource
    set_game_view__create_sprite(game_view__create_sprite)

    // Type SystemView

    // Impl native::SystemView
    set_system_view__get_texture_loader(system_view__get_texture_loader)

    // Impl Drop
    set_system_view__drop(system_view__drop)

    // Type TextureLoader

    // Impl native::TextureLoader
    set_texture_loader__load_texture(texture_loader__load_texture)

    // Impl Drop
    set_texture_loader__drop(texture_loader__drop)

    return ApplicationContext(create_application(
        OpaquePointer(Unmanaged.passRetained(systemView).toOpaque())))
  }

}

// Wrapper classes for types owned by rust

// Type RustString

class RustString {

  private let ref: OpaquePointer?

  init(_ ref: OpaquePointer?) { self.ref = ref }

// Impl Drop
  deinit {
    rust_string__drop(ref)
  }


// Impl 
  func getLength(
      )
          -> Int64 {
    return rust_string__get_length(self.ref
        )
  }
  func getContent(
      )
          -> UnsafeMutablePointer<UInt8>? {
    return rust_string__get_content(self.ref
        )
  }
}

// Type ApplicationContext

class ApplicationContext {

  private let ref: OpaquePointer?

  init(_ ref: OpaquePointer?) { self.ref = ref }

// Impl Drop
  deinit {
    application_context__drop(ref)
  }


// Impl 
  func bindToLoadingView(
      view: LoadingView)
          -> WrappedLoadingPresenter {
    return WrappedLoadingPresenter(application_context__bind_to_loading_view(self.ref
        , OpaquePointer(Unmanaged.passRetained(view).toOpaque())))
  }
  func bindToMainMenuView(
      view: MainMenuView)
          -> WrappedMainMenuPresenter {
    return WrappedMainMenuPresenter(application_context__bind_to_main_menu_view(self.ref
        , OpaquePointer(Unmanaged.passRetained(view).toOpaque())))
  }
  func bindToGameView(
      view: GameView)
          -> WrappedGamePresenter {
    return WrappedGamePresenter(application_context__bind_to_game_view(self.ref
        , OpaquePointer(Unmanaged.passRetained(view).toOpaque())))
  }
}

// Type ClickHandler

class ClickHandler {

  private let ref: OpaquePointer?

  init(_ ref: OpaquePointer?) { self.ref = ref }

// Impl Drop
  deinit {
    click_handler__drop(ref)
  }


// Impl 
  func onClick(
      )
           {
    click_handler__on_click(self.ref
        )
  }
}

// Type DragHandler

class DragHandler {

  private let ref: OpaquePointer?

  init(_ ref: OpaquePointer?) { self.ref = ref }

// Impl Drop
  deinit {
    drag_handler__drop(ref)
  }


// Impl 
  func onDragStart(
      globalX: Float64, globalY: Float64, localX: Float64, localY: Float64)
           {
    drag_handler__on_drag_start(self.ref
        , globalX, globalY, localX, localY)
  }
  func onDragMove(
      globalX: Float64, globalY: Float64, localX: Float64, localY: Float64)
           {
    drag_handler__on_drag_move(self.ref
        , globalX, globalY, localX, localY)
  }
  func onDragEnd(
      globalX: Float64, globalY: Float64, localX: Float64, localY: Float64)
           {
    drag_handler__on_drag_end(self.ref
        , globalX, globalY, localX, localY)
  }
}

// Type LayoutHandler

class LayoutHandler {

  private let ref: OpaquePointer?

  init(_ ref: OpaquePointer?) { self.ref = ref }

// Impl Drop
  deinit {
    layout_handler__drop(ref)
  }


// Impl 
  func onLayout(
      width: Int64, height: Int64)
           {
    layout_handler__on_layout(self.ref
        , width, height)
  }
}

// Type WrappedMainMenuPresenter

class WrappedMainMenuPresenter {

  private let ref: OpaquePointer?

  init(_ ref: OpaquePointer?) { self.ref = ref }

// Impl Drop
  deinit {
    wrapped_main_menu_presenter__drop(ref)
  }

}

// Type WrappedLoadingPresenter

class WrappedLoadingPresenter {

  private let ref: OpaquePointer?

  init(_ ref: OpaquePointer?) { self.ref = ref }

// Impl Drop
  deinit {
    wrapped_loading_presenter__drop(ref)
  }

}

// Type WrappedGamePresenter

class WrappedGamePresenter {

  private let ref: OpaquePointer?

  init(_ ref: OpaquePointer?) { self.ref = ref }

// Impl Drop
  deinit {
    wrapped_game_presenter__drop(ref)
  }

}


// Define the functions for types owned by swift that will be called
// Externally by rust

// Type SwiftString

// Impl Drop

private func swift_string__drop(_self: OpaquePointer?) {
  let _ : SwiftString = Unmanaged.fromOpaque(UnsafeMutableRawPointer(_self!)).takeRetainedValue()
}


// Impl 

private func swift_string__get_length(ref: OpaquePointer?
    )
        -> Int64 {
  let _self : SwiftString = Unmanaged.fromOpaque(UnsafeMutableRawPointer(ref!)).takeUnretainedValue()
  return  _self.length
}

private func swift_string__get_content(ref: OpaquePointer?
    )
        -> UnsafeMutablePointer<UInt8>? {
  let _self : SwiftString = Unmanaged.fromOpaque(UnsafeMutableRawPointer(ref!)).takeUnretainedValue()
  return _self.getContent(
      )


}

// Type HandlerRegistration

// Impl Drop

private func handler_registration__drop(_self: OpaquePointer?) {
  let _ : HandlerRegistration = Unmanaged.fromOpaque(UnsafeMutableRawPointer(_self!)).takeRetainedValue()
}


// Impl ui::HandlerRegistration

private func handler_registration__deregister(ref: OpaquePointer?
    )
        -> Void {
  let _self : HandlerRegistration = Unmanaged.fromOpaque(UnsafeMutableRawPointer(ref!)).takeUnretainedValue()
  _self.deregister(
      )


}

// Type Button

// Impl HasText

private func button__get_text(ref: OpaquePointer?
    )
        -> OpaquePointer? {
  let _self : Button = Unmanaged.fromOpaque(UnsafeMutableRawPointer(ref!)).takeUnretainedValue()
  return OpaquePointer(Unmanaged.passRetained(SwiftString(_self.getText(
      ))).toOpaque())


}

private func button__set_text(ref: OpaquePointer?
    , value: OpaquePointer?)
        -> Void {
  let _self : Button = Unmanaged.fromOpaque(UnsafeMutableRawPointer(ref!)).takeUnretainedValue()
  _self.setText(
      rustStringToString(RustString(value)))


}

// Impl crate::ui::Button

// Impl Drop

private func button__drop(_self: OpaquePointer?) {
  let _ : Button = Unmanaged.fromOpaque(UnsafeMutableRawPointer(_self!)).takeRetainedValue()
}


// Impl HasClickHandlers

private func button__add_click_handler(ref: OpaquePointer?
    , clickHandler: OpaquePointer?)
        -> OpaquePointer? {
  let _self : Button = Unmanaged.fromOpaque(UnsafeMutableRawPointer(ref!)).takeUnretainedValue()
  return OpaquePointer(Unmanaged.passRetained(_self.addClickHandler(
      ClickHandler(clickHandler))).toOpaque())


}

// Type TextArea

// Impl HasText

private func text_area__get_text(ref: OpaquePointer?
    )
        -> OpaquePointer? {
  let _self : TextArea = Unmanaged.fromOpaque(UnsafeMutableRawPointer(ref!)).takeUnretainedValue()
  return  OpaquePointer(Unmanaged.passRetained(SwiftString(_self.text)).toOpaque())
}

private func text_area__set_text(ref: OpaquePointer?
    , value: OpaquePointer?)
        -> Void {
  let _self : TextArea = Unmanaged.fromOpaque(UnsafeMutableRawPointer(ref!)).takeUnretainedValue()
  _self.text = rustStringToString(RustString(value))

}

// Impl Drop

private func text_area__drop(_self: OpaquePointer?) {
  let _ : TextArea = Unmanaged.fromOpaque(UnsafeMutableRawPointer(_self!)).takeRetainedValue()
}


// Type ProgressBar

// Impl HasText

private func progress_bar__get_text(ref: OpaquePointer?
    )
        -> OpaquePointer? {
  let _self : ProgressBar = Unmanaged.fromOpaque(UnsafeMutableRawPointer(ref!)).takeUnretainedValue()
  return OpaquePointer(Unmanaged.passRetained(SwiftString(_self.getText(
      ))).toOpaque())


}

private func progress_bar__set_text(ref: OpaquePointer?
    , value: OpaquePointer?)
        -> Void {
  let _self : ProgressBar = Unmanaged.fromOpaque(UnsafeMutableRawPointer(ref!)).takeUnretainedValue()
  _self.setText(
      rustStringToString(RustString(value)))


}

// Impl Drop

private func progress_bar__drop(_self: OpaquePointer?) {
  let _ : ProgressBar = Unmanaged.fromOpaque(UnsafeMutableRawPointer(_self!)).takeRetainedValue()
}


// Impl HasIntValue

private func progress_bar__get_int_value(ref: OpaquePointer?
    )
        -> Int64 {
  let _self : ProgressBar = Unmanaged.fromOpaque(UnsafeMutableRawPointer(ref!)).takeUnretainedValue()
  return _self.getIntValue(
      )


}

private func progress_bar__set_int_value(ref: OpaquePointer?
    , value: Int64)
        -> Void {
  let _self : ProgressBar = Unmanaged.fromOpaque(UnsafeMutableRawPointer(ref!)).takeUnretainedValue()
  _self.setIntValue(
      value)


}

// Impl ui::ProgressBar

// Type Texture

// Impl Drop

private func texture__drop(_self: OpaquePointer?) {
  let _ : Texture = Unmanaged.fromOpaque(UnsafeMutableRawPointer(_self!)).takeRetainedValue()
}


// Impl native::Texture

private func texture__get_sub_texture(ref: OpaquePointer?
    , left: Int64, top: Int64, width: Int64, height: Int64)
        -> OpaquePointer? {
  let _self : Texture = Unmanaged.fromOpaque(UnsafeMutableRawPointer(ref!)).takeUnretainedValue()
  return OpaquePointer(Unmanaged.passRetained(_self.getSubTexture(
      left, top, width, height)).toOpaque())


}

// Impl HasIntSize

private func texture__get_width(ref: OpaquePointer?
    )
        -> Int64 {
  let _self : Texture = Unmanaged.fromOpaque(UnsafeMutableRawPointer(ref!)).takeUnretainedValue()
  return _self.getWidth(
      )


}

private func texture__get_height(ref: OpaquePointer?
    )
        -> Int64 {
  let _self : Texture = Unmanaged.fromOpaque(UnsafeMutableRawPointer(ref!)).takeUnretainedValue()
  return _self.getHeight(
      )


}

// Type Sprite

// Impl HasMutableLocation

private func sprite__set_location_animated(ref: OpaquePointer?
    , left: Float64, top: Float64, durationSeconds: Float64)
        -> Void {
  let _self : Sprite = Unmanaged.fromOpaque(UnsafeMutableRawPointer(ref!)).takeUnretainedValue()
  _self.setLocationAnimated(
      left, top, durationSeconds)


}

// Impl Drop

private func sprite__drop(_self: OpaquePointer?) {
  let _ : Sprite = Unmanaged.fromOpaque(UnsafeMutableRawPointer(_self!)).takeRetainedValue()
}


// Impl ui::Sprite

private func sprite__set_texture(ref: OpaquePointer?
    , texture: OpaquePointer?)
        -> Void {
  let _self : Sprite = Unmanaged.fromOpaque(UnsafeMutableRawPointer(ref!)).takeUnretainedValue()
  _self.setTexture(
      Unmanaged.fromOpaque(UnsafeRawPointer(texture!)).takeUnretainedValue())


}

private func sprite__propagate_events_to(ref: OpaquePointer?
    , sprite: OpaquePointer?)
        -> Void {
  let _self : Sprite = Unmanaged.fromOpaque(UnsafeMutableRawPointer(ref!)).takeUnretainedValue()
  _self.propagateEventsTo(
      Unmanaged.fromOpaque(UnsafeRawPointer(sprite!)).takeUnretainedValue())


}

private func sprite__remove_from_parent(ref: OpaquePointer?
    )
        -> Void {
  let _self : Sprite = Unmanaged.fromOpaque(UnsafeMutableRawPointer(ref!)).takeUnretainedValue()
  _self.removeFromParent(
      )


}

// Impl HasMutableSize

private func sprite__set_size_animated(ref: OpaquePointer?
    , width: Float64, height: Float64, durationSeconds: Float64)
        -> Void {
  let _self : Sprite = Unmanaged.fromOpaque(UnsafeMutableRawPointer(ref!)).takeUnretainedValue()
  _self.setSizeAnimated(
      width, height, durationSeconds)


}

// Impl HasMutableVisibility

private func sprite__set_visible(ref: OpaquePointer?
    , visible: Bool)
        -> Void {
  let _self : Sprite = Unmanaged.fromOpaque(UnsafeMutableRawPointer(ref!)).takeUnretainedValue()
  _self.setVisible(
      visible)


}

// Impl HasDragHandlers

private func sprite__add_drag_handler(ref: OpaquePointer?
    , dragHandler: OpaquePointer?)
        -> OpaquePointer? {
  let _self : Sprite = Unmanaged.fromOpaque(UnsafeMutableRawPointer(ref!)).takeUnretainedValue()
  return OpaquePointer(Unmanaged.passRetained(_self.addDragHandler(
      DragHandler(dragHandler))).toOpaque())


}

// Type LoadingView

// Impl Drop

private func loading_view__drop(_self: OpaquePointer?) {
  let _ : LoadingView = Unmanaged.fromOpaque(UnsafeMutableRawPointer(_self!)).takeRetainedValue()
}


// Impl ui::LoadingView

private func loading_view__get_progress_indicator(ref: OpaquePointer?
    )
        -> OpaquePointer? {
  let _self : LoadingView = Unmanaged.fromOpaque(UnsafeMutableRawPointer(ref!)).takeUnretainedValue()
  return  OpaquePointer(Unmanaged.passRetained(_self.progressIndicator).toOpaque())
}

private func loading_view__transition_to_main_menu_view(ref: OpaquePointer?
    )
        -> Void {
  let _self : LoadingView = Unmanaged.fromOpaque(UnsafeMutableRawPointer(ref!)).takeUnretainedValue()
  _self.transitionToMainMenuView(
      )


}

// Type MainMenuView

// Impl Drop

private func main_menu_view__drop(_self: OpaquePointer?) {
  let _ : MainMenuView = Unmanaged.fromOpaque(UnsafeMutableRawPointer(_self!)).takeRetainedValue()
}


// Impl ui::MainMenuView

private func main_menu_view__get_start_new_game_button(ref: OpaquePointer?
    )
        -> OpaquePointer? {
  let _self : MainMenuView = Unmanaged.fromOpaque(UnsafeMutableRawPointer(ref!)).takeUnretainedValue()
  return  OpaquePointer(Unmanaged.passRetained(_self.startNewGameButton).toOpaque())
}

private func main_menu_view__transition_to_game_view(ref: OpaquePointer?
    )
        -> Void {
  let _self : MainMenuView = Unmanaged.fromOpaque(UnsafeMutableRawPointer(ref!)).takeUnretainedValue()
  _self.transitionToGameView(
      )


}

// Type GameView

// Impl ui::GameView

// Impl Drop

private func game_view__drop(_self: OpaquePointer?) {
  let _ : GameView = Unmanaged.fromOpaque(UnsafeMutableRawPointer(_self!)).takeRetainedValue()
}


// Impl HasLayoutHandlers

private func game_view__add_layout_handler(ref: OpaquePointer?
    , layoutHandler: OpaquePointer?)
        -> OpaquePointer? {
  let _self : GameView = Unmanaged.fromOpaque(UnsafeMutableRawPointer(ref!)).takeUnretainedValue()
  return OpaquePointer(Unmanaged.passRetained(_self.addLayoutHandler(
      LayoutHandler(layoutHandler))).toOpaque())


}

// Impl HasDragHandlers

private func game_view__add_drag_handler(ref: OpaquePointer?
    , dragHandler: OpaquePointer?)
        -> OpaquePointer? {
  let _self : GameView = Unmanaged.fromOpaque(UnsafeMutableRawPointer(ref!)).takeUnretainedValue()
  return OpaquePointer(Unmanaged.passRetained(_self.addDragHandler(
      DragHandler(dragHandler))).toOpaque())


}

// Impl ui::SpriteSource

private func game_view__create_sprite(ref: OpaquePointer?
    )
        -> OpaquePointer? {
  let _self : GameView = Unmanaged.fromOpaque(UnsafeMutableRawPointer(ref!)).takeUnretainedValue()
  return OpaquePointer(Unmanaged.passRetained(_self.createSprite(
      )).toOpaque())


}

// Type SystemView

// Impl native::SystemView

private func system_view__get_texture_loader(ref: OpaquePointer?
    )
        -> OpaquePointer? {
  let _self : SystemView = Unmanaged.fromOpaque(UnsafeMutableRawPointer(ref!)).takeUnretainedValue()
  return  OpaquePointer(Unmanaged.passRetained(_self.textureLoader).toOpaque())
}

// Impl Drop

private func system_view__drop(_self: OpaquePointer?) {
  let _ : SystemView = Unmanaged.fromOpaque(UnsafeMutableRawPointer(_self!)).takeRetainedValue()
}


// Type TextureLoader

// Impl native::TextureLoader

private func texture_loader__load_texture(ref: OpaquePointer?
    , name: OpaquePointer?)
        -> OpaquePointer? {
  let _self : TextureLoader = Unmanaged.fromOpaque(UnsafeMutableRawPointer(ref!)).takeUnretainedValue()
  return OpaquePointer(Unmanaged.passRetained(_self.loadTexture(
      rustStringToString(RustString(name)))).toOpaque())


}

// Impl Drop

private func texture_loader__drop(_self: OpaquePointer?) {
  let _ : TextureLoader = Unmanaged.fromOpaque(UnsafeMutableRawPointer(_self!)).takeRetainedValue()
}



// Stop-gap functions

private func rustStringToString(_ rustString: RustString) -> String {
    let length = rustString.getLength()
    let content = rustString.getContent()
    let data = Data(bytes: UnsafeRawPointer(content!), count: Int(length))

    return String(data: data, encoding: String.Encoding.utf8)!
}
