use dotenv::dotenv;
use std::{fs, env};
use pandoc::OutputKind;
use std::path::PathBuf;
use pandoc::PandocOption::Template;

struct Config {
  blog_location: String,
  blog_template: String
}

fn load_env() -> Config {
  dotenv().ok();

  let config: Config = Config {
    blog_location: match env::var("BLOG_LOCATION") {
      Ok(val) => val,
      Err(_e) => String::from("")
    },
    blog_template: match env::var("BLOG_TEMPLATE") {
      Ok(val) => val,
      Err(_e) => String::from("")
    }
  };

  return config
}

fn build_out_structure() {
  println!("Building output structure");
  fs::create_dir_all("./out").unwrap();
  fs::create_dir_all("./out/blog").unwrap();
}

fn build_blogs(config: Config) -> std::io::Result<()> {

  println!("\nLoading blogs from: {}", config.blog_location);

  let output_folder: PathBuf = PathBuf::from("./out/blog");
  let pandoc_template = PathBuf::from(config.blog_template);

  for entry in fs::read_dir(config.blog_location)? {
    let mut pandoc = pandoc::new();
    let dir = entry?;
    let file_name = dir.file_name().into_string().unwrap();
    let file_path = dir.path();

    if file_name.contains(".md") {
      let mut output_file: PathBuf = PathBuf::from(&output_folder);
      output_file.push(file_name.replace(".md", ".html"));

      pandoc.add_input(&file_path);
      pandoc.set_output(OutputKind::File(output_file));
      pandoc.add_option(Template(pandoc_template.to_path_buf()));
      match pandoc.execute() {
        Ok(_t) => {},
        Err(e) => println!("Error {}", e)
      }
    }
  }

  Ok(())
}

fn main() -> std::io::Result<()> {
  let config = load_env();

  build_out_structure();
  build_blogs(config).unwrap();

  Ok(())
}
