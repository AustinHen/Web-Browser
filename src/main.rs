mod gui;
mod htmlParser;
mod network;
mod http_format;

fn main() {
    println!("Booting up (what a great debug message) (useful newline)\n ");
    println!("{0}", http_get!("youtube.com", "/video.html"));
    htmlParser::test_parser();
    gui::gui_main();
}


