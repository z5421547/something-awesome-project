use egui::{include_image, Button, ImageSource, Pos2, Sense, TextBuffer, Vec2};
use crate::journal::{Day, Page, PageMode};

/*

*/

pub fn render_scenario(ctx: &egui::Context) {

    let window = egui::Window::new("Journal")
    .default_size(Vec2{x:300.0,y:500.0})
    .min_size(Vec2{x:300.0,y:500.0})
    .resizable(true);

    let title = "example";
    let content = "test";
    let (day,month,year,day_of_week) = (1,1,1,Day::Monday);

    window
    // .open(&mut self.open_windows.journal)
    .show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.strong(title);
            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                ui.label(format!("{} - {day}/{month}/{year}",day_of_week.as_str()));
            });
        });
        ui.separator();
        ui.add_space(10.0);
        ui.label(content);
        // ui.horizontal(|ui| {
        //     if ui.add(egui::ImageButton::new(include_image!("../assets/left_arrow.png"))).clicked() {
        //         self.journal.prev_page();
        //     }
        //     ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
        //         if ui.add(egui::ImageButton::new(include_image!("../assets/right_arrow.png"))).clicked(){
        //             self.journal.next_page();
        //         }
        //     });
        // });
    });

    // if let Some(Page{title,content,date:(day,month,year),day_of_week,mode}) = self.journal.get_current_page() {
    //     let content = match mode {
    //         PageMode::Immediate => content,
    //         PageMode::Lazy(x) => {
    //             self.journal.update();
    //             content[..x/5].into()
    //         },
    //         PageMode::Loaded => content
    //     };

    //     window
    //     .open(&mut self.open_windows.journal)
    //     .show(ctx, |ui| {
    //         ui.horizontal(|ui| {
    //             ui.strong(title);
    //             ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
    //                 ui.label(format!("{} - {day}/{month}/{year}",day_of_week.as_str()));
    //             });
    //         });
    //         ui.separator();
    //         ui.add_space(10.0);
    //         ui.label(content);
    //         ui.horizontal(|ui| {
    //             if ui.add(egui::ImageButton::new(include_image!("../assets/left_arrow.png"))).clicked() {
    //                 self.journal.prev_page();
    //             }
    //             ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
    //                 if ui.add(egui::ImageButton::new(include_image!("../assets/right_arrow.png"))).clicked(){
    //                     self.journal.next_page();
    //                 }
    //             });
    //         });
    //     });
    // } else {
    //     window.open(&mut self.open_windows.journal)
    //     .show(ctx,|ui| {
    //         ui.label("");
    //     });
    // }
}
