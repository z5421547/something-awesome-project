use super::clickable_icon;
use crate::app::OpenWindows;
use egui::{include_image, Color32, Image, Ui, Vec2,RichText};

#[derive(Default)]
pub struct PracticeData {
    pub quizzes: Vec<Quiz>,
}

pub fn practice_button(ui: &mut Ui, open_windows: &mut OpenWindows) {
    let hat: Image<'_> = Image::new(include_image!("../../assets/puzzlePiece.jpeg"))
        .fit_to_exact_size(Vec2 { x: 100.0, y: 100.0 });
    clickable_icon(ui, "Practice", &hat, || {
        open_windows.practice_page = !open_windows.practice_page;
    });
}

pub fn practice_window(
    ui: &mut Ui,
    practice_data: &mut PracticeData,
    open_windows: &mut OpenWindows,
) {
    let window = egui::Window::new("Practice")
        .default_size(Vec2 { x: 450.0, y: 500.0 })
        .min_size(Vec2 { x: 450.0, y: 500.0 })
        .max_width(600.0)
        .scroll(true)
        .resizable(true);
    window
        .open(&mut open_windows.practice_page)
        .show(ui.ctx(), |ui| {
            for q in practice_data.quizzes.iter_mut() {
                quiz(ui, q);
            }
        });
}

#[derive(Clone)]
pub struct Question {
    description: String,
    answers: Vec<String>,
    correct_answer: char,
    chosen_answer: Option<char>,
}

impl Question {
    pub fn new(description: String, answers: Vec<String>, correct_answer: char) -> Self {
        Self {
            description,
            answers,
            correct_answer,
            chosen_answer: None,
        }
    }
}

#[derive(Clone)]
pub struct Quiz {
    title: String,
    questions: Vec<Question>,
    marked_title: String,
    is_completed: bool,
}

impl Quiz {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            questions: vec![],
            marked_title: "".into(),
            is_completed: false,
        }
    }

    pub fn add_question<S: ToString, S2: ToString>(
        mut self,
        description: S,
        answers: Vec<S2>,
        correct_answer: char,
    ) -> Self {
        self.questions.push(Question::new(
            description.to_string(),
            answers.into_iter().map(|a| a.to_string()).collect(),
            correct_answer,
        ));
        self
    }
}

pub fn mark_quiz(quiz: &Quiz) -> f64 {
    if quiz.questions.len() == 0 {
        return 0.0;
    }
    let correct_answer_percentage = quiz
    .questions
    .iter()
    .filter(|q| q.chosen_answer == Some(q.correct_answer))
    .count() as f64
    / quiz.questions.len() as f64;
    (correct_answer_percentage*100.0)
}

pub fn count_answers(quiz: &Quiz) -> usize {
    quiz.questions
        .iter()
        .filter(|q| q.chosen_answer != None)
        .count()
}

pub fn quiz(ui: &mut Ui, quiz: &mut Quiz) {
    let mut chosen_answer = (0, None);
    ui.collapsing(&quiz.title, |ui| {
        for (question, i) in quiz.questions.iter().zip(1..) {
            let q_num = "Question ".to_string() + i.to_string().as_str();
            ui.collapsing(q_num, |ui| {
                ui.label(RichText::new(question.description.as_str()).strong());
                for (answer, letter) in question.answers.iter().zip(["a", "b", "c", "d"]) {
                    ui.horizontal(|ui| {
                        if ui.button(letter).clicked() {
                            chosen_answer = (i-1, letter.chars().next());
                        }
                        ui.label(answer);
                    });
                }
                if let Some(chosen) = question.chosen_answer {
                    if chosen == question.correct_answer {
                        ui.colored_label(Color32::from_rgb(80, 180, 60), "Correct ✅");
                    } else {
                        ui.colored_label(Color32::from_rgb(220, 50, 30), "Incorrect ❌");
                    }
                }
            });
        }
    });
    if chosen_answer.1 != None{
        quiz.questions[chosen_answer.0].chosen_answer = chosen_answer.1;
        if !quiz.is_completed && count_answers(quiz) == quiz.questions.len() {
            quiz.is_completed = true;
            let mark = mark_quiz(quiz);
            quiz.title += format!(": {:.3}%",mark).as_str()
        }
    }
}
