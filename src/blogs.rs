use pandoc::OutputKind;
use pandoc::PandocOption::Template;
use std::path::PathBuf;
use std::fs;
use crate::config;

pub(crate) fn build_blogs(config: &config::Config) -> std::io::Result<()> {
  println!("Building blog html");
  let output_folder: PathBuf = PathBuf::from("./out/blog");
  let pandoc_template = PathBuf::from(&config.blog_template);

  for entry in fs::read_dir(&config.blog_location)? {
    let mut pandoc = pandoc::new();
    let file_entry = entry?;
    let file_name = file_entry.file_name().into_string().unwrap();
    let file_path = file_entry.path();

    if file_name.contains(".md") {
      let mut output_file: PathBuf = PathBuf::from(&output_folder);
      output_file.push(file_name.replace("-write.md", ".html"));

      pandoc.add_input(&file_path);
      pandoc.set_output(OutputKind::File(output_file));
      pandoc.add_option(Template(pandoc_template.to_path_buf()));
      match pandoc.execute() {
        Ok(_t) => {}
        Err(e) => println!("Error {}", e)
      }
    }
  }

  Ok(())
}
