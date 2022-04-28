#![allow(non_snake_case)]

use ignore::{WalkBuilder, DirEntry, Error};
mod arguments;
use arguments::getArguments;
mod colors;
use colors::printRed;
use std::path::Path;
// use std::vec::Vec;

fn is_ignored_dir(entry: &DirEntry) -> bool {
  if entry.metadata().unwrap().file_type().is_dir() &&
    getArguments().ignoreDirs.len() >= 1 {
      let splitPath: Vec<&str> = entry.path().to_str().unwrap().split("/").collect();
      let dir: String = splitPath[splitPath.len()-1].to_string();
      if getArguments().ignoreDirs.contains(&dir) {
        return true;
      } else {
        return false;
      }
  }
  return false;
}

fn main() {
  let args = getArguments();
  let mut paths: Vec<&Path> = vec![];
  
  for result in WalkBuilder::new(args.path)
    .filter_entry(|path| !is_ignored_dir(path))
    .hidden(!args.showHidden)
    .git_ignore(!args.scanGitIgnore)
    .build() {
      println!("{:?}\n", result.unwrap().path().display());

      match result {
        Ok(res) => ,
        Err(e) => panic!("Invalid directory")
      }
      paths.push(result.unwrap().path());
  }

  printRed(String::from("This should be red text"));
}