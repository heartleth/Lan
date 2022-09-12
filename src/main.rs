pub mod dictionary;
pub mod assembling;
pub mod lanparser;
pub mod concrete;
pub mod parse;

use std::collections::HashMap;
use dictionary::{ * };

// use assembling::assemble;

fn main() {
    let lcs_chemistry = vec![
        std::fs::read_to_string("lans_chemistry/main.lan").unwrap()
    ];
    let lines_c :Vec<_> = lcs_chemistry.iter().map(|lc| lc.split('\n').map(|x| x.trim()).filter(|x| x.len() > 0 && (!x.starts_with("#"))).collect::<Vec<_>>()).collect();
    let lines_chemistry = lines_c.concat();

    let lcs_tokipona = vec![
        std::fs::read_to_string("lans_tokipona/main.lan").unwrap()
    ];
    let lines_c :Vec<_> = lcs_tokipona.iter().map(|lc| lc.split('\n').map(|x| x.trim()).filter(|x| x.len() > 0 && (!x.starts_with("#"))).collect::<Vec<_>>()).collect();
    let lines_tokipona = lines_c.concat();
    
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

    let hm_chemistry = lanparser::parse(&lines_chemistry).unwrap();

    let t = std::fs::read_to_string("sentences_chemistry.txt").unwrap();
    for text in t.split("\n") {
        println!("{}", text);
        let dtext = assembling::disassemble(text);
        
        let k = parse::parse(&dtext, hm_chemistry["main"].build(&Vec::new()), &hm_chemistry, &dictionary);
        println!("{}\n", k.unwrap().0.collect_verbose());
    }

    let hm_tokipona = lanparser::parse(&lines_tokipona).unwrap();

    let t = std::fs::read_to_string("sentences_tokipona.txt").unwrap();
    for text in t.split("\n") {
        println!("{}", text);
        let dtext = assembling::disassemble(text);
        
        let k = parse::parse(&dtext, hm_tokipona["main"].build(&Vec::new()), &hm_tokipona, &dictionary);
        println!("{}\n", k.unwrap().0.collect_verbose());
    }
}