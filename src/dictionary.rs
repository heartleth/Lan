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