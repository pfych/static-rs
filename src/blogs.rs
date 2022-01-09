use pandoc::OutputKind;
use pandoc::PandocOption::Template;
use std::path::PathBuf;
use std::{fs};
use crate::{config, utils};
use std::fs::File;
use std::io::Write;
use crate::utils::get_blogs;

pub fn build_blogs(config: &config::Config) -> std::io::Result<()> {
  println!("Building blog html");
  let output_folder: PathBuf = PathBuf::from("./out/blog");
  let pandoc_template = PathBuf::from(&config.blog_template);

  for entry in get_blogs(&config) {
    let mut pandoc = pandoc::new();
    let file_entry = entry;
    let file_name = file_entry.file_name().into_string().unwrap();
    let file_path = file_entry.path();

    if file_name.contains(".md") {
      let mut output_file: PathBuf = PathBuf::from(&output_folder);
      output_file.push(file_name.replace(&config.file_suffix, ".html"));

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

pub fn build_index(config: &config::Config) -> std::io::Result<()> {
  println!("Creating TOC");

  let contents = fs::read_to_string(&config.index_template).expect("Index file should be valid html");

  let mut toc = Vec::new();
  for entry in get_blogs(&config) {
    let file_entry = entry;
    let file_name = file_entry.file_name().into_string().unwrap();
    let file_path = file_entry.path();

    if file_name.contains(".md") {
      let draft = utils::get_metadata(&file_path, "draft");
      if draft.len() != 0 { continue };

      let title = utils::get_metadata(&file_path, "title");
      let url = format!("./blog/{}.html", file_name.replace(&config.file_suffix, ""));

      toc.push(format!("<a href=\"{}\">{} - {}</a>", url, file_name.replace(&config.file_suffix, ""), title))
    }
  }

  let new_contents = contents.replace("TOC", &toc.join(""));

  let out_file = File::create("./out/index.html");
  out_file.unwrap().write(new_contents.as_bytes()).unwrap();

  Ok(())
}
