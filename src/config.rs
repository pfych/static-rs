use dotenv::dotenv;
use std::env;

pub(crate) struct Config {
  pub blog_location: String,
  pub blog_template: String,
  pub image_location: String,
  pub url: String,
  pub author: String
}

pub(crate) fn load_env() -> Config {
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
    author: match env::var("AUTHOR") {
      Ok(val) => val,
      Err(_e) => String::from("")
    },
    url: match env::var("URL") {
      Ok(val) => val,
      Err(_e) => String::from("")
    },
  };

  return config;
}
