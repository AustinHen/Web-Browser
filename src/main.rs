mod gui;
mod htmlParser;
mod network;
mod http_format;
mod url_format;

fn main() {
    println!("Booting up (what a great debug message) (useful newline)\n ");
    network::get_file("html.duckduckgo.com", "/html/?q=hi&redirected=1");
    //htmlParser::test_parser();
    //gui::gui_main();
}


