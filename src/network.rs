use crate::http_get;
use crate::url_format::Url; 
use native_tls::TlsConnector;
use std::io::prelude::*; //prob could cut down this import 
use std::net::TcpStream;
use std::collections::HashMap;
use std::sync::mpsc;

pub fn network_main(url_rcvr: mpsc::Receiver<Url>, dom_sender: mpsc::Sender<String>){
    loop{
        //blocking wait for url
        let url = url_rcvr.recv().unwrap();
        url.print();
        
        //sends response when we get em
        if let Some(res) = get_file(&url){
            println!("reponse code: {0}", res.response_code);
            if res.response_code == 200{ 
                //can just send it body
                println!("sent a body");
                dom_sender.send(res.body).unwrap();
            }else{
                //hit an err -> show the err page
                let redirect_str = res.meta_data.get("redirect");
                let search_string = Some(url.raw);
                dom_sender.send(crate::gui::get_err_html(&format!("{0}", res.response_code), redirect_str, search_string)).unwrap();
            }

        }else{
            //no server so just duckduckgo it 
            let to_search = Url::new(crate::url_format::format_duckduckgo_search(&url.raw));
            if let Some(res) = get_file(&to_search){
                dom_sender.send(res.body).unwrap();
            }else{
                //prob bad to panic
                panic!("merp duck duck go not working");
            }
            
        }
    }
}

//sends a http/https get msg and returns the server's response
pub fn get_file(url: &Url) -> Option<HttpResponse>{
    //sets up tsl
    let connector = TlsConnector::new().unwrap(); 

    let request_msg = http_get!(&url.addr, &url.path);
    let request_msg = request_msg.into_bytes();

    let port_num = if url.protocol == "https" {"443"} else {"80"}; //default to http

    let Ok(stream) = TcpStream::connect(format!("{0}:{port_num}", url.addr)) else {return None;};
    let Ok(mut stream) = connector.connect(&url.addr, stream) else {
        println!("here");
        return None;};
    
    //send get msg
    if stream.write_all(&request_msg).is_err() {
        return None; // prob more rust acceptable way to do this 
    }

    //read in file 
    let mut read_string = String::new();
    stream.read_to_string(&mut read_string).unwrap();

    println!("{:?}", read_string);
    println!("------------------------------");
    HttpResponse::new(&read_string) 
}


pub struct HttpResponse{ //i hate pub
    pub response_code: usize,
    pub meta_data: HashMap<String, String>,
    pub body: String,
}

impl HttpResponse{
    pub fn new(read_string: &str) -> Option<Self>{
        //split body and header 
        let Some((header, body)) = read_string.split_once("\r\n\r\n") else {return None};

        let body = body.trim();

        //parse header 
        let header_lines : Vec<&str> = header.split("\n").collect();
        
        //get the response code out of the first line 
        let mut res_code : usize = 0;
        for char in header_lines[0].chars(){
            if char >= '0' && char <= '9'{
                res_code *= 10;
                res_code += (char as u8  - '0' as u8) as usize;  //c is so nice : no as nonsense 
            }
        }
        //hacky fix  (removes the 1.1 bit) 
        res_code -= 11000;

        //get all meta data 
        let mut meta_data : HashMap<String, String> = HashMap::new();

        for line in header_lines[1..].iter(){
            let Some((title, val)) = line.split_once("") else {
                continue;
            };

            let title = title.trim();
            let val = val.trim();

            meta_data.insert(title.to_string(), val.to_string());

        }

        return Some(Self{response_code: res_code, meta_data , body: body.to_string()});
    }
}
