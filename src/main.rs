#[macro_use]
extern crate clap;

mod args;

#[cfg(test)]
mod tests;

fn main() {
    let matches = args::get_matches();
}
