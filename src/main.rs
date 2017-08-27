#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;

extern crate serde_yaml;
extern crate pandoc;
extern crate wkhtmltopdf;

use std::io::{self, Write};
use std::process::exit;

mod args;
mod config;
mod process;
mod input;
mod utils;
mod build;
mod output;

#[cfg(test)]
mod tests;

use clap::ArgMatches;
use config::Config;

fn ok_or_exit<T>(res: Result<T, String>) -> T {
    return match res {
        Ok(k) => k,
        Err(err) => {
            writeln!(io::stderr(), "Error: {:?}", err).unwrap();
            exit(1);
        }
    };
}

fn get_config(args: ArgMatches) -> Config {
    let mut config = ok_or_exit(config::get_config());
    config.verbosity = args::get_verbose(args);
    return config;
}


fn main() {
    let args = args::get_matches();
    let subcommand = args.subcommand_name().expect("subcommand error");

    match subcommand {
        "build" => {
            let config = get_config(args.clone());
            ok_or_exit(process::build(config));
        }
        cmd => {
            writeln!(io::stderr(), "Unknown command {}.", cmd).unwrap();
            exit(1);
        }
    }
}
