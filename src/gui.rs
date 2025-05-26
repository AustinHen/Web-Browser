use std::sync::mpsc;
use std::thread;
use eframe;
use std::cell::RefCell;
use std::sync::Arc;
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
    dom_head: Option<Arc<DomNode>>,
    url_sender: mpsc::Sender<crate::url_format::Url>, 
    body_rcvr: mpsc::Receiver<String>,
    network_thread : thread::JoinHandle<()>,
}

impl eframe::App for BrowserApp{
    //calls every frame
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        //try to fetch a new body from the network thread
        //im not 100% sure that try_recv will work here -> it may just grab stuff before it is
        //fully writen - if that turns out to be the case we can just use a timeout with like 0
        //seconds or somethin
        if let Ok(mut new_body) = self.body_rcvr.try_recv(){
            //have new body to parse / draw
            if let Some(new_dom_head) = parse_doc(&mut new_body){
                self.dom_head = Some(new_dom_head);
            }else{
                panic!("we have a body but no parse");
            }
        }

        //draw shit
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
        let (body_sender, body_rcvr) = mpsc::channel();
        let network_thread = thread::spawn(move || {crate::network::network_main(url_rcvr, body_sender)});
        BrowserApp{ 
            search_string: "".to_string(),
            dom_head: Some(get_default_dom_head()),
            url_sender, 
            body_rcvr,
            network_thread,

        }
    }

    fn walk_tree(&mut self, ui: &mut egui::Ui){
        if let Some(ref head) = self.dom_head{
            self.walk_tree_helper(&head.clone(), ui, &mut HeadState::new());
        }
          
    }
    fn walk_tree_helper(&mut self, cur_node: &Arc<DomNode>, ui: &mut egui::Ui, state: &mut HeadState){
        let prev_state = state.clone();
        if cur_node.tag_name == "content"{
            match *cur_node.data.borrow(){
                DomNodeData::Content(ref i) => {
                    let mut text = eframe::egui::RichText::new(format!("{i}")).font(eframe::egui::FontId::proportional(state.font_size as f32));
                    if let Some(ref link) = state.link{
                        text = text.underline();
                        text = text.color(eframe::egui::Color32::LIGHT_BLUE);
                        if ui.label(text).clicked(){
                            self.search_string = link.clone(); 
                            self.url_sender.send(crate::url_format::Url::new(self.search_string.clone())).unwrap();
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
            self.walk_tree_helper(child, ui, state);
        }
        state.font_size = prev_state.font_size;
        state.link = prev_state.link;

    }



    fn get_search_bar(&mut self, ui: &mut egui::Ui){
        let response = ui.add(eframe::egui::TextEdit::singleline(&mut self.search_string).desired_width(f32::INFINITY));

        if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
            self.url_sender.send(crate::url_format::Url::new(self.search_string.clone())).unwrap();
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

    pub fn update_state(&mut self, node: &Arc<DomNode>){
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

pub fn get_err_html(err_no: &str, redirect_url: Option<&String>, search_string: Option<String>) -> String{ 
    let mut ret = String::new();

    ret.push_str(&format!(r#"
                          <html> 
                          <body>
                          <h1>encountered an err | errno : {err_no}</h1>
                          "#));
    if let Some(redirect_url) = redirect_url{
        ret.push_str(&format!(r#"<a href = "{redirect_url}">redirected to: {redirect_url} </a>"#));
    }

    if let Some(search_string) = search_string{
        let ddg_url = crate::url_format::format_duckduckgo_search(&search_string);
        ret.push_str(&format!(r#"<a href = "{ddg_url}">redirected to: {search_string} </a>"#));
    }

    ret.push_str(" </body> </html> ");
    ret
}


fn get_default_dom_head() -> Arc<DomNode>{
    let title = Arc::new(DomNode{
        tag_name: "content".to_string(),
        children: RefCell::new(Vec::new()),
        data: RefCell::new(DomNodeData::Content("Browser".to_string())),
    });

    let title_head = Arc::new(DomNode{
        tag_name: "h1".to_string(),
        children: RefCell::new(vec![title]),
        data: RefCell::new(DomNodeData::ValueMap(HashMap::new()))
    });

    let more_text = Arc::new(DomNode{
        tag_name: "content".to_string(),
        children: RefCell::new(Vec::new()),
        data: RefCell::new(DomNodeData::Content("the worlds best browser".to_string())),
    });

    Arc::new(DomNode{
        tag_name: "html".to_string(),
        children: RefCell::new(vec![title_head, more_text]),
        data: RefCell::new(DomNodeData::ValueMap(HashMap::new()))
    })
}


