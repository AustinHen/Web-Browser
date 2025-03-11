mod types;
use regex::Regex;
use std::fs;


fn main() {
    println!("Booting up (what a great debug message) (useful newline)\n ");
    test_parser();
}

fn parse_html_string(doc: &str){
    /*Converts html into a dom tree */
    //denote the indexes of all of the opening and closing brackets
    //I dont trust regex to be O of n for something this simple so im just doing it by hand 
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
        process_tag(&substring);

        if opens.len() > idx+1{
            process_non_tag(doc, closes[idx], opens[idx+1]);
        }

    }
}

fn process_tag(inner_tag: &str){
    let tag_type_regex = Regex::new(r"<\s*(/?)\s*(\w+)\s*").unwrap(); //TODO fix regex
    let feild_value_pair_regex = Regex::new(r#"\s*(\w*)\s*=\s*\"([^\"]*)\"\s*"#).unwrap();

    println!("inner tag: {inner_tag}");
    
    for i in tag_type_regex.captures_iter(inner_tag){
        println!("merp {:?}, {:?}", &i[1], &i[2]);
    }

    for i in feild_value_pair_regex.captures_iter(inner_tag){
        println!("values {:?}, {:?}", &i[1], &i[2]);
    }

}


fn process_non_tag(to_process: &str, start_idx: usize, close_idx: usize){
    //print!("inner: {0} \n", to_process);
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
