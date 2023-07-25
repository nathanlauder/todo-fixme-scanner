extern crate structopt;
use structopt::StructOpt;
use std::path::PathBuf;
#[derive(StructOpt, Debug)]
#[structopt(name = "TODO & FIXME Scanner",
            about="\nThis package can be used to recursively scan all files in a project\nto create a centralized list of TODOs and FIXMEs.  They can then be displayed\nin an interactive list format specific to a file.")]
pub struct CliArgs {

  /// Ignore all hidden files and folders, default to true
  #[structopt (long="--scan-hidden")]
  pub show_hidden: bool,

  /// Scan files regardless of gitignore status, default to false
  #[structopt (long="--scan-ignored")]
  pub scan_git_ignore: bool,

  /// Provide a list of space separated directories to ignore (.git dirName).
  /// This dir name should be just the name of the top dir to ignore.
  /// No full path (src/myDir/anotherDir), no slash at the end (myDir/)
  #[structopt(long="--ignore-dirs")]
  pub ignore_dirs: Vec<String>,

  /// Specifies a path to run the command on
  #[structopt(short, long, default_value = ".", parse(from_os_str))]
  pub path: PathBuf,

  /// Only scan FIXMEs, default to false
  #[structopt(long, short="-f")]
  pub only_fixme: bool,

  /// Only scan TODOs, default to false
  #[structopt(long, short="-t")]
  pub only_todo: bool,

  /// Show scanner in action
  #[structopt(short, long)]
  pub verbose: bool,
}

pub fn get_arguments() -> CliArgs {
  CliArgs::from_args()
}