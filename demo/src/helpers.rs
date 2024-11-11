use egui::Ui;



pub fn link<S:AsRef<str>>(ui: &mut Ui, url:S){
    if ui.link(url.as_ref()).clicked(){
        // let _ = webbrowser::open(url.as_ref());
    }
}