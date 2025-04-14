use eframe;

struct BrowserApp{
    search_string: String, 
}

impl BrowserApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        return BrowserApp{search_string: "".to_string()};
    }

    fn get_search_bar(&mut self, ui: &mut egui::Ui){
        let response = ui.add(eframe::egui::TextEdit::singleline(&mut self.search_string).desired_width(f32::INFINITY));

        if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
            //TODO handle searching 
        }
        ui.add_space(10.0);
    }
}

impl eframe::App for BrowserApp{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(&ctx, |ui| {
            self.get_search_bar(ui);
        });

    }

}


pub fn gui_main() {
    let _ = eframe::run_native(
        "eframe template",
        eframe::NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(BrowserApp::new(cc)))),
    );
}


