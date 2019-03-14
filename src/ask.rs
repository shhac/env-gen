extern crate dialoguer;

use dialoguer::{theme::ColorfulTheme, Select};

pub fn ask(question: &str) {
  println!("QUESTION");
  println!("{}", question);
}
