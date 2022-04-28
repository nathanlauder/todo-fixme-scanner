#![allow(non_camel_case_types)]

extern crate structopt;
use structopt::StructOpt;
use std::path::PathBuf;

#[derive(StructOpt, Debug)]
#[structopt(name = "\nFIXTODO Scanner",
            about="\nThis package can be used to recursively scan all files in a project\nto create a centralized list of TODOs and FIXMEs.  They can then be displayed\nin an interactive list format specific to a file.")]
pub struct cliArgs {

  /// Ignore all hidden files and folders, default to true
  #[structopt (long="--scan-hidden")]
  pub showHidden: bool,

  /// Scan files regardless of gitignore status, default to false
  #[structopt (long="--scan-ignored")]
  pub scanGitIgnore: bool,

  /// Provide a list of space separated directories to ignore (.git dirName).
  /// This dir name should be just the name of the top dir to ignore.
  /// No full path (src/myDir/anotherDir), no slash at the end (myDir/)
  #[structopt(long="--ignore-dirs")]
  pub ignoreDirs: Vec<String>,

  /// Specifies a path to run the command on
  #[structopt(short, long, default_value = ".", parse(from_os_str))]
  pub path: PathBuf,

  /// Ignore FIXMEs, default to false
  #[structopt(long, short="-f")]
  pub noFixme: bool,

  /// Ignore TODOs, default to false
  #[structopt(long, short="-t")]
  pub noTodo: bool,

  /// Show scanner in action
  #[structopt(short, long)]
  pub verbose: bool,
}

pub fn getArguments() -> cliArgs {
  cliArgs::from_args()
}