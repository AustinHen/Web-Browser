mod gui;
mod htmlParser;
mod network;
mod http_format;
mod url_format;


fn main() {
    //have three threads: main thread, gui thread and search thread -> 
    //gui thread will propigate search strings up to main and recive dom trees 
    //search thread will take search strings from main and pass up dom trees 
    //main will comunicate between the two
    
    


    println!("Booting up (what a great debug message) (useful newline)\n ");
    let search = url_format::Url::new(String::from("https://html.duckduckgo.com/html/"));

    network::get_file(search);
    //htmlParser::test_parser();
    //gui::gui_main();
    
    //TODO join threads 
}


