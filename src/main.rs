use dotenv::dotenv;
use std::{fs, env};
use pandoc::OutputKind;
use std::path::{PathBuf, Path};
use pandoc::PandocOption::Template;
use image::imageops::FilterType;
use regex::Regex;
use chrono::prelude::*;
use std::ops::Sub;
use std::fs::{DirEntry, File};
use simple_xml_builder::XMLElement;

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

  return config;
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
      output_file.push(file_name.replace("-write.md", ".html"));

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

fn get_file_date(file_entry: DirEntry) -> std::io::Result<String> {
  let file_name = file_entry.file_name().into_string().unwrap();

  let edit_time_unix = Local::now().timestamp().sub(file_entry.metadata()?.modified()?.elapsed().unwrap().as_secs() as i64);
  let edit_time_native = NaiveDateTime::from_timestamp(edit_time_unix, 0);
  let edit_time = edit_time_native.format("%H:%M:%S +1000").to_string();

  let timestamp = [&file_name.replace("-write.md", "").replace("-", " "), "00:00:00"].join(" ");
  let create_date = match NaiveDateTime::parse_from_str(&timestamp, "%y %m %d %H:%M:%S") {
    Ok(time) => time.format("%a, %d %b %Y").to_string(),
    Err(e) => ["Err", &e.to_string()].join(" ")
  };

  Ok([&create_date, &edit_time, ""].join(" "))
}

fn build_rss(config: &Config) -> std::io::Result<()> {
  println!("Building rss");

  let rss_file = File::create("./out/rss.xml")?;
  let mut rss_element = XMLElement::new("rss");
  rss_element.add_attribute("version", "2.0");
  rss_element.add_attribute("xmlns:atom", "http://www.w3.org/2005/Atom");

  let mut channel = XMLElement::new("channel");

  let mut title = XMLElement::new("title");
  title.add_text("pfy.ch");
  channel.add_child(title);

  let mut link = XMLElement::new("link");
  link.add_text("https://pfy.ch");
  channel.add_child(link);

  let mut description = XMLElement::new("description");
  description.add_text("Pfych blogs");
  channel.add_child(description);

  let mut atom = XMLElement::new("atom:link");
  atom.add_attribute("href", "https://pfy.ch/rss.xml");
  atom.add_attribute("rel", "self");
  atom.add_attribute("type", "application/rss+xml");
  channel.add_child(atom);


  for entry in fs::read_dir(&config.blog_location)? {
    let file_entry = entry?;
    let file_name = file_entry.file_name().into_string().unwrap();

    if file_name.contains(".md") {
      let mut rss_item = XMLElement::new("item");

      let mut guid = XMLElement::new("guid");
      guid.add_attribute("isPermaLink", "false");
      guid.add_text(&file_name);
      rss_item.add_child(guid);

      let mut title = XMLElement::new("title");
      title.add_text("Example");
      rss_item.add_child(title);

      let mut link = XMLElement::new("link");
      link.add_text(format!("https://pfy.ch/blog/{}", &file_name.replace("-write.md", ".html")));
      rss_item.add_child(link);

      let mut description = XMLElement::new("description");

      let file_content = fs::read_to_string(format!("./out/blog/{}", &file_name.replace("-write.md", ".html"))).unwrap().replace("\n", "");
      let remove_head_regex = Regex::new("^.*</h1> ").unwrap();
      let remove_trail = Regex::new("</div></body></html>").unwrap();
      let fix_images_regex = Regex::new("<img src=\".").unwrap();
      description.add_text(format!("<![CDATA[{}]]>", fix_images_regex.replace_all(
        remove_head_regex.replace(
          remove_trail.replace_all(
            &file_content,
            "",
          ).as_ref(),
          "").as_ref(),
        "<img src=\"https://pfy.ch/blog",
      )));
      rss_item.add_child(description);

      let mut author = XMLElement::new("author");
      author.add_text("pfych");
      rss_item.add_child(author);

      let mut pub_date = XMLElement::new("pubDate");
      pub_date.add_text(get_file_date(file_entry).unwrap());
      rss_item.add_child(pub_date);

      channel.add_child(rss_item)
    }
  }

  rss_element.add_child(channel);
  rss_element.write(rss_file).unwrap();

  Ok(())
}

fn main() -> std::io::Result<()> {
  let config = load_env();

  build_out_structure();
  build_blogs(&config).unwrap();
  build_images(&config).unwrap();
  build_rss(&config).unwrap();

  Ok(())
}
