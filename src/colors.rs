extern crate colored;
use colored::*;

// pub fn printer() {
//   println!(
//       "{}, {}, {}, {}, {}, {}, and some normal text.",
//       format!("Bold").bold(),
//       format!("Red").red(),
//       format!("Yellow").yellow(),
//       format!("Green Strikethrough").green().strikethrough(),
//       format!("Blue Underline").blue().underline(),
//       format!("Purple Italics").purple().italic()
//   );
// }

pub fn printRed(msg: String) {
  println!("{}", msg.red().bold());
}

pub fn printGreen(msg: String) {
  println!("{}", msg.green().bold());
}