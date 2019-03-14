pub struct Answer {
  default: bool,
  text: String,
  value: String,
}

pub struct Question {
  question: String,
  name: String,
  answers: Answers,
}

pub type Answers = Vec<Answer>;
pub type Questions = LinkedHashMap<String, Question>;

pub fn toVariable(question: Question, answerIndex: u8) -> String {
  let numAnswers usize = question.answers.len();
  if (numAnswers == 0 || answerIndex > numAnswers) {
    return question.name + "=";
  }
  if (answerIndex == 0) {
    let answer_itr = question.answers.iter();
    let ans: Answer = answer_itr.find(|&ans| ans.default == true);
    return question.name + "=" + ans.value;
  }
  return question.name + "=" + question.answers[answerIndex].value;
}
