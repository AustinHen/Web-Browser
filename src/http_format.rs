//All http messages will be gets 
#[macro_export]
macro_rules! http_get {
    ($host: expr, $url: expr) => {
        {
            $crate::http_header!(
                "GET", 
                $url, 
                "HTTP/1.1",
                "Host", $host, 
                "User-Agent", "Mozilla/5.0 (Windows; U; Windows NT 6.1; en-US; rv:1.9.1.8) Gecko/20091102 Firefox/3.5.5", // just some random stuff
                "Accept", "text/html",
                "Accept-Language", "en-us",
                "Accept-Encoding", "deflate",
                "Accept-Charset", "utf-8",
                "Cache-Control", "no-cache",
                "Connection", "close"
                 )
        }
    };

}


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
            temp_str.push(' ');
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


// some mechanism to extract host / file path 
