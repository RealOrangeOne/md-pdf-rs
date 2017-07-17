#[macro_use] extern crate clap;
#[macro_use] extern crate serde_derive;

extern crate serde_yaml;


mod args;
mod config;

#[cfg(test)]
mod tests;

fn main() {
    config::get_config();
}
