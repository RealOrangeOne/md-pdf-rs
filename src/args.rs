use clap::{App, AppSettings, ArgMatches};


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
}


pub fn get_matches() -> ArgMatches<'static> {
    return build().get_matches();
}
