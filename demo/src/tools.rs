use egui::Pos2;

pub struct Tools {
    position: Option<Pos2>
}

impl Tools {
    pub fn new() -> Self {
        Self {
            position: None
        }
    }

    pub fn is_active(&self) -> bool {
        self.position.is_some()
    }

    // pub fn render(&self)

    pub fn set_position(&mut self,position:Option<Pos2>) {
        self.position = position;
    }

    // pub fn clear_position(&mut self){
    //     self.position = None;
    // }
}