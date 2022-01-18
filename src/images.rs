use crate::config;
use image::imageops::FilterType;
use std::path::{PathBuf, Path};
use std::{fs, thread};
use regex::Regex;

fn resize(in_path: &str, out_path: &str, width: u32) {
  match image::open(in_path) {
    Ok(img) => img.resize(width, width, FilterType::Nearest).save(out_path).unwrap(),
    Err(e) => eprintln!("Error {}", e)
  }
}

pub fn build_images(config: config::Config) -> std::io::Result<()> {
  println!("Building images");

  let mut threads: Vec<_> = Vec::new();

  for entry in fs::read_dir(&config.image_location).unwrap() {
    threads.push(thread::spawn(|| {
      let output_folder: PathBuf = PathBuf::from("./out/blog/images");

      let file_entry = entry.unwrap();
      let file_name = file_entry.file_name().into_string().unwrap();
      let file_path = file_entry.path();

      let mut output_file: PathBuf = PathBuf::from(&output_folder);
      let re = Regex::new(".JPG").unwrap();
      output_file.push(re.replace(&file_name, ".jpg").to_string());

      if !Path::new(&output_file).exists() {
        println!("Resizing {}", &file_name);
        resize(file_path.to_str().unwrap(), output_file.to_str().unwrap(), 720);
        println!("Completed Resizing {}", &file_name);
      }
    }))
  };

  for thread in threads {
    thread.join().unwrap()
  }

  Ok(())
}
