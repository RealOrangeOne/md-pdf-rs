#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;

extern crate serde_yaml;

mod args;
mod config;
mod process;
mod input;
mod utils;

#[cfg(test)]
mod tests;

fn main() {
    let args = args::get_matches();
    if args.subcommand_name().unwrap() == "build" {
        let mut config = config::get_config().expect("Config error");
        config.verbosity = args::get_verbose(args);
        process::build(config);
    }
}
