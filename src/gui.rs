use eframe;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use crate::htmlParser::*;
//REMOVE THIS IMPORT AFTER TESTING
use std::fs;

pub fn gui_main() {
    let _ = eframe::run_native(
        "eframe template",
        eframe::NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(BrowserApp::temp_new(cc)))),
    );
}

struct BrowserApp{
    search_string: String, 
    dom_head: Option<Rc<DomNode>>,
}

#[derive(Clone)]
struct HeadState{
    font_size: usize,
    link: Option<String>
}

impl HeadState{
    pub fn new() -> Self{
        HeadState {
            font_size: 12,
            link: None
        }
    }

    pub fn update_state(&mut self, node: &Rc<DomNode>){
        const FONT_STEP_SIZE: usize = 2;
        match node.tag_name.as_str(){
            //fight me 
            "h1" => {self.font_size = 12 + FONT_STEP_SIZE*5},
            "h2" => {self.font_size = 12 + FONT_STEP_SIZE*4},
            "h3" => {self.font_size = 12 + FONT_STEP_SIZE*3},
            "h4" => {self.font_size = 12 + FONT_STEP_SIZE*2},
            "h5" => {self.font_size = 12 + FONT_STEP_SIZE*1},
            "p" => {self.font_size = 12},
            "a" => {
                match *node.data.borrow(){
                    DomNodeData::ValueMap(ref map) => {
                        if let Some(link) = map.get("href"){
                            self.link = Some(link.clone());
                        }
                    },
                    _ => {}
                }
            },
            _ => {},
        };
    }
}

impl BrowserApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        BrowserApp{ 
            search_string: "".to_string(),
            dom_head: Some(get_default_dom_head())
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
        fn walk_tree_helper(cur_node: &Rc<DomNode>, ui: &mut egui::Ui, state: &mut HeadState){
            let prev_state = state.clone();
            if cur_node.tag_name == "content"{
                match *cur_node.data.borrow(){
                    //TODO make mult lines 
                    DomNodeData::Content(ref i) => {
                        let mut text = eframe::egui::RichText::new(format!("{i}")).font(eframe::egui::FontId::proportional(state.font_size as f32));
                        if let Some(ref link) = state.link{
                            text = text.underline();
                            text = text.color(eframe::egui::Color32::LIGHT_BLUE);
                            if ui.label(text).clicked(){
                                println!("clicked {0}", link);
                            }
                        }else{
                            ui.label(text);
                        }

                        
                    },
                    _ => println!("no content"),
                }
            }else{
                //updates the state
                state.update_state(cur_node);
            }

            for child in cur_node.children.borrow().iter(){
                walk_tree_helper(child, ui, state);
            }
            state.font_size = prev_state.font_size;
            state.link = prev_state.link;

        }

        if let Some(ref head) = self.dom_head{
            walk_tree_helper(head, ui, &mut HeadState::new());
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
            egui::ScrollArea::vertical().show(ui, |ui| {
                self.walk_tree(ui);
            });
        });
    }
}

//todo make macro do make pages for us 
fn get_err_dom_head(err_no: &str) -> Rc<DomNode>{
    let title = Rc::new(DomNode{
        tag_name: "content".to_string(),
        children: RefCell::new(Vec::new()),
        data: RefCell::new(DomNodeData::Content("ERROR!!".to_string())),
    });

    let title_head = Rc::new(DomNode{
        tag_name: "h1".to_string(),
        children: RefCell::new(vec![title]),
        data: RefCell::new(DomNodeData::ValueMap(HashMap::new()))
    });

    let more_text = Rc::new(DomNode{
        tag_name: "content".to_string(),
        children: RefCell::new(Vec::new()),
        data: RefCell::new(DomNodeData::Content(format!("Errno:{err_no}"))),
    });

    Rc::new(DomNode{
        tag_name: "html".to_string(),
        children: RefCell::new(vec![title_head, more_text]),
        data: RefCell::new(DomNodeData::ValueMap(HashMap::new()))
    })
}

fn get_default_dom_head() -> Rc<DomNode>{
    let title = Rc::new(DomNode{
        tag_name: "content".to_string(),
        children: RefCell::new(Vec::new()),
        data: RefCell::new(DomNodeData::Content("Browser".to_string())),
    });

    let title_head = Rc::new(DomNode{
        tag_name: "h1".to_string(),
        children: RefCell::new(vec![title]),
        data: RefCell::new(DomNodeData::ValueMap(HashMap::new()))
    });

    let more_text = Rc::new(DomNode{
        tag_name: "content".to_string(),
        children: RefCell::new(Vec::new()),
        data: RefCell::new(DomNodeData::Content("the worlds best browser".to_string())),
    });

    Rc::new(DomNode{
        tag_name: "html".to_string(),
        children: RefCell::new(vec![title_head, more_text]),
        data: RefCell::new(DomNodeData::ValueMap(HashMap::new()))
    })
}


