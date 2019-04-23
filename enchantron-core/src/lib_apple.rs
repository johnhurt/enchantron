#[macro_use]
extern crate lazy_static;

use std::os::raw::c_void;
use std::sync::{Arc,RwLock};

use event::{EventBus};
use native_impl::TextureLoader;
use presenter::{ LoadingPresenter, MainMenuPresenter, GamePresenter };
use ui_impl::{ LoadingView, MainMenuView, GameView, ClickHandler, RustString,
    click_handler_binding, rust_string_binding};

pub mod impl_swift;
pub use impl_swift::*;

mod event;
mod native;
mod presenter;
mod ui;

mod native_apple;
mod ui_apple;


mod ui_impl {
  pub use super::ui_apple::*;
}

mod native_impl {
  pub use super::native_apple::*;
}

lazy_static! {
  static ref UI_BINDINGS : RwLock<Option<ext_ui_binding>> = RwLock::new(None);
  static ref NATIVE_BINDINGS : RwLock<Option<ext_native_binding>> = RwLock::new(None);
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct int_ui_binding {
  pub click_handler: ext_click_handler,
  pub rust_string: ext_rust_string,
  pub main_menu_presenter: ext_main_menu_presenter,
  pub game_presenter: ext_game_presenter
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ext_ui_binding {
  pub loading_view: ext_loading_view,
  pub main_menu_view: ext_main_menu_view,
  pub game_view: ext_game_view,
  pub button: ext_button,
  pub progress_bar: ext_progress_bar,
  pub handler_registration: ext_handler_registration,
  pub texture: ext_texture,
  pub swift_string: ext_swift_string
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ext_native_binding {
  pub texture_loader: ext_texture_loader,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ext_application_context {
  event_bus: *mut Arc<EventBus>,
  texture_loader: *mut Arc<TextureLoader>,
  internal_ui_binding: int_ui_binding,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ext_loading_view {
  pub get_progress_indicator: extern "C" fn(_self: *mut c_void) -> *mut c_void,
  pub destroy: extern "C" fn(_self: *mut c_void)
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ext_main_menu_view {
  pub get_start_game_button: extern "C" fn(_self: *mut c_void) -> *mut c_void,
  pub transition_to_game_view: extern "C" fn(_self: *mut c_void),
  pub destroy: extern "C" fn(_self: *mut c_void)
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ext_handler_registration {
  pub deregister: extern "C" fn(_self: *mut c_void),
  pub destroy: extern "C" fn(_self: *mut c_void)
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ext_click_handler {
  pub on_click: extern "C" fn(_self: *mut ClickHandler),
  pub drop: extern "C" fn (_self: *mut ClickHandler)
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ext_rust_string {
  pub get_length: extern "C" fn(_self: *mut RustString) -> i64,
  pub get_content: extern "C" fn(_self: *mut RustString) -> *mut u8,
  pub drop: extern "C" fn(_self: *mut RustString)
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ext_swift_string {
  pub get_length: extern "C" fn(_self: *mut c_void) -> i64,
  pub get_content: extern "C" fn(_self: *mut c_void) -> *mut u8,
  pub destroy: extern "C" fn(_self: *mut c_void)
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ext_button {
  pub add_click_handler:
      extern "C" fn(
          _self: *mut c_void, 
          click_handler: *mut ClickHandler) 
          -> *mut c_void,
  pub get_text: extern "C" fn(_self: *mut c_void) -> *mut c_void,
  pub set_text: extern "C" fn(_self: *mut c_void, text: *mut RustString),
  pub destroy: extern "C" fn(_self: *mut c_void)
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ext_progress_bar {
  pub get_int_value: extern "C" fn(_self: *mut c_void) -> i64,
  pub set_int_value: extern "C" fn(_self: *mut c_void, value: i64),
  pub get_text: extern "C" fn(_self: *mut c_void) -> *mut c_void,
  pub set_text: extern "C" fn(_self: *mut c_void, text: *mut RustString),
  pub destroy: extern "C" fn(_self: *mut c_void)
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ext_game_view {
  pub get_width: extern "C" fn(_self: *mut c_void) -> i64,
  pub get_height: extern "C" fn(_self: *mut c_void) -> i64,
  pub get_x: extern "C" fn(_self: *mut c_void) -> i64,
  pub get_y: extern "C" fn(_self: *mut c_void) -> i64,
  pub destroy: extern "C" fn(_self: *mut c_void)
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ext_texture {
  pub get_sub_texture:
    extern "C" fn(_self: *mut c_void, left: i64, top: i64, width: i64, height: i64) -> *mut c_void,
  pub get_width: extern "C" fn(_self: *mut c_void) -> i64,
  pub get_height: extern "C" fn(_self: *mut c_void) -> i64,
  pub destroy: extern "C" fn(_self: *mut c_void)
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ext_texture_loader {
  pub load_texture: extern "C" fn(
      _self: *mut c_void, 
      resource_name: *mut RustString) 
      -> *mut c_void,
  pub destroy: extern "C" fn(_self: *mut c_void)
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ext_main_menu_presenter {
  pub drop: extern "C" fn(_self: *mut Arc<MainMenuPresenter<MainMenuView>>)
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ext_game_presenter {
  pub drop: extern "C" fn(_self: *mut Arc<GamePresenter<GameView>>)
}

#[no_mangle]
pub extern "C" fn create_application_context(
    ext_texture_loader: *mut c_void,
    ext_ui: ext_ui_binding,
    ext_native: ext_native_binding) 
    -> ext_application_context {

  let texture_loader = Arc::new(TextureLoader::new(ext_texture_loader));
  
  let mut ui_bindings_impl = UI_BINDINGS.write().unwrap();

  *ui_bindings_impl = Some(ext_ui);

  let mut native_bindings_impl = NATIVE_BINDINGS.write().unwrap();

  *native_bindings_impl = Some(ext_native);

  let event_bus = EventBus::new();

  let result = ext_application_context {
    event_bus: Box::into_raw(Box::new(event_bus)),
    texture_loader: Box::into_raw(Box::new(texture_loader)),
    internal_ui_binding: int_ui_binding {
      click_handler: click_handler_binding,
      rust_string: rust_string_binding,
      main_menu_presenter: ext_main_menu_presenter {
        drop: drop_main_menu_presenter,
      },
      game_presenter: ext_game_presenter {
        drop: drop_game_presenter,
      }
    },
  };

  result
}

#[no_mangle]
pub extern "C" fn bind_loading_view(
    application_context: ext_application_context,
    ext_view: *mut c_void) 
      -> *mut Arc<LoadingPresenter<LoadingView>> {
  
  let event_bus = unsafe { &*(application_context.event_bus) }.clone();

  let view = LoadingView::new(ext_view);

  let result = LoadingPresenter::new(view, event_bus);

  Box::into_raw(Box::new(result))
}

#[no_mangle]
pub extern "C" fn bind_main_menu_view(
    application_context: ext_application_context,
    ext_view: *mut c_void) 
    -> *mut Arc<MainMenuPresenter<MainMenuView>> {

  let event_bus = unsafe { &*(application_context.event_bus) }.clone();

  let main_menu_view = MainMenuView::new(ext_view);

  let result = MainMenuPresenter::new(main_menu_view, event_bus);

  Box::into_raw(Box::new(result))
}

#[no_mangle]
pub extern "C" fn bind_game_view(
    application_context: ext_application_context,
    ext_view: *mut c_void) 
    -> *mut Arc<GamePresenter<GameView>> {
  let event_bus = unsafe { &*(application_context.event_bus) }.clone();

  let game_view = GameView::new(ext_view);

  let result = GamePresenter::new(game_view, event_bus);

  Box::into_raw(Box::new(result))
}

extern "C" fn drop_main_menu_presenter(
    p: *mut Arc<MainMenuPresenter<MainMenuView>>) {
  let _ = unsafe { Box::from_raw(p) };
}

extern "C" fn drop_game_presenter(
    p: *mut Arc<GamePresenter<GameView>>) {
  let _ = unsafe { Box::from_raw(p) };
}

pub(crate) fn get_ui_bindings() -> ext_ui_binding {
  
  if let Ok(opt_bind) = UI_BINDINGS.read() {
    if let Some(result) = *opt_bind {
      result
    }
    else {
      panic!("No ui bindings have been set for the current application context")
    }
  }
  else {
    panic!("Failed to acquire read lock on external ui bindings")
  }
}

pub(crate) fn get_native_bindings() -> ext_native_binding {
  
  if let Ok(opt_bind) = NATIVE_BINDINGS.read() {
    if let Some(result) = *opt_bind {
      result
    }
    else {
      panic!("No native bindings have been set for the current application context")
    }
  }
  else {
    panic!("Failed to acquire read lock on external native bindings")
  }
}

#[no_mangle]
pub extern "C" fn bind(in_bindings: bin_shared_binding) -> rust_shared_binding {
  panic!("blah")
}