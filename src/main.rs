mod gui;
mod htmlParser;
mod network;
mod http_format;
mod url_format;

fn main() {
    println!("Booting up (what a great debug message) (useful newline)\n ");
    let search = url_format::Url::new(String::from("https://html.duckduckgo.com/html/"));

    network::get_file(search);
    //htmlParser::test_parser();
    //gui::gui_main();
}


