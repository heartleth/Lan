pub mod dictionary;
pub mod assembling;
pub mod lanparser;
pub mod concrete;
pub mod parse;

use assembling::assemble;
use dictionary::{ * };

fn main() {
    let lcs_bro = vec![
        std::fs::read_to_string("lans_brother/main.lan").unwrap()
    ];
    let hm_bro = lanparser::load_lan(&lcs_bro).unwrap();
    
    let lcs_chemistry = vec![
        std::fs::read_to_string("lans_chemistry/main.lan").unwrap()
    ];
    let hm_chemistry = lanparser::load_lan(&lcs_chemistry).unwrap();
    
    let lc = &std::fs::read_to_string("dictionary.dic").unwrap()[..];
    let dictionary = load_dictionary(lc);
    
    let t = std::fs::read_to_string("sentences_bro.txt").unwrap();
    for text in t.split("\n") {
        println!("{}", text);
        let dtext = assembling::disassemble(text);
        
        let k = parse::parse(&dtext, hm_bro["main"].build(&Vec::new()), &hm_bro, &dictionary);
        for morphome in &k.unwrap().0 {
            println!("'{}' '{}'", assemble(&morphome.text), morphome.name);
        }
    }

    let t = std::fs::read_to_string("sentences_chemistry.txt").unwrap();
    for text in t.split("\n") {
        println!("{}", text);
        let dtext = assembling::disassemble(text);
        
        let k = parse::parse(&dtext, hm_chemistry["main"].build(&Vec::new()), &hm_chemistry, &dictionary);
        for morphome in &k.unwrap().0 {
            println!("'{}' '{}'", assemble(&morphome.text), morphome.name);
        }
    }
}