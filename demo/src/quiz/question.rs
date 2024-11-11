#[derive(serde::Deserialize, serde::Serialize,PartialEq,Eq)]
pub enum AnswerKey {
    Single(String),
    Multi(Vec<char>),
    MultiRequired(Vec<char>)
}


#[derive(serde::Deserialize, serde::Serialize,PartialEq,Eq)]
pub enum QuestionType{
    MultipleChoice{correct_answer:AnswerKey,answers:Vec<String>},
    SingleAnswer{correct_answer:String,answer: String}
}

impl QuestionType {
    pub fn mark(&self,response:AnswerKey) -> i32 {
        match (self,response) {
            (Self::SingleAnswer {correct_answer,..},AnswerKey::Single(given)) => if *correct_answer == given {1} else {0},
            
            (Self::MultipleChoice {correct_answer:AnswerKey::Multi(expected),..},AnswerKey::Multi(given)) => if expected.iter().zip(given).any(|(&a,b)| a == b) {1} else {0},

            (Self::MultipleChoice {correct_answer:AnswerKey::MultiRequired(expected),..},AnswerKey::MultiRequired(given)) => if expected.iter().zip(given).all(|(&a,b)| a == b) {1} else {0},
            _ => unreachable!()
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Question {
    q_type: QuestionType,
    description:String
}

impl Question {
    pub fn mark(&self,response:AnswerKey) -> i32 {
        self.q_type
    }
}

pub struct QuizResponse {
    answers: Vec<AnswerKey>
}