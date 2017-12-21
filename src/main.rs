#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;

extern crate serde_yaml;
extern crate pandoc;
extern crate wkhtmltopdf;
extern crate sciter;
extern crate zip;
extern crate mktemp;
extern crate tera;
extern crate chrono;

use std::io::{self, Write};
use std::process::exit;

mod args;
mod config;
mod input;
mod utils;
mod build;
mod output;
mod processors;
mod html;
mod assets;
mod templating;

#[cfg(test)]
mod tests;

use clap::ArgMatches;
use input::read_input_files;
use config::Config;
use build::build_input;
use output::output;
use utils::ok_or_exit;


fn build(config: Config) -> Result<(), String> {
    let input = try!(read_input_files(config.absolute_inputs()));
    let raw_html = try!(build_input(config.clone(), input));
    println!("{}", raw_html);
    try!(output(config, raw_html));
    return Ok(());
}

fn get_config(args: ArgMatches) -> Config {
    let mut config = ok_or_exit(config::read::get_config());
    config.verbosity = args::get_verbose(args);
    return config;
}


fn main() {
    let args = args::get_matches();
    let subcommand = args.subcommand_name().expect("subcommand error");
    match subcommand {
        "build" => {
            let config = get_config(args.clone());
            utils::ok_or_exit(build(config.clone()));
        }
        cmd => {
            writeln!(io::stderr(), "Unknown command {}.", cmd).unwrap();
            exit(1);
        }
    }
}
