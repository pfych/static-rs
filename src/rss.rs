use std::fs::{DirEntry, File};
use chrono::{Local, NaiveDateTime};
use std::ops::Sub;
use crate::config;
use simple_xml_builder::XMLElement;
use std::{fs, io};
use std::io::BufRead;
use regex::Regex;

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

pub(crate) fn build_rss(config: &config::Config) -> std::io::Result<()> {
  println!("Building rss");

  let rss_file = File::create("./out/rss.xml")?;
  let mut rss_element = XMLElement::new("rss");
  rss_element.add_attribute("version", "2.0");
  rss_element.add_attribute("xmlns:atom", "http://www.w3.org/2005/Atom");

  let mut channel = XMLElement::new("channel");

  let mut title = XMLElement::new("title");
  title.add_text(&config.url);
  channel.add_child(title);

  let mut link = XMLElement::new("link");
  link.add_text(&config.url);
  channel.add_child(link);

  let mut description = XMLElement::new("description");
  description.add_text("Pfych blogs");
  channel.add_child(description);

  let mut atom = XMLElement::new("atom:link");
  atom.add_attribute("href", format!("{}/rss.xml", &config.url));
  atom.add_attribute("rel", "self");
  atom.add_attribute("type", "application/rss+xml");
  channel.add_child(atom);


  for entry in fs::read_dir(&config.blog_location)? {
    let file_entry = entry?;
    let file_name = file_entry.file_name().into_string().unwrap();
    let file_path = file_entry.path();

    if file_name.contains(".md") {
      let mut rss_item = XMLElement::new("item");

      let mut guid = XMLElement::new("guid");
      guid.add_attribute("isPermaLink", "false");
      guid.add_text(&file_name);
      rss_item.add_child(guid);

      let mut title = XMLElement::new("title");
      let md_file = File::open(&file_path).unwrap();
      let md_lines = io::BufReader::new(md_file);
      let mut title_val = String::from("New post");
      for (_i, line) in md_lines.lines().enumerate() {
        let line = line.unwrap();
        if line.contains("title:") {
          title_val = line.replace("title: ", "");
          break;
        }
      }
      title.add_text(title_val);
      rss_item.add_child(title);

      let mut link = XMLElement::new("link");
      link.add_text(format!("{}/blog/{}", &config.url, &file_name.replace("-write.md", ".html")));
      rss_item.add_child(link);

      let mut description = XMLElement::new("description");

      let file_content = fs::read_to_string(format!("./out/blog/{}", &file_name.replace("-write.md", ".html"))).unwrap().replace("\n", "");
      let remove_head_regex = Regex::new("^.*</h1> ").unwrap();
      let remove_trail_regex = Regex::new("</div></body></html>").unwrap();
      let fix_images_regex = Regex::new("<img src=\".").unwrap();

      description.add_text(format!("{}", fix_images_regex.replace_all(
        remove_head_regex.replace(
          remove_trail_regex.replace_all(
            &file_content,
            "",
          ).as_ref(),
          "").as_ref(),
        format!("<img src=\"{}/blog", &config.url),
      )));

      rss_item.add_child(description);

      let mut author = XMLElement::new("author");
      author.add_text(&config.author);
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
