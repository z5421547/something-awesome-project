mod question;
use question::*;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Quiz {
    title: String,
    description: Option<String>,
    questions: Vec<Question>
}

impl Quiz {
    pub fn mark(&self) -> i32 {
        todo!()
    }
}






pub fn render_quiz(ctx: &egui::Context) {
    todo!()
}