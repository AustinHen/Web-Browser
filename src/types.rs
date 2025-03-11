//all html types 
// hh
pub enum TagType{
    NoOp,
    PairTag(PairTag),
    SoloTags(SoloTag)

}

pub enum PairTag{
    A,
    B,
}


pub enum SoloTag{

}

pub fn string_to_tagtype(tag_str: &str) -> TagType{
    match tag_str{
        "a" => TagType::PairTag(PairTag::A),
        "b" => TagType::PairTag(PairTag::B),
        _ => {TagType::NoOp}
    }
}

