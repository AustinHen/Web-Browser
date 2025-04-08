//all vars
struct RustUI{
    DomHead: String,

}

//all events possible
enum Message {
    GetPage(String),
    ATag(String),
    //maybe sometin to dynamically load in buttons 
}

//the code or somthin 
impl Sandbox for RustUI{
    type Message = Message;

    //app constructor 
    fn new() -> Self{
        //get all parts

    }

    fn title(&self) -> String{
        String::from("web browser 0.1")
    }

    fn update(&mut self, message: Message){
        match Message{
            _ => println!("merp")
        }
    }

    fn view(&self) -> Element<Message> {
    }
}


