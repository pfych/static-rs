use std::{env, fs};

#[derive(Debug, serde::Deserialize)]
pub struct Config {
  pub blog_location: String,
  pub blog_template: String,
  pub index_template: String,
  pub image_page_template: String,
  pub image_location: String,
  pub url: String,
  pub author: String,
  pub file_suffix: String,
}

pub fn load_env() -> Config {
  let file = fs::File::open(match env::var("STATIC_RS_CONFIG") {
    Ok(val) => val,
    Err(_e) => String::from("./config.json")
  }).expect("file should open read only");

  serde_json::from_reader(file).expect("File should be valid JSON")
}
