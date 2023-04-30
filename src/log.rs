extern crate colored;
use colored::*;
use std::fmt::{Display, Debug};

#[path = "./arguments.rs"] mod arguments;

pub fn debug(expr: &impl Debug) {
  if arguments::getArguments().verbose {
    println!("{:?}", expr);
  }
}

pub fn info(expr: impl Display) {
  if arguments::getArguments().verbose {
    println!("{}", expr);
  }
}

pub fn error(msg: String) {
  let error: ColoredString = "ERROR".to_string().red();
  if arguments::getArguments().verbose {
    println!("{} {}", error, msg.red());
  }
}

pub fn success(msg: String) {
  let success: ColoredString = "SUCCESS".to_string().green().bold();
  if arguments::getArguments().verbose {
    println!("{} {}", success, msg.green().bold());
  }
}

pub fn warn (msg: String) {
  let warning: ColoredString = "WARNING:".to_string().yellow().bold();
  if arguments::getArguments().verbose {
    println!("{} {}", warning, msg.yellow().bold())
  }
}