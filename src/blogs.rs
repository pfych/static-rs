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

  for entry in get_blogs(config) {
    let mut pandoc = pandoc::new();
    let file_entry = entry;
    let file_name = file_entry.file_name().into_string().expect("File should have a valid name");
    let file_path = file_entry.path();

    if file_path.extension().unwrap_or_default() == "md" {
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

  let contents = fs::read_to_string(&config.index_template).expect("Index file should be readable");

  let mut toc: Vec<String> = Vec::new();
  for entry in get_blogs(config) {
    let file_entry = entry;
    let file_name = file_entry.file_name().into_string().expect("File should have a valid name");
    let file_path = file_entry.path();

    if file_path.extension().unwrap_or_default() == "md" {
      let draft = utils::get_metadata(&file_path, "draft");
      if !draft.is_empty() { continue };

      let title = utils::get_metadata(&file_path, "title");
      let url = format!("./blog/{}.html", file_name.replace(&config.file_suffix, ""));

      let mut category = utils::get_metadata(&file_path, "category");
      if category.is_empty() {
        category = String::from("📄")
      }

      toc.push(format!("<a href=\"{1}\">{2} {0} <span class=\"name\">{3}</span></a>", category, url, file_name.replace(&config.file_suffix, ""), title))
    }
  }

  let new_contents = contents.replace("TOC", &toc.join(""));

  let out_file = File::create("./out/index.html");
  out_file.unwrap().write_all(new_contents.as_bytes()).expect("Unable to write to file");

  Ok(())
}
