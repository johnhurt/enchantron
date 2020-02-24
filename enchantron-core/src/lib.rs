#[macro_use]
extern crate log;
#[macro_use]
extern crate getset;

cfg_if::cfg_if! {
    if #[cfg(test)] {
        #[macro_use]
        extern crate mockall;
    }
}

pub use self::lib_gen::*;

mod util;

#[macro_use]
pub(crate) mod ui;

#[macro_use]
mod model;

mod game;

mod application_context;
mod event;
mod img;
mod native;
mod presenter;
mod view;

mod view_types;

mod lib_gen;
