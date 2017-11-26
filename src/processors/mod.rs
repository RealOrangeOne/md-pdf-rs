use config::Config;

mod head_cleanup;
mod strip_blank;
mod rebrand;
mod references;
mod images;
mod static_files;

pub const PROCESSORS: [fn(Config, String) -> Result<String, String>; 6] =
    [
        head_cleanup::head_cleanup,
        rebrand::rebrand,
        references::references,
        images::images,
        static_files::static_files,
        strip_blank::strip_blank,
    ];
