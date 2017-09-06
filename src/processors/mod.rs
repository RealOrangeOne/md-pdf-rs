use config::Config;

pub mod html_cleanup;
pub mod strip_blank;


pub const PROCESSORS: [fn(Config, String) -> Result<String, String>; 2] =
    [html_cleanup::html_cleanup, strip_blank::strip_blank];
