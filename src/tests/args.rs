use args;

use std::error::Error;
use clap::ErrorKind;

#[test]
fn incorrect_subcommand() {
    let out = args::get_matches_for(vec!("mdp"));
    assert!(out.is_err());
    assert_eq!(out.unwrap_err().kind, ErrorKind::MissingArgumentOrSubcommand);

    let out = args::get_matches_for(vec!("mdp", "invalid"));
    assert!(out.is_err());
    assert_eq!(out.unwrap_err().kind, ErrorKind::UnknownArgument);
}

#[test]
fn verbose_number() {
    fn get_verbose_level(arg_list: Vec<&str>) -> u64 {
        return args::get_verbose(args::get_matches_for(arg_list).unwrap());
    }

    assert_eq!(get_verbose_level(vec!("mdp", "build", "-v")), 1);
    assert_eq!(get_verbose_level(vec!("mdp", "build", "-vv")), 2);
    assert_eq!(get_verbose_level(vec!("mdp", "-v", "build", "-vv")), 3);
    assert_eq!(get_verbose_level(vec!("mdp", "-vv", "build", "-v")), 3);
    assert_eq!(get_verbose_level(vec!("mdp", "-v", "build", "-v")), 2);
    assert_eq!(get_verbose_level(vec!("mdp", "-v", "build")), 1);
    assert_eq!(get_verbose_level(vec!("mdp", "--verbose", "build", "-v")), 2);
    assert_eq!(get_verbose_level(vec!("mdp", "-v", "build", "--verbose")), 2);
}


#[test]
fn build_subcommand() {
    let out = args::get_matches_for(vec!("mdp", "build"));
    assert!(out.is_ok());
    assert_eq!(out.unwrap().subcommand_name().unwrap(), "build");
}


#[test]
fn version_string() {
    let out = args::get_matches_for(vec!("mdp", "--version"));
    assert!(out.is_err());
    assert_eq!(out.unwrap_err().kind, ErrorKind::VersionDisplayed);

    let out = args::get_matches_for(vec!("mdp", "build", "--version"));
    assert!(out.is_err());
    assert_eq!(out.unwrap_err().kind, ErrorKind::UnknownArgument);
}
