#![allow(non_snake_case)]

use ignore::{WalkBuilder, DirEntry};
mod arguments;
use arguments::getArguments;

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
  
  for result in WalkBuilder::new(args.path).filter_entry(|path| !is_ignored_dir(path)).hidden(false).build() {
    println!("{:?}\n", result.unwrap().path().display());
  }
}