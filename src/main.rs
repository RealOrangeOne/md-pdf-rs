#[macro_use]
extern crate clap;

mod args;

#[cfg(test)]
mod tests;

fn main() {
    args::get_matches();
}
