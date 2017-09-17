use config::Config;

mod head_cleanup;
mod strip_blank;
mod rebrand;
mod references;


pub const PROCESSORS: [fn(Config, String) -> Result<String, String>; 4] =
    [
        head_cleanup::head_cleanup,
        rebrand::rebrand,
        references::references,
        strip_blank::strip_blank,
    ];
