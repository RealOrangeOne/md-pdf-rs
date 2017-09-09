use pandoc::{self, Pandoc, PandocOutput, PandocError};
use std::error::Error;
use utils::get_exe_dir;
use build::csl::unpack_csl;
use std::fs::remove_file;
use std::path::PathBuf;


fn execute_pandoc(input: String, csl_dir: Option<PathBuf>) -> Result<PandocOutput, PandocError> {
    let mut renderer = Pandoc::new();
    renderer.set_output_format(pandoc::OutputFormat::Html, vec![]);
    renderer.set_input_format(pandoc::InputFormat::Markdown, vec![]);
    renderer.set_input(pandoc::InputKind::Pipe(input));
    renderer.set_output(pandoc::OutputKind::Pipe);
    renderer.add_option(pandoc::PandocOption::Smart);
    renderer.add_option(pandoc::PandocOption::Standalone);
    renderer.add_pandoc_path_hint(&get_exe_dir());
    return renderer.execute();
}


pub fn render(input: String) -> Result<String, String> {
    let csl_dir = unpack_csl("apa".into());
    let output = execute_pandoc(input, Some(csl_dir.clone()));
    if csl_dir.exists() {
        remove_file(csl_dir);
    }
    if output.is_err() {
        return Err(output.err().unwrap().description().into());
    }
    return match output.unwrap() {
        pandoc::PandocOutput::ToBuffer(out) => Ok(out),
        _ => Err("Incorrect output type.".into()),
    };
}
