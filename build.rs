extern crate include_dir;

use std::env;
use std::path::Path;
use include_dir::include_dir;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("assets.rs");

    include_dir("assets").as_variable("SRC").to_file(dest_path).unwrap();
}
