use eframe;
use std::rc::Rc;
use crate::htmlParser::*;

//REMOVE THIS IMORT AFTER TESTING
use std::fs;

struct BrowserApp{
    search_string: String, 
    dom_head: Option<Rc<DomNode>>,
}

impl BrowserApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        BrowserApp{ 
            search_string: "".to_string(),
            dom_head: None
        }
    }

    pub fn temp_new(_cc: &eframe::CreationContext<'_>) -> Self {
        let string = fs::read_to_string("testHtmlFiles/htmltest1.txt");
        let head = match string {
            Ok(mut i) => parse_doc(&mut i),
            _ => panic!("could not open file")
        };

        BrowserApp{
            search_string: "".to_string(),
            dom_head: head
        }
    }
    
    fn walk_tree(&mut self, ui: &mut egui::Ui){
        //TODO make loop
        fn walk_tree_helper(cur_node: &Rc<DomNode>, ui: &mut egui::Ui){
            if cur_node.tag_name == "content"{
                match *cur_node.data.borrow(){
                    DomNodeData::Content(ref i) => {ui.label(format!("{i}"));},
                    _ => println!("no content"),
                }
            }

            for child in cur_node.children.borrow().iter(){
                walk_tree_helper(child, ui);
            }

        }

        if let Some(ref head) = self.dom_head{
            walk_tree_helper(head, ui);
        }
          
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
            self.walk_tree(ui);
        });

    }

}


pub fn gui_main() {
    let _ = eframe::run_native(
        "eframe template",
        eframe::NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(BrowserApp::temp_new(cc)))),
    );
}


