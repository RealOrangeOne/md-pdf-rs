#[macro_use]
extern crate clap;

mod args;

fn main() {
    let matches = args::get_matches();
}
