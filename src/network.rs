use crate::http_get;
use std::io::prelude::*;
use std::net::TcpStream;

//todo change oprion into result
pub fn get_file(url: &str, file_name: &str) -> Option<String>{
    let request_msg = http_get!(url, file_name);
    let request_msg = request_msg.into_bytes();
    let Ok(mut stream) = TcpStream::connect(format!("{url}:80")) else {return None;};

    //send get msg
    if stream.write(&request_msg).is_err() {
        return None; // prob more rust acceptable way to do this 
    }

    //read in file 
    let mut temp_buff = [0; 128];
    stream.read(& mut temp_buff);

    println!("{:?}", temp_buff);


    
    Some(String::new()) // TODO change ret
}

