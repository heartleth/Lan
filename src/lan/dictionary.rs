use std::collections::HashMap;
use super::assembling;
use std::rc::Rc;

#[derive(Clone)]
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

pub struct VocaRc {
    pub argv :Vec<Rc<str>>,
    pub text :Vec<char>
}
pub type DictionaryRc = HashMap<String, Vec<VocaRc>>;

impl VocaRc {
    pub fn from<'s>(t :Rc<str>) -> VocaRc {
        let s :Vec<Rc<str>> = t.split_whitespace().map(Rc::from).collect();
        VocaRc {
            argv: s[1..].to_vec(),
            text: assembling::disassemble(&s[0])
        }
    }
}

pub fn load_dictionaryrc(dict :&str) -> DictionaryRc {
    let mut dictionary = HashMap::new();
    let voca_list :Vec<_> = dict.split('\n')
    .map(|x| x.trim()).filter(|x| x.len() > 0 && (!x.starts_with("#"))).collect();
    let mut mode :&str = "";
    for t in voca_list {
        if t.starts_with("PART") {
            mode = &t[5..];
            dictionary.insert(String::from(&t[5..]), Vec::new());
        }
        else {
            dictionary.get_mut(&mode.to_string()).unwrap().push(VocaRc::from(Rc::from(t)));
        }
    }
    return dictionary;
}

pub fn add_dictionaryrc(target :&mut DictionaryRc, dict :&str) {
    let voca_list :Vec<_> = dict.split('\n')
    .map(|x| x.trim()).filter(|x| x.len() > 0 && (!x.starts_with("#"))).collect();
    let mut mode :&str = "";
    for t in voca_list {
        if t.starts_with("PART") {
            mode = &t[5..];
            target.insert(String::from(&t[5..]), Vec::new());
        }
        else {
            target.get_mut(&mode.to_string()).unwrap().push(VocaRc::from(Rc::from(t)));
        }
    }
}

pub fn dictrc_to_dict<'d>(dicrc :&'d DictionaryRc) -> Dictionary<'d> {
    let mut dictionary = HashMap::new();
    for (k, v) in dicrc {
        let vocas = v.iter().map(|e| Voca {
            argv: e.argv.iter().map(|e| e.as_ref()).collect(),
            text: e.text.clone()
        }).collect();
        dictionary.insert(&k[..], vocas);
    }
    dictionary
}