use std::fs::{File, DirEntry};
use std::{io, fs};
use std::io::BufRead;
use std::path::{Path};
use crate::config;
use regex::Regex;

pub fn get_metadata(file: &Path, meta: &str) -> String {
  let md_file = File::open(&file).unwrap();
  let md_lines = io::BufReader::new(md_file);
  let mut val = "".to_string();
  for (_i, line) in md_lines.lines().enumerate() {
    let line = line.unwrap();
    let meta_regex = Regex::new(format!("^{}:.*$", &meta).as_str()).unwrap();
    if meta_regex.is_match(line.as_str()) {
      val = line.replace(&[meta, ": "].join(""), "");
      break;
    }
  }

  val
}

pub fn get_blogs(config: &config::Config) -> Vec<DirEntry> {
  let mut paths: Vec<_> = fs::read_dir(&config.blog_location).unwrap()
    .map(|r| r.unwrap())
    .collect();

  paths.sort_by_key(|dir| dir.path());

  paths
}
