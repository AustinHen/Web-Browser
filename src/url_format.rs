struct Url{
    raw: String,
    protocol: String, //defaults to http : redirect will tell us if it is https
    addr: String,
    path: String,
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
                protocol,
                addr: addr.to_string(),
                path: format!("{0}/", path.to_string())
            } 
        }

        return Self {
            raw: raw.to_string(),
            protocol,
            addr: to_split.to_string(),
            path: String::from("/"),
        } 
    }
}
