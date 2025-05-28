//calls http_header with some values filled in 
#[macro_export]
macro_rules! http_get {
    ($host: expr, $url: expr) => {
        {
            $crate::http_header!(
                "GET", 
                $url, 
                "HTTP/1.1",
                "Host", $host, 
                "User-Agent", "Mozilla/4.0 (compatible; MSIE 8.0; Windows NT 6.0; Trident/4.0)", // just some random stuff
                "Connection", "close" 
                 )
        }
    };

}

//used to constructed a header for a http msg 
#[macro_export]
macro_rules! http_header {
    ($msg_type : expr, $url : expr, $version : expr, $($name : expr, $val : expr ),*) => {
        {
            //TODO do better 
            //idk what the best way to concat strings is in rust 
            let mut temp_str = String::new();
            temp_str.push_str($msg_type);
            temp_str.push(' ');
            temp_str.push_str($url);
            temp_str.push(' ');
            temp_str.push_str($version);
            temp_str.push_str("\r\n");

            $(
                temp_str.push_str($name);
                temp_str.push(':');
                temp_str.push(' ');
                temp_str.push_str($val);
                temp_str.push_str("\r\n");
             )*
            temp_str.push_str("\r\n");
        
            temp_str
        }
    };
}
