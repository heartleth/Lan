use std::collections::HashMap;
use super::assembling;

pub struct Voca<'s> {
    pub argv :Vec<&'s str>,
    pub text :Vec<char>
}

impl Voca<'_> {
    pub fn from<'s>(t :&'s str) -> Voca<'s> {
        let s :Vec<_> = t.split_whitespace().collect();
        Voca {
            argv: s[1..].to_vec(),
            text: assembling::disassemble(s[0])
        }
    }
}

pub type Dictionary<'s> = HashMap<&'s str, Vec<Voca<'s>>>;

pub fn load_dictionary<'s>(dict :&'s str) -> Dictionary<'s> {
    let mut dictionary = HashMap::new();
    let voca_list :Vec<_> = dict.split('\n')
        .map(|x| x.trim()).filter(|x| x.len() > 0 && (!x.starts_with("#"))).collect();
    let mut mode :&str = "";
    for t in voca_list {
        if t.starts_with("PART") {
            mode = &t[5..];
            dictionary.insert(&t.clone()[5..], Vec::new());
        }
        else {
            dictionary.get_mut(mode).unwrap().push(Voca::from(t.clone()));
        }
    }
    return dictionary;
}