use clap::{App, AppSettings, ArgMatches, Arg};

#[cfg(test)]
use clap::Result;


fn build() -> App<'static, 'static> {
    return App::new(crate_name!())
        .bin_name("mdp")
        .about(crate_description!())
        .version(crate_version!())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .global_setting(AppSettings::VersionlessSubcommands)
        .global_setting(AppSettings::ColoredHelp)
        .global_setting(AppSettings::GlobalVersion)
        .global_setting(AppSettings::StrictUtf8)
        .arg(Arg::with_name("verbose")
            .global(true)
            .short("v")
            .long("verbose")
            .help("Show verbose output")
            .multiple(true)
        )
}


pub fn get_matches() -> ArgMatches<'static> {
    return build().get_matches();
}

#[cfg(test)]
pub fn get_matches_for(args : Vec<String>) -> Result<ArgMatches<'static>> {
    return build().get_matches_from_safe(args);
}

pub fn get_verbose(m : ArgMatches) -> u64 {
    let sub = m.subcommand_matches(&m.subcommand_name().unwrap()).unwrap();
    m.occurrences_of("verbose") + sub.occurrences_of("verbose")
}
