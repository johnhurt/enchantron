extern crate cbindgen;
extern crate regex;

use std::env;
use std::fs::{File, OpenOptions, remove_file};
use std::io::prelude::*;
use std::panic;
use std::path::Path;

use regex::Regex;

fn main() {

  let result = panic::catch_unwind(|| {

    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let header_file_name = "enchantron.h";

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_config(cbindgen::Config::from_file("cbindgen.toml").unwrap())
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(header_file_name);

    let mut f = File::open(header_file_name).expect("Header file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Failed to read from header file");
    
    let ignored_type_pattern 
        = Regex::new(r"^typedef struct (\S+) (\S+);$").unwrap();

    let contents_str = contents.into_boxed_str();

    let to_replace : Vec<String> = contents_str.lines()
        .filter_map(|line| {
          if (&ignored_type_pattern).is_match(line) {
            let caps = &ignored_type_pattern.captures(line).unwrap();
            if &caps[1] == &caps[2] {
              Some(caps[1].to_string().clone())
            }
            else {
              None
            }
          }
          else {
            None
          }
        }).collect();

    let result : Vec<String> = contents_str.lines()
        .filter_map(|line| {

          if (&ignored_type_pattern).is_match(line) {
            None
          }
          else {
            let mut mut_line = String::from(line).clone();

            for to_void in &to_replace {
              mut_line = mut_line.replace(to_void, "void");
            }

            Some(String::from(mut_line))
          }
        }).collect();

    let path = Path::new(header_file_name);

    let _ = remove_file(path);

    let _ = File::create(header_file_name);

    let mut options = OpenOptions::new();
    options.write(true);
    let mut writer : File = options.open(path).unwrap();

    for line in &result {
      let _ = writer.write_all(line.as_bytes());
      let _ = writer.write_all("\n".as_bytes());
    }
  });

  if let Err(e) = result {
    if let Some(e) = e.downcast_ref::<&'static str>() {
      println!("Got an error: {}", e);
    } else {
      println!("Got an unknown error: {:?}", e);
    }
  }
  
}
