use dotenv::dotenv;
use std::{fs, env};
use pandoc::OutputKind;
use std::path::{PathBuf, Path};
use pandoc::PandocOption::Template;
use image::imageops::FilterType;
use regex::Regex;

struct Config {
  blog_location: String,
  blog_template: String,
  image_location: String,
}

fn resize(in_path: &str, out_path: &str, width: u32) {
  match image::open(in_path) {
    Ok(img) => img.resize(width, width, FilterType::Nearest).save(out_path).unwrap(),
    Err(e) => eprintln!("Error {}", e)
  }
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
    },
    image_location: match env::var("IMAGE_LOCATION") {
      Ok(val) => val,
      Err(_e) => String::from("")
    },
  };

  return config
}

fn build_out_structure() {
  println!("Building output structure");
  fs::create_dir_all("./out").unwrap();
  fs::create_dir_all("./out/blog").unwrap();
  fs::create_dir_all("./out/blog/images").unwrap();
}

fn build_blogs(config: &Config) -> std::io::Result<()> {
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

fn build_images(config: &Config) -> std::io::Result<()> {
  println!("Building images");
  let output_folder: PathBuf = PathBuf::from("./out/blog/images");

  for entry in fs::read_dir(&config.image_location)? {
    let file_entry = entry?;
    let file_name = file_entry.file_name().into_string().unwrap();
    let file_path = file_entry.path();

    let mut output_file: PathBuf = PathBuf::from(&output_folder);
    let re = Regex::new(".JPG").unwrap();
    output_file.push(re.replace(&file_name, ".jpg").to_string());

    if !Path::new(&output_file).exists() {
      println!("Resizing {}", &file_name);

      resize(file_path.to_str().unwrap(), output_file.to_str().unwrap(), 720)
    }
  }

  Ok(())
}

fn main() -> std::io::Result<()> {
  let config = load_env();

  build_out_structure();
  build_blogs(&config).unwrap();
  build_images(&config).unwrap();

  Ok(())
}
