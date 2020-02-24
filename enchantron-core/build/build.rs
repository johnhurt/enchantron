#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate log;

use simplelog::{
    CombinedLogger, Config, LevelFilter, TermLogger, TerminalMode, WriteLogger,
};
use std::fs::File;
use std::panic;

use gen::*;

mod gen;

#[cfg(test)]
fn main() {}

#[cfg(not(test))]
fn main() {
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::default(),
        )
        .unwrap(),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create("enchantron_build.log").unwrap(),
        ),
    ])
    .expect("Umm logger?");

    info!("Running build script");

    let result = panic::catch_unwind(|| {
        info!("Running build script");

        generate_swift_bindings();
    });

    if let Err(e) = result {
        if let Some(e) = e.downcast_ref::<&'static str>() {
            error!("Got an error: {}", e);
        } else {
            error!("Got an unknown error: {:?}", e);
        }
    }

    info!("build.rs done");
}
