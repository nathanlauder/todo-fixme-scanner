mod arguments;
mod log;
mod file;

use ignore::{WalkBuilder, DirEntry, Error};
use std::path::Path;

use arguments::get_arguments;
use log::{debug, success, warn};
use file::{read_files, is_image_file};

fn is_node_modules(entry: &DirEntry) -> bool {
  if entry.metadata().unwrap().file_type().is_dir() {
      let split_path: Vec<&str> = entry.path().to_str().unwrap().split("/").collect();
      let dir: String = split_path[split_path.len()-1].to_string();
      return dir == "node_modules".to_string();
  }
  return false;
}

fn is_ignored_dir(entry: &DirEntry) -> bool {
  if entry.metadata().unwrap().file_type().is_dir() &&
    arguments::get_arguments().ignore_dirs.len() >= 1 {
      let split_path: Vec<&str> = entry.path().to_str().unwrap().split("/").collect();
      let dir: String = split_path[split_path.len()-1].to_string();
      return get_arguments().ignore_dirs.contains(&dir);
  }
  return false;
}

fn filter_files(entries: Vec<DirEntry>) -> Vec<DirEntry> {
  let mut files: Vec<DirEntry> = Vec::new();
  for entry in entries {
    if entry.path().is_file() { 
      if !is_image_file(entry.path()) {
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
    let args = arguments::get_arguments();

    let walk = WalkBuilder::new(args.path)
        .filter_entry(|path|
            !is_ignored_dir(path) && 
            !is_node_modules(path))
        .hidden(!args.show_hidden)
        .git_ignore(!args.scan_git_ignore)
        .build();

    let entries_result: Result<Vec<DirEntry>, Error> = walk.collect();
    let entries: Vec<DirEntry> = entries_result?;

    let files: Vec<DirEntry> = filter_files(entries);
    let paths: Vec<&Path> = files.iter().map(|d| d.path()).collect();

    debug(&paths);

    let file_map = read_files(paths);
    // scan_for_todos(file_map);

    success("This funny business should be green".to_string());
    Ok(())
}
