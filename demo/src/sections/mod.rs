mod help;
mod extra;
pub mod learn;
pub mod practice;

use egui::{Image, ImageButton, Ui};

pub fn clickable_icon(ui: &mut Ui,title:&str,img:&Image<'_>,f:impl FnOnce()) {
    let button = ImageButton::new(img.clone());

    ui.vertical_centered(|ui| {
        if ui.add(button).clicked(){
            f();
        }
        ui.label(title);
    });
}

pub fn text_list_item(ui: &mut Ui,s:&str){
    ui.horizontal(|ui| {
        ui.label("-");
        ui.label(s);
    });
}

pub fn list_item(ui: &mut Ui,add_content:impl FnOnce(&mut Ui)){
    ui.horizontal(|ui| {
        ui.label("-");
        add_content(ui);
    });
}

pub fn list(ui: &mut Ui,id:&str, add_content:impl FnOnce(&mut Ui)){
    ui.indent(id, |ui|{
        // inline_text(ui,|ui| add_content(ui));
        add_content(ui);
    });
}

pub fn inline_text(ui: &mut Ui,add_contents:impl FnOnce(&mut Ui)){
    // ui.with_layout(egui::Layout::, add_contents)
    ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
        ui.horizontal_wrapped(|ui| {
            add_contents(ui);
        });
    });
}

pub trait ListItem{
    fn add_to_list(self,ui:&mut Ui);
}

pub trait UiList{
    fn list(&mut self,id:&str,add_contents:impl FnOnce(&mut Ui));

    fn li(&mut self,add_contents:impl ListItem);
}

impl UiList for Ui {
    fn list(&mut self,id:&str,add_contents:impl FnOnce(&mut Ui)) {
        self.indent(id, |ui| {
            add_contents(ui);
        });
    }

    fn li(&mut self,add_contents:impl ListItem) {
        add_contents.add_to_list(self);
    }
}

impl ListItem for &str {
    fn add_to_list(self,ui:&mut Ui) {
        ui.horizontal(|ui| {
            ui.label("-");
            ui.label(self);
        });
    }
}

// pub fn clickable_icon(ui: &mut Ui,title:&str,img:&Image<'_>) {
//     let button = ImageButton::new(img.clone());

//     ui.vertical(|ui| {
//         if ui.add(button).clicked(){
//             f();
//         }
//         ui.label(title);
//     });
// }