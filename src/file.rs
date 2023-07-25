use std::collections::HashMap;
use std::{fs, path::Path};
#[path = "./log.rs"] mod log;
#[path = "./comments.rs"] mod comments;
use log::{debug, warn, info};
use std::io::{prelude::*, BufReader};
use std::ffi::OsStr;
use comments::get_comment_string;

pub fn get_file_extension(path: &Path) -> Option<&str> {
  path.extension().and_then(OsStr::to_str)
}

pub fn read_files(paths: Vec<&Path>) -> HashMap<&Path, Vec<String>> {
  let mut files: HashMap<&Path, Vec<String>> = HashMap::new();
  for path in paths {
    let file = fs::File::open(path).expect("no such file");
    let buf = BufReader::new(file);
    let contents: Vec<String> = buf.lines()
      .map(|line| line.expect("could not parse line"))
      .collect();
    files.insert(path, contents);
  } 
  files
}

pub fn is_image_file(file_path: &Path) -> bool {
  let extension: Option<&str> = file_path.extension().and_then(OsStr::to_str);
  let imgs: Vec<String> = vec![
    "jpg",
    "jpeg",
    "png",
    "webp",
    "gif",
    "pdf",
    "pptx",
    "psd",
    "afdesign",
    "ico"
  ].into_iter().map(|ex| { ex.to_string() }).collect();
  match extension {
    Some(ext) => return imgs.contains(&ext.to_string()),
    None => {
      warn("Unsure of file extension. Skipping".to_string());
      return false;
    }
  }
}

pub fn scan_for_todos(file_map: HashMap<&Path, String>) {
  for file in file_map.keys() {
    info(format!("file in map: {:?}", file));
    let ext = get_file_extension(file).unwrap_or("");
    let mut line_counter: u32 = 1;
    if ext != "" {
      for line in file_map.get(file) {
        line.split(" ");
      }
    }
  }

}