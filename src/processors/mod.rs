use config::Config;

pub mod head_cleanup;
pub mod strip_blank;


pub const PROCESSORS: [fn(Config, String) -> Result<String, String>; 2] =
    [head_cleanup::head_cleanup, strip_blank::strip_blank];
