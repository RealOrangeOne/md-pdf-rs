use config::Config;
use pandoc::Pandoc;

fn build_pandoc(config: Config) -> Pandoc {
    return Pandoc::new();
}


pub fn render(config: Config, input: String) {
    let renderer = build_pandoc(config);
}
