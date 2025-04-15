mod gui;
mod htmlParser;
mod network;

fn main() {
    println!("Booting up (what a great debug message) (useful newline)\n ");
    htmlParser::test_parser();
    gui::gui_main();
}


