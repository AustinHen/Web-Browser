mod gui; 
use regex::Regex;
use std::fs;
use std::rc::Rc;
use std::collections::HashMap;
use std::collections::VecDeque;

fn main() {
    println!("Booting up (what a great debug message) (useful newline)\n ");
    //test_parser();
}

fn parse_doc(doc: &mut str){ //TODO figure out how to take ownership
                             //TODO look into cfg
    let doc = preprocess(doc);
    let dom_head = generate_dom_tree(doc.as_str());
}

fn preprocess(doc: &mut str) -> String{
   return remove_comments(doc); 
   //maybe do somethin else like pre fetch content in srcs 
}

fn remove_comments(doc: & str) -> String{
    let r = Regex::new(r"(<!--.*-->)").unwrap();
    let ret = r.replace_all(doc, "").into_owned();
    return ret;
}

fn generate_dom_tree(preprocessed_doc: & str) -> Option<DomNode>{
    //TODO maybe update to call preprocesser 
    let doc = preprocessed_doc; //just easier name to work with
    let mut opens : Vec<usize> = vec![];
    let mut closes : Vec<usize> = vec![];

    for (idx, char) in doc.chars().enumerate(){
        if char == '<'{
            opens.push(idx);
        }
        if char == '>'{
            closes.push(idx);
        }
    }

    //TODO handle this better -> Prob should not throw an error just return somthin 
    assert!(opens.len() == closes.len(), "there should be same number of open and closing brackets"); 
    
    //anything between a open and a close is an iner tag
    //anything between a close and an open is out of a tag 
    //TODO fix comments 
    /*tokenizes doc*/
    let mut stack : VecDeque<Rc<DomNode>> = VecDeque::new();
    let mut head : Option<Rc<DomNode>> = None;
    for (idx, str_idx) in opens.iter().enumerate(){
        let substring: String = doc.chars().skip(opens[idx]).take(closes[idx] - str_idx + 1).collect();
        //println!("{substring}");
        //process_tag(&substring);

        if opens.len() > idx+1{
            //process_non_tag(doc, closes[idx], opens[idx+1]);
        }

    }

    todo!();
}

struct DomNode{
    tag_name: String, //TODO could make enum of types but idk yet 
    children: Vec<Rc<DomNode>>,
    data: DomNodeData,
}

enum DomNodeData{
    Content(String),
    ValueMap(HashMap<String, String>),
}

impl DomNode{
    fn get_tag(tag_content: &str) -> Option<TagCreateResult>{
        let tag_type_regex = Regex::new(r"<\s*(/?)\s*(\w+)\s*").unwrap(); 
        let feild_value_pair_regex = Regex::new(r#"\s*(\w*)\s*=\s*\"([^\"]*)\"\s*"#).unwrap();

        let ret_tag_name: String; //TODO remove all of this so we dont need to randomly allocate
                                   //more space
        let mut ret_values: HashMap<String, String> = HashMap::new();

        if let Some(i) = tag_type_regex.captures_iter(tag_content).next(){ 
            ret_tag_name = i[2].to_string();
            
            if &i[1] == "/"{
                return Some(TagCreateResult::ClosingTag);//its a closing tag 
            }
        }else{
            return None;
        }

        for i in feild_value_pair_regex.captures_iter(tag_content){
            ret_values.insert(i[1].to_string(), i[2].to_string());
        }
        //TODO break up into more lines
        return Some(TagCreateResult::Node(Self {tag_name: ret_tag_name, children: Vec::new(), data: DomNodeData::ValueMap(ret_values)}));
 
    }

    fn get_non_tag(content: &str) -> Option<Self>{
        if content.trim().is_empty(){
            return None;
        }
        return Some(Self {tag_name: "content".to_string(), children: Vec::new(), data: DomNodeData::Content(content.to_string())});
    }

    fn is_standalone_tag() -> bool{
        todo!();
    }
}

enum TagCreateResult{
    ClosingTag, //for closing 
    Node(DomNode)
}



//--------------------REMOVE BELLOW---------------------//

/*Converts html into a dom tree */
fn parse_html_string(doc: &str){
    let temp = remove_comments(&doc);
    let doc = temp.as_str();

    let mut opens : Vec<usize> = vec![];
    let mut closes : Vec<usize> = vec![];

    for (idx, char) in doc.chars().enumerate(){
        if char == '<'{
            opens.push(idx);
        }
        if char == '>'{
            closes.push(idx);
        }
    }

    //TODO handle this better -> Prob should not throw an error just return somthin 
    assert!(opens.len() == closes.len(), "there should be same number of open and closing brackets"); 
    
    //anything between a open and a close is an iner tag
    //anything between a close and an open is out of a tag 
    //TODO fix comments 
    /*tokenizes doc*/
    for (idx, str_idx) in opens.iter().enumerate(){
        let substring: String = doc.chars().skip(opens[idx]).take(closes[idx] - str_idx + 1).collect();
        //println!("{substring}");
        //process_tag(&substring);

        if opens.len() > idx+1{
            //process_non_tag(doc, closes[idx], opens[idx+1]);
        }

    }
}


fn test_parser(){
    /*reads in html files and passes them into parse_html_string*/
    print!("STARTING HTML TEST 1 \n");
    let string = fs::read_to_string("testHtmlFiles/htmltest1.txt");
    match string {
       Ok(i) => parse_html_string(&i),
       _ => panic!("could not open file")
    };

}
