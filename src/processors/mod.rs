use config::Config;

mod head_cleanup;
mod strip_blank;
mod rebrand;
mod references;
mod images;


pub const PROCESSORS: [fn(Config, String) -> Result<String, String>; 5] =
    [
        head_cleanup::head_cleanup,
        rebrand::rebrand,
        references::references,
        images::images,
        strip_blank::strip_blank,
    ];
