

use egui::{Context,include_image,Vec2};

#[derive(serde::Deserialize, serde::Serialize,Clone)]
pub enum FileIcon{
    Path(String),
    Text,
    Folder
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct File {
    icon: FileIcon,
    name: String,
}

impl File {
    pub fn new<S:Into<String>>(icon:FileIcon,name: S) -> Self {
        File {
            icon,
            name:name.into()
        }
    }

    pub fn render(&self, ctx:&Context){
        egui::Area::new(egui::Id::new(self.name.as_str())).show(ctx, |ui| {
            let img = match self.icon.clone() {
                FileIcon::Path(icon_path) => egui::Image::new(icon_path),
                FileIcon::Text => egui::Image::new(include_image!("../assets/txt.png")),
                _=> unimplemented!()
            }.fit_to_exact_size(Vec2 { x: 100.0, y: 100.0 });
            ui.add(img);
            ui.label(self.name.as_str());
        });
    }

    pub fn update(&mut self,ctx: &Context) {
        egui::Area::new(egui::Id::new(self.name.as_str())).show(ctx, |ui| {
            let img = match self.icon.clone() {
                FileIcon::Path(icon_path) => egui::Image::new(icon_path),
                FileIcon::Text => egui::Image::new(include_image!("../assets/txt.png")),
                _=> unimplemented!()
            }.fit_to_exact_size(Vec2 { x: 100.0, y: 100.0 });
            ui.add(img);
            ui.add_sized(Vec2{x:100.0,y:20.0}, egui::TextEdit::singleline(&mut self.name));
        });
    }
}