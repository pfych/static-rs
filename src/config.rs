use std::{env, fs};

pub struct Config {
  pub blog_location: String,
  pub blog_template: String,
  pub index_template: String,
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

  let json: serde_json::Value = serde_json::from_reader(file).expect("File should be valid JSON");

  println!("Running with following config:\n{}\n", json);

  let config = Config {
    blog_location: json.get("blog_location").unwrap().as_str().unwrap().to_string(),
    blog_template: json.get("blog_template").unwrap().as_str().unwrap().to_string(),
    index_template: json.get("index_template").unwrap().as_str().unwrap().to_string(),
    image_location: json.get("image_location").unwrap().as_str().unwrap().to_string(),
    author: json.get("author").unwrap().as_str().unwrap().to_string(),
    url: json.get("url").unwrap().as_str().unwrap().to_string(),
    file_suffix: json.get("file_suffix").unwrap().as_str().unwrap().to_string(),
  };

  return config;
}
