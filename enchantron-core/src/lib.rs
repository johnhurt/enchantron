use enchantron::MainMenuPresenter;
use std::sync::{Arc};
use std::os::raw::{c_void};
use std::string::String;
use enchantron::ui::{HasClickHandlers, HasText, Button, MainMenuView};
use enchantron::event::HandlerRegistration;

mod enchantron;

#[repr(C)]
pub struct ext_handler_registration {
    _self: *mut c_void,
    deregister: extern fn(_self: *mut c_void)
}

#[repr(C)]
pub struct ext_click_handler {
    _self: *mut ExternalClickHandler,
    on_click: extern fn(_self: *mut ExternalClickHandler)
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext_has_click_handlers {
    _self: *mut c_void,
    add_click_handler: extern fn(_self: *mut c_void, 
            click_handler: ext_click_handler) 
                    -> ext_handler_registration 
}

#[repr(C)]
pub struct ext_text {
    length: i32,
    content: *const u8
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext_has_text {
    _self: *mut c_void,
    get_text: extern fn(_self: *mut c_void) -> ext_text,
    set_text: extern fn(_self: *mut c_void, text: ext_text)
}

#[repr(C)]
pub struct ext_button {
    click_handlers: ext_has_click_handlers,
    text: ext_has_text
}

#[repr(C)]
pub struct ext_main_menu_view {
    _self: *mut c_void,
    start_game_button: ext_button
}

#[repr(C)]
pub struct ext_main_menu_presenter {
    _self: *mut Arc<MainMenuPresenter<ExternalMainMenuView>>
}

#[no_mangle]
pub extern fn allocate_string(length: i32) -> *const u8 {
    let v : Vec<u8> = Vec::with_capacity(length as usize);
    let result = v.as_ptr();
    std::mem::forget(v);
    result
}

#[no_mangle]
pub extern fn bind_main_menu_view(ext_view: ext_main_menu_view) 
        -> ext_main_menu_presenter {
    let main_menu_view = ExternalMainMenuView {
        raw_main_menu_view: ext_view
    };

    let raw_result = Box::new(enchantron::MainMenuPresenter::new(main_menu_view));

    ext_main_menu_presenter{
        _self: Box::into_raw(raw_result)
    }
}

extern fn handle_click(handler_ptr: *mut ExternalClickHandler) {
    let handler = unsafe { &*handler_ptr };
    (handler.raw_click_handler)()
}

struct ExternalHasClickHandlers {
    raw_has_click_handlers: ext_has_click_handlers
}

struct ExternalHasText {
    raw_has_text: ext_has_text
}

struct ExternalClickHandler {
    raw_click_handler: Box<Fn() + 'static>
}

struct ExternalMainMenuView {
    raw_main_menu_view: ext_main_menu_view
}

fn to_ext_click_handler(click_handler: Box<Fn() + 'static>) 
        -> ext_click_handler {
    
    let boxed_handler = Box::new(ExternalClickHandler {
        raw_click_handler: click_handler
    });

    ext_click_handler {
        _self: Box::into_raw(boxed_handler),
        on_click: handle_click
    }
}

fn to_rust_handler_registration(
        ext_handler: ext_handler_registration) 
                -> HandlerRegistration {
    HandlerRegistration::new(Box::new(move || { 
            (ext_handler.deregister)(ext_handler._self)
    }))
}

impl HasClickHandlers for ExternalHasClickHandlers {
    fn add_click_handler(&mut self, handler: Box<Fn() + 'static>) 
            -> HandlerRegistration {

        to_rust_handler_registration(
                (self.raw_has_click_handlers.add_click_handler)(
                        self.raw_has_click_handlers._self, 
                        to_ext_click_handler(handler)))
    }

}

impl HasText for ExternalHasText {

    fn get_text(&self) -> String {
        let text = (self.raw_has_text.get_text)(
                self.raw_has_text._self);
                
        let vec_content : Vec<u8> = unsafe { 
                Vec::from_raw_parts(
                        text.content as *mut u8, 
                        text.length as usize, 
                        text.length as usize) 
        };
        
        String::from_utf8(vec_content).unwrap()
    }

    fn set_text(&mut self, text: String) {
        let content = text.into_bytes();
        let length = content.len() as i32;

        (self.raw_has_text.set_text)(
                self.raw_has_text._self, ext_text {
                    length: length,
                    content: content.as_ptr()
                })
    }
}

impl MainMenuView for ExternalMainMenuView {
    fn get_start_game_button(&self) -> Button {
        let start_game_button = &self.raw_main_menu_view.start_game_button;

        Button {
            click_handlers: Box::new(ExternalHasClickHandlers {
                raw_has_click_handlers: start_game_button.click_handlers
            }),
            text: Box::new(ExternalHasText {
                raw_has_text: start_game_button.text
            })
        }
    }
}
