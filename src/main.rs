mod gui;
mod htmlParser;

fn main() {
    println!("Booting up (what a great debug message) (useful newline)\n ");
    htmlParser::test_parser();
    gui::gui_main();
}

