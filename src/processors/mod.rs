use config::Config;

mod head_cleanup;
mod strip_blank;
mod rebrand;


pub const PROCESSORS: [fn(Config, String) -> Result<String, String>; 3] =
    [head_cleanup::head_cleanup, rebrand::rebrand, strip_blank::strip_blank];
