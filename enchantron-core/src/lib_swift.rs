#[macro_use]
extern crate lazy_static;

use std::sync::RwLock;

pub use swift_application_context::ApplicationContext;


use std::os::raw::c_void;
use ui::ClickHandler;
use ui::HasClickHandlers;
use ui::HasText;
use util::RustString;

#[macro_use]
mod ui;

mod event;
mod presenter;
mod swift_application_context;
mod util;

lazy_static! {
  static ref BINDINGS : RwLock<Bindings> = RwLock::new(Bindings::default());
}

fn with_mutable_bindings(to_call: Box<Fn(&mut Bindings)>) {
  if let Ok(mut bindings) = BINDINGS.write() {
    to_call(&mut *bindings)
  }
  else {
    panic!("Failed to acquire write lock on bindings object")
  }
}

#[no_mangle]
pub extern "C" fn create_application() -> *mut ApplicationContext {
  Box::into_raw(Box::new(ApplicationContext::new()))
}

// Rust structure containing all the methods bound to non-rust-owned types
#[derive(Default)]
#[allow(non_snake_case)]
struct Bindings {

  swift_string__drop: Option<extern "C" fn(_self: *mut c_void
      )
          >,

  swift_string__get_length: Option<extern "C" fn(_self: *mut c_void
      )
          -> i64>,

  swift_string__get_content: Option<extern "C" fn(_self: *mut c_void
      )
          -> *mut u8>,

  handler_registration__deregister: Option<extern "C" fn(_self: *mut c_void
      )
          >,

  handler_registration__drop: Option<extern "C" fn(_self: *mut c_void
      )
          >,

  button__get_text: Option<extern "C" fn(_self: *mut c_void
      )
          -> *mut c_void>,

  button__set_text: Option<extern "C" fn(_self: *mut c_void
      , value: *mut RustString)
          >,

  button__add_click_handler: Option<extern "C" fn(_self: *mut c_void
      , click_handler: *mut ClickHandler)
          -> *mut c_void>,

  button__drop: Option<extern "C" fn(_self: *mut c_void
      )
          >,

  text_area__drop: Option<extern "C" fn(_self: *mut c_void
      )
          >,

  text_area__get_text: Option<extern "C" fn(_self: *mut c_void
      )
          -> *mut c_void>,

  text_area__set_text: Option<extern "C" fn(_self: *mut c_void
      , value: *mut RustString)
          >,

  main_menu_view__get_start_new_game_button: Option<extern "C" fn(_self: *mut c_void
      )
          -> *mut c_void>,

  main_menu_view__transition_to_game_view: Option<extern "C" fn(_self: *mut c_void
      )
          >,

  main_menu_view__drop: Option<extern "C" fn(_self: *mut c_void
      )
          >,
}

// All the functions for types owned by rust
// Type RustString

// Impl Drop

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn rust_string__drop
(_self: *mut RustString
        ) {
  let _ = unsafe { Box::from_raw(_self) };
}
// Impl 

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn rust_string__get_length
(_self: *mut RustString
        ) -> i64 {
  let s = unsafe { &*_self };
  s.get_length()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn rust_string__get_content
(_self: *mut RustString
        ) -> *mut u8 {
  let s = unsafe { &*_self };
  s.get_content()
}
// Type ApplicationContext

// Impl Drop

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn application_context__drop
(_self: *mut ApplicationContext
        ) {
  let _ = unsafe { Box::from_raw(_self) };
}
// Type ClickHandler

// Impl Drop

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn click_handler__drop
(_self: *mut ClickHandler
        ) {
  let _ = unsafe { Box::from_raw(_self) };
}
// Impl 

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn click_handler__on_click
(_self: *mut ClickHandler
        ) {
  let s = unsafe { &*_self };
  s.on_click()
}

// All the structs and impls for types not owned by rust

// Type SwiftString

pub struct SwiftString(*mut c_void);
#[allow(non_snake_case)]
impl Drop for SwiftString {
  fn drop(
      &mut self
      )
           {
    
    (SWIFT_STRING__DROP)(self.0
        )
  }
}

#[allow(non_snake_case)]
impl SwiftString {
  pub fn get_length(
      & self
      )
          -> i64 {
    
    (SWIFT_STRING__GET_LENGTH)(self.0
        )
  }
  pub fn get_content(
      & self
      )
          -> *mut u8 {
    
    (SWIFT_STRING__GET_CONTENT)(self.0
        )
  }
}


// Type HandlerRegistration

pub struct HandlerRegistration(*mut c_void);
#[allow(non_snake_case)]
impl ui::HandlerRegistration for HandlerRegistration {
  fn deregister(
      & self
      )
           {
    
    (HANDLER_REGISTRATION__DEREGISTER)(self.0
        )
  }
}

#[allow(non_snake_case)]
impl Drop for HandlerRegistration {
  fn drop(
      &mut self
      )
           {
    ui::HandlerRegistration::deregister(self);
    (HANDLER_REGISTRATION__DROP)(self.0
        )
  }
}


// Type Button

pub struct Button(*mut c_void);
#[allow(non_snake_case)]
impl HasText for Button {
  fn get_text(
      & self
      )
          -> String {
    
    SwiftString((BUTTON__GET_TEXT)(self.0
        )).to_string()
  }
  fn set_text(
      & self
      , value: String)
           {
    
    (BUTTON__SET_TEXT)(self.0
        , Box::into_raw(Box::new(RustString::new(value))))
  }
}

#[allow(non_snake_case)]
impl HasClickHandlers for Button {
  type R = HandlerRegistration;
  fn add_click_handler(
      & self
      , click_handler: ClickHandler)
          -> Self::R {
    
    HandlerRegistration((BUTTON__ADD_CLICK_HANDLER)(self.0
        , Box::into_raw(Box::new(click_handler))))
  }
}

#[allow(non_snake_case)]
impl ui::Button for Button {
}

#[allow(non_snake_case)]
impl Drop for Button {
  fn drop(
      &mut self
      )
           {
    
    (BUTTON__DROP)(self.0
        )
  }
}


// Type TextArea

pub struct TextArea(*mut c_void);
#[allow(non_snake_case)]
impl Drop for TextArea {
  fn drop(
      &mut self
      )
           {
    
    (TEXT_AREA__DROP)(self.0
        )
  }
}

#[allow(non_snake_case)]
impl HasText for TextArea {
  fn get_text(
      & self
      )
          -> String {
    
    SwiftString((TEXT_AREA__GET_TEXT)(self.0
        )).to_string()
  }
  fn set_text(
      & self
      , value: String)
           {
    
    (TEXT_AREA__SET_TEXT)(self.0
        , Box::into_raw(Box::new(RustString::new(value))))
  }
}


// Type MainMenuView

pub struct MainMenuView(*mut c_void);
#[allow(non_snake_case)]
impl ui::MainMenuView for MainMenuView {
  type B = Button;
  fn get_start_new_game_button(
      & self
      )
          -> Self::B {
    
    Button((MAIN_MENU_VIEW__GET_START_NEW_GAME_BUTTON)(self.0
        ))
  }
  fn transition_to_game_view(
      & self
      )
           {
    
    (MAIN_MENU_VIEW__TRANSITION_TO_GAME_VIEW)(self.0
        )
  }
}

#[allow(non_snake_case)]
impl Drop for MainMenuView {
  fn drop(
      &mut self
      )
           {
    
    (MAIN_MENU_VIEW__DROP)(self.0
        )
  }
}


// All the externalized methods for all types plus their binding code
// RustString
// Drop
// 
// SwiftString
// Drop

#[allow(non_snake_case)]
lazy_static!{
  static ref SWIFT_STRING__DROP : extern "C" fn(_self: *mut c_void
      )
          
            = BINDINGS.read()
                .expect("unable to read from external bindings")
                .swift_string__drop
                .expect("swift_string__drop has not been set");
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn set_swift_string__drop(binding_fn: extern "C" fn(_self: *mut c_void
    )
        ) {
  with_mutable_bindings(Box::new(move |bindings| {
    if let Some(_) = bindings.swift_string__drop {
      panic!("swift_string__drop is already bound")
    }
    bindings.swift_string__drop = Some(binding_fn);
  }));
}
// 

#[allow(non_snake_case)]
lazy_static!{
  static ref SWIFT_STRING__GET_LENGTH : extern "C" fn(_self: *mut c_void
      )
          -> i64
            = BINDINGS.read()
                .expect("unable to read from external bindings")
                .swift_string__get_length
                .expect("swift_string__get_length has not been set");
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn set_swift_string__get_length(binding_fn: extern "C" fn(_self: *mut c_void
    )
        -> i64) {
  with_mutable_bindings(Box::new(move |bindings| {
    if let Some(_) = bindings.swift_string__get_length {
      panic!("swift_string__get_length is already bound")
    }
    bindings.swift_string__get_length = Some(binding_fn);
  }));
}

#[allow(non_snake_case)]
lazy_static!{
  static ref SWIFT_STRING__GET_CONTENT : extern "C" fn(_self: *mut c_void
      )
          -> *mut u8
            = BINDINGS.read()
                .expect("unable to read from external bindings")
                .swift_string__get_content
                .expect("swift_string__get_content has not been set");
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn set_swift_string__get_content(binding_fn: extern "C" fn(_self: *mut c_void
    )
        -> *mut u8) {
  with_mutable_bindings(Box::new(move |bindings| {
    if let Some(_) = bindings.swift_string__get_content {
      panic!("swift_string__get_content is already bound")
    }
    bindings.swift_string__get_content = Some(binding_fn);
  }));
}
// ApplicationContext
// Drop
// HandlerRegistration
// ui::HandlerRegistration

#[allow(non_snake_case)]
lazy_static!{
  static ref HANDLER_REGISTRATION__DEREGISTER : extern "C" fn(_self: *mut c_void
      )
          
            = BINDINGS.read()
                .expect("unable to read from external bindings")
                .handler_registration__deregister
                .expect("handler_registration__deregister has not been set");
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn set_handler_registration__deregister(binding_fn: extern "C" fn(_self: *mut c_void
    )
        ) {
  with_mutable_bindings(Box::new(move |bindings| {
    if let Some(_) = bindings.handler_registration__deregister {
      panic!("handler_registration__deregister is already bound")
    }
    bindings.handler_registration__deregister = Some(binding_fn);
  }));
}
// Drop

#[allow(non_snake_case)]
lazy_static!{
  static ref HANDLER_REGISTRATION__DROP : extern "C" fn(_self: *mut c_void
      )
          
            = BINDINGS.read()
                .expect("unable to read from external bindings")
                .handler_registration__drop
                .expect("handler_registration__drop has not been set");
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn set_handler_registration__drop(binding_fn: extern "C" fn(_self: *mut c_void
    )
        ) {
  with_mutable_bindings(Box::new(move |bindings| {
    if let Some(_) = bindings.handler_registration__drop {
      panic!("handler_registration__drop is already bound")
    }
    bindings.handler_registration__drop = Some(binding_fn);
  }));
}
// ClickHandler
// Drop
// 
// Button
// HasText

#[allow(non_snake_case)]
lazy_static!{
  static ref BUTTON__GET_TEXT : extern "C" fn(_self: *mut c_void
      )
          -> *mut c_void
            = BINDINGS.read()
                .expect("unable to read from external bindings")
                .button__get_text
                .expect("button__get_text has not been set");
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn set_button__get_text(binding_fn: extern "C" fn(_self: *mut c_void
    )
        -> *mut c_void) {
  with_mutable_bindings(Box::new(move |bindings| {
    if let Some(_) = bindings.button__get_text {
      panic!("button__get_text is already bound")
    }
    bindings.button__get_text = Some(binding_fn);
  }));
}

#[allow(non_snake_case)]
lazy_static!{
  static ref BUTTON__SET_TEXT : extern "C" fn(_self: *mut c_void
      , value: *mut RustString)
          
            = BINDINGS.read()
                .expect("unable to read from external bindings")
                .button__set_text
                .expect("button__set_text has not been set");
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn set_button__set_text(binding_fn: extern "C" fn(_self: *mut c_void
    , value: *mut RustString)
        ) {
  with_mutable_bindings(Box::new(move |bindings| {
    if let Some(_) = bindings.button__set_text {
      panic!("button__set_text is already bound")
    }
    bindings.button__set_text = Some(binding_fn);
  }));
}
// HasClickHandlers

#[allow(non_snake_case)]
lazy_static!{
  static ref BUTTON__ADD_CLICK_HANDLER : extern "C" fn(_self: *mut c_void
      , click_handler: *mut ClickHandler)
          -> *mut c_void
            = BINDINGS.read()
                .expect("unable to read from external bindings")
                .button__add_click_handler
                .expect("button__add_click_handler has not been set");
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn set_button__add_click_handler(binding_fn: extern "C" fn(_self: *mut c_void
    , click_handler: *mut ClickHandler)
        -> *mut c_void) {
  with_mutable_bindings(Box::new(move |bindings| {
    if let Some(_) = bindings.button__add_click_handler {
      panic!("button__add_click_handler is already bound")
    }
    bindings.button__add_click_handler = Some(binding_fn);
  }));
}
// ui::Button
// Drop

#[allow(non_snake_case)]
lazy_static!{
  static ref BUTTON__DROP : extern "C" fn(_self: *mut c_void
      )
          
            = BINDINGS.read()
                .expect("unable to read from external bindings")
                .button__drop
                .expect("button__drop has not been set");
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn set_button__drop(binding_fn: extern "C" fn(_self: *mut c_void
    )
        ) {
  with_mutable_bindings(Box::new(move |bindings| {
    if let Some(_) = bindings.button__drop {
      panic!("button__drop is already bound")
    }
    bindings.button__drop = Some(binding_fn);
  }));
}
// TextArea
// Drop

#[allow(non_snake_case)]
lazy_static!{
  static ref TEXT_AREA__DROP : extern "C" fn(_self: *mut c_void
      )
          
            = BINDINGS.read()
                .expect("unable to read from external bindings")
                .text_area__drop
                .expect("text_area__drop has not been set");
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn set_text_area__drop(binding_fn: extern "C" fn(_self: *mut c_void
    )
        ) {
  with_mutable_bindings(Box::new(move |bindings| {
    if let Some(_) = bindings.text_area__drop {
      panic!("text_area__drop is already bound")
    }
    bindings.text_area__drop = Some(binding_fn);
  }));
}
// HasText

#[allow(non_snake_case)]
lazy_static!{
  static ref TEXT_AREA__GET_TEXT : extern "C" fn(_self: *mut c_void
      )
          -> *mut c_void
            = BINDINGS.read()
                .expect("unable to read from external bindings")
                .text_area__get_text
                .expect("text_area__get_text has not been set");
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn set_text_area__get_text(binding_fn: extern "C" fn(_self: *mut c_void
    )
        -> *mut c_void) {
  with_mutable_bindings(Box::new(move |bindings| {
    if let Some(_) = bindings.text_area__get_text {
      panic!("text_area__get_text is already bound")
    }
    bindings.text_area__get_text = Some(binding_fn);
  }));
}

#[allow(non_snake_case)]
lazy_static!{
  static ref TEXT_AREA__SET_TEXT : extern "C" fn(_self: *mut c_void
      , value: *mut RustString)
          
            = BINDINGS.read()
                .expect("unable to read from external bindings")
                .text_area__set_text
                .expect("text_area__set_text has not been set");
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn set_text_area__set_text(binding_fn: extern "C" fn(_self: *mut c_void
    , value: *mut RustString)
        ) {
  with_mutable_bindings(Box::new(move |bindings| {
    if let Some(_) = bindings.text_area__set_text {
      panic!("text_area__set_text is already bound")
    }
    bindings.text_area__set_text = Some(binding_fn);
  }));
}
// MainMenuView
// ui::MainMenuView

#[allow(non_snake_case)]
lazy_static!{
  static ref MAIN_MENU_VIEW__GET_START_NEW_GAME_BUTTON : extern "C" fn(_self: *mut c_void
      )
          -> *mut c_void
            = BINDINGS.read()
                .expect("unable to read from external bindings")
                .main_menu_view__get_start_new_game_button
                .expect("main_menu_view__get_start_new_game_button has not been set");
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn set_main_menu_view__get_start_new_game_button(binding_fn: extern "C" fn(_self: *mut c_void
    )
        -> *mut c_void) {
  with_mutable_bindings(Box::new(move |bindings| {
    if let Some(_) = bindings.main_menu_view__get_start_new_game_button {
      panic!("main_menu_view__get_start_new_game_button is already bound")
    }
    bindings.main_menu_view__get_start_new_game_button = Some(binding_fn);
  }));
}

#[allow(non_snake_case)]
lazy_static!{
  static ref MAIN_MENU_VIEW__TRANSITION_TO_GAME_VIEW : extern "C" fn(_self: *mut c_void
      )
          
            = BINDINGS.read()
                .expect("unable to read from external bindings")
                .main_menu_view__transition_to_game_view
                .expect("main_menu_view__transition_to_game_view has not been set");
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn set_main_menu_view__transition_to_game_view(binding_fn: extern "C" fn(_self: *mut c_void
    )
        ) {
  with_mutable_bindings(Box::new(move |bindings| {
    if let Some(_) = bindings.main_menu_view__transition_to_game_view {
      panic!("main_menu_view__transition_to_game_view is already bound")
    }
    bindings.main_menu_view__transition_to_game_view = Some(binding_fn);
  }));
}
// Drop

#[allow(non_snake_case)]
lazy_static!{
  static ref MAIN_MENU_VIEW__DROP : extern "C" fn(_self: *mut c_void
      )
          
            = BINDINGS.read()
                .expect("unable to read from external bindings")
                .main_menu_view__drop
                .expect("main_menu_view__drop has not been set");
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn set_main_menu_view__drop(binding_fn: extern "C" fn(_self: *mut c_void
    )
        ) {
  with_mutable_bindings(Box::new(move |bindings| {
    if let Some(_) = bindings.main_menu_view__drop {
      panic!("main_menu_view__drop is already bound")
    }
    bindings.main_menu_view__drop = Some(binding_fn);
  }));
}

// Additional stop-gap code

impl ToString for SwiftString {
  fn to_string(&self) -> String {
    let length = self.get_length() as usize;
    let mut vec_data : Vec<u8> = Vec::with_capacity(length);
    unsafe {
      vec_data.set_len(length);
      self.get_content().copy_to_nonoverlapping(
          vec_data.as_mut_ptr(),
          length);
    }

    String::from_utf8(vec_data).unwrap()
  }
}