use std::fs;

mod config;
mod images;
mod rss;
mod blogs;

fn build_out_structure() {
  println!("Building output structure");
  fs::create_dir_all("./out").unwrap();
  fs::create_dir_all("./out/blog").unwrap();
  fs::create_dir_all("./out/blog/images").unwrap();
}

fn main() {
  let config = config::load_env();

  build_out_structure();
  blogs::build_blogs(&config).unwrap();
  rss::build_rss(&config).unwrap();
  images::build_images(config).unwrap();
}
