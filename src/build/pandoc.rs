use pandoc::{self, Pandoc, PandocOutput, PandocError};
use std::error::Error;
use utils::get_exe_dir;
use config::{Config, References};


fn execute_pandoc(
    input: String,
    references: Option<References>,
) -> Result<PandocOutput, PandocError> {
    let mut renderer = Pandoc::new();
    renderer.set_output_format(pandoc::OutputFormat::Html, vec![]);
    renderer.set_input_format(pandoc::InputFormat::Markdown, vec![]);
    renderer.set_input(pandoc::InputKind::Pipe(input));
    renderer.set_output(pandoc::OutputKind::Pipe);
    renderer.add_option(pandoc::PandocOption::Smart);
    renderer.add_option(pandoc::PandocOption::Standalone);
    renderer.add_pandoc_path_hint(&get_exe_dir());
    if references.is_some() {
        let References { csl, bibliography } = references.unwrap();
        renderer.set_bibliography(&bibliography);
        renderer.set_csl(&csl);
    }
    return renderer.execute();
}


pub fn render(config: Config, input: String) -> Result<String, String> {
    let output = execute_pandoc(input, config.references);
    if output.is_err() {
        return Err(output.err().unwrap().description().into());
    }
    return match output.unwrap() {
        pandoc::PandocOutput::ToBuffer(out) => Ok(out),
        _ => Err("Incorrect output type.".into()),
    };
}
