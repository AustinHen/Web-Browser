use crate::http_get;
use native_tls::TlsConnector;
use std::io::prelude::*;
use std::net::TcpStream;
use std::collections::HashMap;

pub fn get_file(url: &str, file_name: &str) -> Option<HttpResponse>{
    //sets up tsl
    let connector = TlsConnector::new().unwrap(); 

    let request_msg = http_get!(url, file_name);
    let request_msg = request_msg.into_bytes();

    let Ok(stream) = TcpStream::connect(format!("{url}:443")) else {return None;};
    let Ok(mut stream) = connector.connect(url, stream) else {
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

    Some(HttpResponse::new(&read_bytes)) // TODO change ret
}


pub struct HttpResponse{
    pub response_code: usize,
    pub meta_data: HashMap<String, String>,
    pub body: String,
}

impl HttpResponse{
    pub fn new(bytes: &[u8]) -> Self{
        let mut cols_idxs : Vec<usize> = Vec::new();
        let mut nl_idxs : Vec<usize> = Vec::new();

        //finds idxs 
        for (i, val) in bytes.iter().enumerate(){
            if val == &(':' as u8){
                cols_idxs.push(i);
            }

            if val == &('\n' as u8) {
                nl_idxs.push(i);
            }
        }
        
        //we dont have a valid http msg -> can just say we got a 400
        if nl_idxs.len() == 0{
            return Self{ response_code: 400, meta_data: HashMap::new(), body: String::new()};
        }

        //get the response code out of the first line 
        for i in 0..nl_idxs[0]{
        }

        todo!();
    }
}
