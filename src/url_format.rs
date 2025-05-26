pub struct Url{
    pub raw: String,
    pub protocol: String, //defaults to http : redirect will tell us if it is https
    pub addr: String,
    pub path: String,
}

impl Url{
    pub fn new(raw: String) -> Url{
        let raw = raw.trim(); 

        let split : Vec<&str> = raw.split("://").collect();

        let mut protocol = String::from("http");
        if split.len() > 1{
            //the first part is the protocol
            protocol = split[0].to_string();
        }

        let to_split = if split.len() == 1 { split[0] } else { split[1] };

        if let Some((addr, path)) = to_split.split_once('/'){
            return Self {
                raw: raw.to_string(),
                protocol: protocol.trim().to_string(),
                addr: addr.trim().to_string(),
                path: format!("/{0}", path.trim())
            } 
        }

        return Self {
            raw: raw.to_string(),
            protocol,
            addr: to_split.to_string(),
            path: String::from("/"),
        } 
    }

    pub fn print(&self) {
        println!("raw: {0} \n protocol: {1} \n addr: {2} \n path: {3}", self.raw, self.protocol, self.addr, self.path);
    }
}

pub fn format_duckduckgo_search(search_string: &str) -> String{
    let mut ret = String::from("https://html.duckduckgo.com/html?q=");
    ret.push_str(&format_uri(search_string));
    ret
}

pub fn format_uri (s: &str) -> String{
    let mut ret = String::new();
    //uri rules:
    // - if A-Z a-z 0-9 - _ . ! ~ * ' ( ) leave unchanged
    // - if space return + 
    // - else return &(hex code) 
    for c in s.chars(){
        let is_encoded = (c as u8 >= 'A' as u8 && c as u8 <= 'Z' as u8) ||
                        (c as u8 >= 'a' as u8 && c as u8 <= 'z' as u8) ||
                        (c as u8 >= '0' as u8 && c as u8 <= '9' as u8) || 
                        c == '-' || c == '_' || c == '.' || c == '!' || c == '~' ||
                        c == '*' || c == '\'' || c == '(' || c == ')' ||
                        c=='8';  //teheheheehheehhehehehehehehehehehehehe
        if is_encoded{
            ret.push(c);
        } else if c == ' '{
            ret.push('+'); 
        } else {
            ret.push('&');
            ret.push_str(&format!("{:X}", c as u8)); //converts to hex
        }
    }
    ret
}
