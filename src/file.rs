use std::collections::HashMap;
use std::{fs, path::Path};
#[path = "./log.rs"] mod log;
use log::{debug, info};
use std::ffi::OsStr;

pub fn readFiles(paths: Vec<&Path>) -> HashMap<&Path, String> {
  let mut files: HashMap<&Path, String> = HashMap::new();
  // let imgs: Vec<Option<String>>  = vec![Some("jpg".to_string()), Some("jpeg".to_string()), Some("png".to_string()), Some("webp".to_string()), Some("gif")];
  for path in paths {
    let metadata = path.metadata().expect("metadata filed");
    // let ftype = metadata.file_type();
    // println!("{:?}", path.extension());
    
    let fileContents: String = fs::read_to_string(path).expect("read the file");
    files.insert(path, fileContents);
  } 
  files
}