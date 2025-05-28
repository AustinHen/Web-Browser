use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::fs;
use std::sync::Arc;

use regex::Regex;

pub fn parse_doc(doc: &mut str) -> Option<Arc<DomNode>>{ 
    let doc = preprocess(doc);
    generate_dom_tree(doc.as_str())
}

fn preprocess(doc: &mut str) -> String{
    return remove_excess(&remove_doc_type(&remove_comments(doc))); 
}

fn remove_comments(doc: & str) -> String{
    let r = Regex::new(r"(<!--.*-->)").unwrap();
    let ret = r.replace_all(doc, "").into_owned();
    return ret;
}

fn remove_doc_type(doc: & str) -> String{
    let r = Regex::new(r"(<!DOCTYPE.*>)").unwrap();
    let ret = r.replace_all(doc, "<html>").into_owned();
    return ret;
}

//removes up to the first <  and everyting after the last >
fn remove_excess(doc: &str) -> String{
    let mut open_marker = 0; 
    for (i, val) in doc.chars().enumerate(){ 
        if val == '<'{
            open_marker = i;
            break;
        }
    }

    let mut close_marker = 0; 
    for (i, val) in doc.chars().rev().enumerate(){
        if val == '>'{
            close_marker = doc.len() - i;
            break
        }
    }

    if close_marker < open_marker{
        return doc.to_string();
    }
    
    //substrings are weird so ima just do this so no bug
    let mut ret = String::new();

    for (i, c) in doc.chars().enumerate(){
        if i >= open_marker && i <= close_marker{
            ret.push(c);
        }
    }
    ret
}

fn generate_dom_tree(preprocessed_doc: & str) -> Option<Arc<DomNode>>{
    let doc = preprocessed_doc; 
    println!("{0}", doc);
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
    /*tokenizes doc*/
    let mut stack : VecDeque<Arc<DomNode>> = VecDeque::new();
    let mut dom_head : Option<Arc<DomNode>> = None;
    for (idx, str_idx) in opens.iter().enumerate(){
        //ADDS TAG
        let substring: String = doc.chars().skip(opens[idx]).take(closes[idx] - str_idx + 1).collect();
        if let Some(TagCreateResult::Node(to_add)) = DomNode::get_tag(&substring){
            let to_add = Arc::new(to_add);
            if let Some(head) = stack.front(){
                //add to_add as child 
                head.children.borrow_mut().push(to_add.clone());
            }
            
            //makes to_add new head
            if ! DomNode::is_standalone_tag(&to_add.clone()){
                stack.push_front(to_add.clone());
            }

            //updates dom head TODO make better  
            if idx == 0{
                println!("HERRRRRRRRRRRRRRRRRRRRRREEEEEEEEEEEEEEEEEEEEEEEE");
                dom_head = Some(to_add);
            }
        }else{
            //prob close tag for most recent tag
            stack.pop_front(); 
        }

        //ADDS NON TAG
        if opens.len() <= idx+1{
            continue; // no non tag 
        }

        let substring: String = doc.chars().skip(closes[idx] + 1).take(opens[idx+1]-closes[idx]-1).collect();
        if let Some(to_add) = DomNode::get_non_tag(&substring){
            if let Some(head) = stack.front(){
                let to_add = Arc::new(to_add);
                head.children.borrow_mut().push(to_add);
            }            
        }

    }

    return dom_head; 
}

pub struct DomNode{
    pub tag_name: String, //TODO could make enum of types but idk yet 
    pub children: RefCell<Vec<Arc<DomNode>>>,
    pub data: RefCell<DomNodeData>,
}

pub enum DomNodeData{
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
        //thats one long line 
        return Some(TagCreateResult::Node(Self {tag_name: ret_tag_name, children: RefCell::new(Vec::new()), data: RefCell::new(DomNodeData::ValueMap(ret_values))}));
 
    }

    fn remove_special_chars(content: &str) -> String{
        let r = Regex::new(r"(&.*;)").unwrap();
        let ret = r.replace_all(content, "").into_owned();
        return ret;
    }

    fn get_non_tag(content: &str) -> Option<Self>{
        if content.trim().is_empty(){
            return None;
        }
        return Some(Self {tag_name: "content".to_string(), children: RefCell::new(Vec::new()), data: RefCell::new(DomNodeData::Content(Self::remove_special_chars(content.trim()).to_string()))});
    }

    fn is_standalone_tag(node: &Self) -> bool{
        //kinda just looks at a list of em 
        let self_closing_tags = vec![
            "area",
            "br",
            "col",
            "embed",
            "hr",
            "img",
            "input",
            "link",
            "meta",
            "source",
            "track",
            "wbr"
        ];
        // .contains() but worse
        for tag in self_closing_tags{
            if node.tag_name == tag{
                return true;
            }
        }
        false
    }
}

//TODO move to tests/debug file 
pub fn print_tree(head : Arc<DomNode>, depth : usize){
    for _ in 0..depth{
        print!("--");
    }

    if head.tag_name == "content"{
        print!("content: ");
        match *head.data.borrow(){
            DomNodeData::Content(ref i) => println!("{i}"),
            _ => println!("no content"),
        }
    }else{
        println!("{0}", head.tag_name);
    }

    //prints children
    for child in head.children.borrow().iter(){
        print_tree(child.clone(), depth + 1);
    }

}

enum TagCreateResult{
    ClosingTag, //for closing 
    Node(DomNode)
}



//--------------------REMOVE BELLOW---------------------//


pub fn test_parser(){
    /*reads in html files and passes them into parse_html_string*/
    print!("STARTING HTML TEST 1 \n");
    let string = fs::read_to_string("testHtmlFiles/ddghi.txt");
    match string {
       Ok(mut i) => parse_doc(&mut i),
       _ => panic!("could not open file")
    };

}
