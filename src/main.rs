#![allow(non_snake_case)]
mod arguments;
mod log;
mod file;

use ignore::{WalkBuilder, DirEntry, Error};
use std::fs::read;
use std::{fs, path::Path};
use std::collections::HashMap;

use arguments::getArguments;
use log::{info, debug, success, warn};
use file::{readFiles};
use std::ffi::OsStr;

fn is_node_modules(entry: &DirEntry) -> bool {
  if entry.metadata().unwrap().file_type().is_dir() {
      let splitPath: Vec<&str> = entry.path().to_str().unwrap().split("/").collect();
      let dir: String = splitPath[splitPath.len()-1].to_string();
      return dir == "node_modules".to_string();
  }
  return false;
}

fn is_ignored_dir(entry: &DirEntry) -> bool {
  if entry.metadata().unwrap().file_type().is_dir() &&
    arguments::getArguments().ignoreDirs.len() >= 1 {
      let splitPath: Vec<&str> = entry.path().to_str().unwrap().split("/").collect();
      let dir: String = splitPath[splitPath.len()-1].to_string();
      return getArguments().ignoreDirs.contains(&dir);
  }
  return false;
}

fn isImageFile(fileExtension: Option<&OsStr>) -> bool {
  // let extension: String = fileExtension.and_then(OsStr::to_str).unwrap().to_string();
  let extension = fileExtension.and_then(OsStr::to_str);
  let imgs: Vec<String> = vec!["jpg",
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
    Some(ex) => return imgs.contains(&ex.to_string()),
    None => {
      println!("{:?}", extension);
      warn("Unsure of file extension. Skipping".to_string());
      return false;
    }
  }
}

fn filter_files(entries: Vec<DirEntry>) -> Vec<DirEntry> {
  let mut files: Vec<DirEntry> = Vec::new();
  for entry in entries {
    if entry.path().is_file() { 
      if !isImageFile(entry.path().extension()) {
        files.push(entry);
      } else {
        warn(entry.path().extension().expect("msg").to_str().unwrap().to_string());
      }
    }
  }
  files
}

// Return type of Result allows us to use the '?' operator
fn main() -> Result<(), Error> {
    let args = arguments::getArguments();

    let walk = WalkBuilder::new(args.path)
        .filter_entry(|path| !is_ignored_dir(path))
        .filter_entry(|path| !is_node_modules(path))
        .hidden(!args.showHidden)
        .git_ignore(!args.scanGitIgnore)
        .build();

    let entries: Result<Vec<DirEntry>, Error> = walk.collect();
    let entries: Vec<DirEntry> = entries?;

    let files: Vec<DirEntry> = filter_files(entries);
    let paths: Vec<&Path> = files.iter().map(|d| d.path()).collect();

    debug(&paths);

    readFiles(paths);

    success("This funny business should be green".to_string());
    Ok(())
}
