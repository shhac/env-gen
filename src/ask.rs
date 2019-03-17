extern crate dialoguer;

use dialoguer::{theme::ColorfulTheme, Select};
use super::question;

pub fn ask(question: question::Question) {
  println!("QUESTION");
  println!("{}", question);

  let get_answer_text = | ans: question::Answer | ans.text;
  let choices = question.answers.iter(get_answer_text).map().collect();

  let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(question.text)
        .default(0)
        .items(&choices[..])
        .interact()
        .unwrap();

}
