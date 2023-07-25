extern crate colored;
use colored::*;
use std::fmt::{Display, Debug};
#[path = "./arguments.rs"] mod arguments;

#[allow(dead_code)]
pub fn debug(expr: &impl Debug) {
  if arguments::get_arguments().verbose {
    println!("{:?}", expr);
  }
}

#[allow(dead_code)]
pub fn info(expr: impl Display) {
  if arguments::get_arguments().verbose {
    println!("{}", expr);
  }
}

#[allow(dead_code)]
pub fn error(msg: String) {
  let error: ColoredString = "ERROR".to_string().red();
  if arguments::get_arguments().verbose {
    println!("{} {}", error, msg.red());
  }
}

#[allow(dead_code)]
pub fn success(msg: String) {
  let success: ColoredString = "SUCCESS".to_string().green().bold();
  if arguments::get_arguments().verbose {
    println!("{} {}", success, msg.green().bold());
  }
}

pub fn warn (msg: String) {
  let warning: ColoredString = "WARNING:".to_string().yellow().bold();
  if arguments::get_arguments().verbose {
    println!("{} {}", warning, msg.yellow().bold())
  }
}