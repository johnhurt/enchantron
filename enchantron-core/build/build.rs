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

fn main() {
    // let url = format!("vscode://vadimcn.vscode-lldb/launch/config?{{request:'attach',pid:{}}}", std::process::id());
    // std::process::Command::new("code")
    //     .arg("--open-url")
    //     .arg(url)
    //     .output()
    //     .unwrap();
    // std::thread::sleep_ms(10000);

    CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Info,
        Config::default(),
        File::create("enchantron_build.log").unwrap(),
    )])
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
