
// This is an automatically generated file that lets a swift application
// bind to the rust library with a single method call

import Foundation

class RustBinder {
  // Bind all the swift functions into rust
  class func bindToRust() -> ApplicationContext {

    // Type SwiftString

    // Impl Drop
    set_swift_string__drop(swift_string__drop)

    // Impl 
    set_swift_string__get_length(swift_string__get_length)
    set_swift_string__get_content(swift_string__get_content)

    // Type HandlerRegistration

    // Impl ui::HandlerRegistration
    set_handler_registration__deregister(handler_registration__deregister)

    // Impl Drop
    set_handler_registration__drop(handler_registration__drop)

    // Type Button

    // Impl HasText
    set_button__get_text(button__get_text)
    set_button__set_text(button__set_text)

    // Impl HasClickHandlers
    set_button__add_click_handler(button__add_click_handler)

    // Impl ui::Button

    // Impl Drop
    set_button__drop(button__drop)

    // Type TextArea

    // Impl Drop
    set_text_area__drop(text_area__drop)

    // Impl HasText
    set_text_area__get_text(text_area__get_text)
    set_text_area__set_text(text_area__set_text)

    // Type MainMenuView

    // Impl ui::MainMenuView
    set_main_menu_view__get_start_new_game_button(main_menu_view__get_start_new_game_button)
    set_main_menu_view__transition_to_game_view(main_menu_view__transition_to_game_view)

    // Impl Drop
    set_main_menu_view__drop(main_menu_view__drop)

    return ApplicationContext(create_application())
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


// Define the functions for types owned by swift that will be called
// Externally by rust

// Type SwiftString

// Impl Drop

private func swift_string__drop(_self: UnsafeMutableRawPointer?) {
  let _ : SwiftString = Unmanaged.fromOpaque(_self!).takeRetainedValue()
}


// Impl 

private func swift_string__get_length(ref: UnsafeMutableRawPointer?
    )
        -> Int64 {
  let _self : SwiftString = Unmanaged.fromOpaque(ref!).takeUnretainedValue()
  return  _self.length
}

private func swift_string__get_content(ref: UnsafeMutableRawPointer?
    )
        -> UnsafeMutablePointer<UInt8>? {
  let _self : SwiftString = Unmanaged.fromOpaque(ref!).takeUnretainedValue()
  return _self.getContent(
      )


}

// Type HandlerRegistration

// Impl ui::HandlerRegistration

private func handler_registration__deregister(ref: UnsafeMutableRawPointer?
    )
        -> Void {
  let _self : HandlerRegistration = Unmanaged.fromOpaque(ref!).takeUnretainedValue()
  _self.deregister(
      )


}

// Impl Drop

private func handler_registration__drop(_self: UnsafeMutableRawPointer?) {
  let _ : HandlerRegistration = Unmanaged.fromOpaque(_self!).takeRetainedValue()
}


// Type Button

// Impl HasText

private func button__get_text(ref: UnsafeMutableRawPointer?
    )
        -> UnsafeMutableRawPointer? {
  let _self : Button = Unmanaged.fromOpaque(ref!).takeUnretainedValue()
  return UnsafeMutableRawPointer(Unmanaged.passRetained(SwiftString(_self.getText(
      ))).toOpaque())


}

private func button__set_text(ref: UnsafeMutableRawPointer?
    , value: OpaquePointer?)
        -> Void {
  let _self : Button = Unmanaged.fromOpaque(ref!).takeUnretainedValue()
  _self.setText(
      rustStringToString(RustString(value)))


}

// Impl HasClickHandlers

private func button__add_click_handler(ref: UnsafeMutableRawPointer?
    , clickHandler: OpaquePointer?)
        -> UnsafeMutableRawPointer? {
  let _self : Button = Unmanaged.fromOpaque(ref!).takeUnretainedValue()
  return UnsafeMutableRawPointer(Unmanaged.passRetained(_self.addClickHandler(
      ClickHandler(clickHandler))).toOpaque())


}

// Impl ui::Button

// Impl Drop

private func button__drop(_self: UnsafeMutableRawPointer?) {
  let _ : Button = Unmanaged.fromOpaque(_self!).takeRetainedValue()
}


// Type TextArea

// Impl Drop

private func text_area__drop(_self: UnsafeMutableRawPointer?) {
  let _ : TextArea = Unmanaged.fromOpaque(_self!).takeRetainedValue()
}


// Impl HasText

private func text_area__get_text(ref: UnsafeMutableRawPointer?
    )
        -> UnsafeMutableRawPointer? {
  let _self : TextArea = Unmanaged.fromOpaque(ref!).takeUnretainedValue()
  return  UnsafeMutableRawPointer(Unmanaged.passRetained(SwiftString(_self.text)).toOpaque())
}

private func text_area__set_text(ref: UnsafeMutableRawPointer?
    , value: OpaquePointer?)
        -> Void {
  let _self : TextArea = Unmanaged.fromOpaque(ref!).takeUnretainedValue()
  _self.text = rustStringToString(RustString(value))

}

// Type MainMenuView

// Impl ui::MainMenuView

private func main_menu_view__get_start_new_game_button(ref: UnsafeMutableRawPointer?
    )
        -> UnsafeMutableRawPointer? {
  let _self : MainMenuView = Unmanaged.fromOpaque(ref!).takeUnretainedValue()
  return  UnsafeMutableRawPointer(Unmanaged.passRetained(_self.startNewGameButton).toOpaque())
}

private func main_menu_view__transition_to_game_view(ref: UnsafeMutableRawPointer?
    )
        -> Void {
  let _self : MainMenuView = Unmanaged.fromOpaque(ref!).takeUnretainedValue()
  _self.transitionToGameView(
      )


}

// Impl Drop

private func main_menu_view__drop(_self: UnsafeMutableRawPointer?) {
  let _ : MainMenuView = Unmanaged.fromOpaque(_self!).takeRetainedValue()
}



// Stop-gap functions

private func rustStringToString(_ rustString: RustString) -> String {
    let length = rustString.getLength()
    let content = rustString.getContent()
    let data = Data(bytes: UnsafeRawPointer(content!), count: Int(length))

    return String(data: data, encoding: String.Encoding.utf8)!
}
