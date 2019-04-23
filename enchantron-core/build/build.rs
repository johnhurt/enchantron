#[macro_use] extern crate lazy_static;
#[macro_use] extern crate handlebars;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde;
#[macro_use] extern crate derive_builder;
#[macro_use] extern crate log;

extern crate simplelog;
extern crate cbindgen;
extern crate regex;
extern crate heck;
extern crate itertools;

use simplelog::{
    CombinedLogger,
    TermLogger,
    WriteLogger,
    LevelFilter,
    Config
    };
use std::env;
use std::fs::{File, OpenOptions, remove_file};
use std::io::prelude::*;
use std::panic;
use std::path::Path;
use serde_json::from_str;
use regex::Regex;

use gen::*;

mod gen;

fn main() {
  CombinedLogger::init(vec![
      TermLogger::new(LevelFilter::Info, Config::default()).unwrap(),
      WriteLogger::new(LevelFilter::Info, Config::default(),
          File::create("enchantron_build.log").unwrap())
  ]).expect("Umm logger?");

  let result = panic::catch_unwind(|| {

    generate_swift_bindings();
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let header_file_name = "enchantron.h";

    let config = cbindgen::Config::from_file("cbindgen.toml")
        .unwrap_or_else(|_| { panic!("No toml") });

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_config(config)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(header_file_name);

  });

  if let Err(e) = result {
    if let Some(e) = e.downcast_ref::<&'static str>() {
      error!("Got an error: {}", e);
    } else {
      error!("Got an unknown error: {:?}", e);
    }
  }

}
