use crate::http_get;
use crate::url_format::Url; 
use native_tls::TlsConnector;
use std::io::prelude::*; //prob could cut down this import 
use std::net::TcpStream;
use std::collections::HashMap;
use std::sync::mpsc;
use std::rc::Rc;

pub fn network_main(url_rcvr: mpsc::Receiver<Url>, dom_sender: mpsc::Sender<Rc<crate::htmlParser::DomNode>>){
    loop{
        //blocking wait for url
        let url = url_rcvr.recv().unwrap();
        //sends response when we get em
        if let Some(mut res) = get_file(url){
            
            crate::htmlParser::parse_doc(&mut res.body);
        }else{
            //had an err prob redirect to search

        }
    }
}

//formats the search string and calls get file to preform a duckduckgo search
pub fn duckduckgo_search(search_string) -> Option<HttpResponse>{
    todo!();
}

//sends a http/https get msg and returns the server's response
pub fn get_file(url: Url) -> Option<HttpResponse>{
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
    let mut read_bytes = Vec::new();

    loop {
        let mut temp_buff = [0; 128];
        if let Ok(len) = stream.read(& mut temp_buff){ 
            if len == 0 {
                break;
            }

            read_bytes.extend_from_slice(&temp_buff[0..len as usize]);
        }else{
            println!("err when reading file");
            break;
        }
    }
    println!("{0}", String::from_utf8(read_bytes.clone()).unwrap());

    HttpResponse::new(&read_bytes) 
}


pub struct HttpResponse{ //i hate pub
    pub response_code: usize,
    pub meta_data: HashMap<String, String>,
    pub body: String,
}

impl HttpResponse{
    pub fn new(bytes: &[u8]) -> Option<Self>{
        //split body and header 
        let bytes = bytes.to_vec();
        let bytes = String::from_utf8(bytes).unwrap();
        let Some((header, body)) = bytes.split_once("\r\n\r\n") else {return None};

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
