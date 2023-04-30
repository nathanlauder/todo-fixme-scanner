#![allow(non_snake_case)]

use ignore::{WalkBuilder, DirEntry, Error};
mod arguments;
use arguments::getArguments;

#[macro_use]
mod log;
use log::{info, debug, success};

use std::{fs, path::Path};

fn is_ignored_dir(entry: &DirEntry) -> bool {
  if entry.metadata().unwrap().file_type().is_dir() &&
    arguments::getArguments().ignoreDirs.len() >= 1 {
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


fn readFile(paths: Vec<&Path>) {
    for path in paths {
      let fileContents: String = fs::read_to_string(path).expect("read the file");
      info("\n\n\n\n");
      info(fileContents);
    } 
}

fn filter_files(entries: Vec<DirEntry>) -> Vec<DirEntry> {
  let mut files: Vec<DirEntry> = Vec::new();
  for entry in entries {
    if entry.path().is_file() {
      files.push(entry);
    }
  }
  files
}

// Return type of Result allows us to use the '?' operator
fn main() -> Result<(), Error> {
    let args = arguments::getArguments();

    let walk = WalkBuilder::new(args.path)
        .filter_entry(|path| !is_ignored_dir(path))
        .hidden(!args.showHidden)
        .git_ignore(!args.scanGitIgnore)
        .build();


    let entries: Result<Vec<DirEntry>, Error> = walk.collect();
    let entries: Vec<DirEntry> = entries?;

    let files: Vec<DirEntry> = filter_files(entries);
    let paths: Vec<&Path> = files.iter().map(|d| d.path()).collect();

    debug(&paths);

    info("========================");

    readFile(paths);

    success("This funny business should be green".to_string());
    Ok(())
}
