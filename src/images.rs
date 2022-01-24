use crate::config;
use image::imageops::FilterType;
use std::path::{PathBuf, Path};
use std::{fs, thread};
use regex::Regex;
use std::fs::File;
use std::io::Write;

fn resize(in_path: &str, out_path: &str, width: u32) {
  match image::open(in_path) {
    Ok(img) => img.resize(width, width, FilterType::Nearest).save(out_path).unwrap(),
    Err(e) => eprintln!("Error {}", e)
  }
}

pub fn build_images(config: config::Config) -> std::io::Result<()> {
  println!("Building images");

  let mut threads: Vec<_> = Vec::new();
  let mut images = Vec::new();

  for entry in fs::read_dir(&config.image_location).unwrap() {
    let file_name = entry.as_ref().unwrap().file_name().into_string().unwrap();
    let re = Regex::new(".JPG").unwrap();

    images.push(format!(
      "<a id=\"{}\" href=\"#{}\"><div id=\"{}\" class=\"anchor\"></div><img src=\"./blog/images/{}\" alt=\"{}\" /></a>",
      re.replace(&file_name, ".jpg"),
      re.replace(&file_name, ".jpg"),
      re.replace(&file_name, ".jpg"),
      re.replace(&file_name, ".jpg"),
      re.replace(&file_name, ".jpg")
    ));

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
        resize(file_path.to_str().unwrap(), output_file.to_str().unwrap(), 1920);
        println!("Completed Resizing {}", &file_name);
      }
    }))
  };

  for thread in threads {
    thread.join().unwrap()
  }

  let contents = fs::read_to_string(&config.image_page_template).expect("Image page template file should be readable");
  let new_contents = contents.replace("IMAGES", &images.join(""));

  let out_file = File::create("./out/images.html");
  out_file.unwrap().write_all(new_contents.as_bytes()).expect("Unable to write to file");

  Ok(())
}
