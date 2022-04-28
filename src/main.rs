#![allow(non_snake_case)]

use ignore::{WalkBuilder, DirEntry, Error};
mod arguments;
use arguments::getArguments;
mod colors;
use colors::printRed;
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

// Return type of Result allows us to use the '?' operator
fn main() -> Result<(), Error> {
    let args = getArguments();

    // create Walk object
    // know that when we walk, we may encounter errors
    // these errors are basically Java's IOException, which can happen
    // for various reasons
    let walk = WalkBuilder::new(args.path)
        .filter_entry(|path| !is_ignored_dir(path))
        .hidden(!args.showHidden)
        .git_ignore(!args.scanGitIgnore)
        .build();

    // transform Walk to Vec<DirEntry>
    // notice that Rust actually transforms Vec<Result<DirEntry, Error>>
    // into Result<Vec<DirEntry>, Error>, where Error is the first error
    // that might be encountered
    // Vec<_> here means Vec<DirEntry>, it's implicit but you can write it if you want
    let entries: Result<Vec<_>, Error> = walk.collect();
    // and now we resolve the Result (returning Err if necessary)
    // now we have Vec<DirEntry>
    let entries = entries?;
    
    // finally, transform Vec<DirEntry> to Vec<&Path> by mapping it
    // Vec<_> means Vec<&Path>, again it's implicit and you can write it if you want
    let paths: Vec<_> = entries.iter().map(|d| d.path()).collect();

    println!("{:?}", paths);

    printRed(String::from("This should be red text"));
    Ok(())
}
