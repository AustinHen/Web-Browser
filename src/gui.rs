use std::sync::mpsc;
use std::thread;
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
        Box::new(|cc| Ok(Box::new(BrowserApp::new(cc)))),
    );
}

struct BrowserApp{
    search_string: String, 
    dom_head: Option<Rc<DomNode>>,
    url_sender: mpsc::Sender<crate::url_format::Url>, 
    dom_head_rcvr: mpsc::Receiver<Rc<DomNode>>,
    network_thread : thread::JoinHandle<usize>,
}

impl eframe::App for BrowserApp{
    //calls every frame
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        //try update dom tree 
        egui::CentralPanel::default().show(&ctx, |ui| {
            self.get_search_bar(ui);
            egui::ScrollArea::vertical().show(ui, |ui| {
                self.walk_tree(ui);
            });
        });
    }
}

impl BrowserApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let (url_sender, url_rcvr) = mpsc::channel();
        let (dom_sender, dom_rcvr) = mpsc::channel();
        let network_thread = thread::spawn();
        BrowserApp{ 
            search_string: "".to_string(),
            dom_head: Some(get_default_dom_head()),
            url_sender, 
            dom_head_rcvr: dom_rcvr,
            network_thread,

        }
    }

    fn walk_tree(&mut self, ui: &mut egui::Ui){
        fn walk_tree_helper(cur_node: &Rc<DomNode>, ui: &mut egui::Ui, state: &mut HeadState){
            let prev_state = state.clone();
            if cur_node.tag_name == "content"{
                match *cur_node.data.borrow(){
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



#[derive(Clone)]
struct HeadState{
    font_size: usize,
    link: Option<String>
}

impl HeadState{
    pub fn new() -> Self{
        //sets up network thread
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

//todo make macro do make pages for us 
fn get_err_dom_head(err_no: &str, redirect_str: &str) -> Rc<DomNode>{
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

    let redirect_url = Rc::new(DomNode{
        tag_name: "content".to_string(),
        children: RefCell::new(Vec::new()),
        data: RefCell::new(DomNodeData::Content(format!("go to: {redirect_str}"))),
    });

    let mut value_map : HashMap<String, String> = HashMap::new();
    value_map.insert(String::from("href"), String::from("redirect_str"));

    let redirect_url_a_tag = Rc::new(DomNode{
        tag_name: "a".to_string(),
        children: RefCell::new(vec!(redirect_url)),
        data: RefCell::new(DomNodeData::ValueMap(value_map)),
    });

    Rc::new(DomNode{
        tag_name: "html".to_string(),
        children: RefCell::new(vec![title_head, more_text, redirect_url_a_tag]),
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


