extern crate include_dir;
extern crate rsass;

use std::env;
use std::path::Path;
use include_dir::include_dir;
use rsass::{OutputStyle, compile_scss};
use std::fs::File;
use std::io::{Read, Write};

const ASSETS_DIR: &str = "assets";
const ASSET_KEY: &str = "SRC";
const STATIC_DIR: &str = "static";

fn build_stylesheet() {
    let mut in_file =
        File::open(Path::new(STATIC_DIR).join("style.scss")).expect("Failed to open scss file");
    let mut buffer = Vec::new();
    in_file.read_to_end(&mut buffer).expect("Failed to read scss");
    let compiled =
        compile_scss(buffer.as_slice(), OutputStyle::Compressed).expect("SCSS compile failed");
    let mut out_file =
        File::create(Path::new(ASSETS_DIR).join("style.css")).expect("Failed to open css file");
    out_file.write_all(&mut compiled.as_slice()).expect("Failed to write css to file");
}

fn embed_assets() {
    let out_dir = env::var("OUT_DIR").expect("Missing output directory variable");
    let dest_path = Path::new(&out_dir).join("assets.rs");
    include_dir(ASSETS_DIR).as_variable(ASSET_KEY).to_file(dest_path).unwrap();
}

fn main() {
    build_stylesheet();
    embed_assets();
}
