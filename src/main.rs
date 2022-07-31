pub mod dictionary;
pub mod assembling;
pub mod lanparser;
pub mod concrete;
pub mod parse;

use std::collections::HashMap;
use dictionary::{ * };

use assembling::assemble;

fn main() {
    let lcs = vec![
        std::fs::read_to_string("lans/main.lan").unwrap()
    ];
    let lines_c :Vec<_> = lcs.iter().map(|lc| lc.split('\n').map(|x| x.trim()).filter(|x| x.len() > 0 && (!x.starts_with("#"))).collect::<Vec<_>>()).collect();
    let lines = lines_c.concat();
    
    let lc = &std::fs::read_to_string("dictionary.dic").unwrap()[..];
    let voca_list :Vec<_> = lc.split('\n')
        .map(|x| x.trim()).filter(|x| x.len() > 0 && (!x.starts_with("#"))).collect();
    let mut dictionary :Dictionary = HashMap::new();
    let mut mode :&str = "";
    for t in voca_list {
        if t.starts_with("PART") {
            mode = &t[5..];
            dictionary.insert(&t[5..], Vec::new());
        }
        else {
            dictionary.get_mut(mode).unwrap().push(Voca::from(t));
        }
    }

    let hm = lanparser::parse(&lines).unwrap();
    
    let t = std::fs::read_to_string("sentences.txt").unwrap();
    for text in t.split("\n") {
        println!("{}", text);
        let dtext = assembling::disassemble(text);
        
        let k = parse::parse(&dtext, hm["main"].build(&Vec::new()), &hm, &dictionary);
        for morphome in &k.unwrap().0 {
            println!("'{}' '{}'", assemble(&morphome.text), morphome.name);
        }
    }
}