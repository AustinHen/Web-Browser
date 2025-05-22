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
