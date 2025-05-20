use crate::http_get;
use std::io::prelude::*;
use std::net::TcpStream;
use std::collections::HashMap;

pub fn get_file(url: &str, file_name: &str) -> Option<HttpResponse>{
    let request_msg = http_get!(url, file_name);
    let request_msg = request_msg.into_bytes();
    let Ok(mut stream) = TcpStream::connect(format!("{url}:80")) else {return None;};

    //send get msg
    if stream.write(&request_msg).is_err() {
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
    println!("{:?}", read_bytes);

    Some(HttpResponse::new()) // TODO change ret
}


pub struct HttpResponse{
    pub response_code: usize,
    pub meta_data: HashMap<String, String>,
    pub body: String,
}

impl HttpResponse{
    pub fn new() -> Self{
        todo!();
    }
}
