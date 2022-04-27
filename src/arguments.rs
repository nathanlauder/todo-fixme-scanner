#![allow(non_camel_case_types)]

extern crate structopt;
use structopt::StructOpt;
use std::path::PathBuf;

#[derive(StructOpt, Debug)]
#[structopt(name = "FIXME & TODO list scanner")]
pub struct cliArgs {
  /// Ignore all hidden files and folders
  #[structopt (long="--ignore-hidden")]
  pub ignoreHidden: bool,

  /// Provide a list of space separated directories to ignore (.git dirName).
  /// This dir name should be just the name of the top dir to ignore.
  /// No full path (src/myDir/anotherDir), no slash at the end (myDir/)
  #[structopt(long="--ignore-dirs")]
  pub ignoreDirs: Vec<String>,

  /// Specifies a path to run the command on
  #[structopt(short, long, default_value = ".", parse(from_os_str))]
  pub path: PathBuf,

  /// Ignore FIXMEs
  #[structopt(long)]
  pub noFixme: bool,

  /// Ignore TODOs
  #[structopt(long)]
  pub noTodo: bool,

  // The number of occurrences of the `v/verbose` flag
  /// Verbose mode (-v, -vv, -vvv, etc.)
  #[structopt(short, long, parse(from_occurrences))]
  pub verbose: u8,
}

pub fn getArguments() -> cliArgs {
  cliArgs::from_args()
}