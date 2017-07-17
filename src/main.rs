#[macro_use] extern crate clap;
#[macro_use] extern crate serde_derive;

extern crate serde_yaml;


mod args;
mod config;

#[cfg(test)]
mod tests;

fn main() {
    let args = args::get_matches();
    if args.subcommand_name().unwrap() == "build" {
        config::get_config();
    }
}
