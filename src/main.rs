mod gui;
mod htmlParser;
mod network;
mod http_format;

fn main() {
    println!("Booting up (what a great debug message) (useful newline)\n ");
    network::get_file("httpbin.org", "/ip");
    //htmlParser::test_parser();
    //gui::gui_main();
}


