#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;

extern crate serde_yaml;


mod args;
mod config;
mod process;
mod input;

#[cfg(test)]
mod tests;

fn main() {
    let args = args::get_matches();
    if args.subcommand_name().unwrap() == "build" {
        let config = config::get_config();
        process::build(config);
    }
}
