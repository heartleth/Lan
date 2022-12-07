pub mod dictionary;
pub mod assembling;
pub mod lanparser;
pub mod concrete;
pub mod parse;

use std::time::{ Instant };
use dictionary::{ * };

fn main() {
    let lcs_chemistry = vec![ std::fs::read_to_string("lans_chemistry/main.lan").unwrap() ];
    let lcs_tokipona = vec![ std::fs::read_to_string("lans_tokipona/main.lan").unwrap() ];
    let lcs_korean = vec![ std::fs::read_to_string("lans_korean/main.lan").unwrap(),
        std::fs::read_to_string("lans_korean/agglutinate.lan").unwrap(),
        std::fs::read_to_string("lans_korean/substantives.lan").unwrap() ];
    
    let lc = &std::fs::read_to_string("dictionary.dic").unwrap()[..];
    let dictionary = load_dictionary(lc);

    let hm_chemistry = lanparser::load_lan(&lcs_chemistry).unwrap();

    let t = std::fs::read_to_string("sentences_chemistry.txt").unwrap();
    for text in t.split("\n") {
        println!("{}", text);
        let start = Instant::now();
        let dtext = assembling::disassemble(text);
        parse::init_parse();
        let k = parse::parse(&dtext, hm_chemistry["main"].build(&Vec::new()), &hm_chemistry, &dictionary);
        for _ in 1 .. 100 {
            parse::init_parse();
            parse::parse(&dtext, hm_chemistry["main"].build(&Vec::new()), &hm_chemistry, &dictionary);
        }
        let dur = start.elapsed();
        println!("{}", k.unwrap().0.collect_verbose());
        println!("==> {:?}\n", dur / 100);
    }

    let hm_tokipona = lanparser::load_lan(&lcs_tokipona).unwrap();

    let t = std::fs::read_to_string("sentences_tokipona.txt").unwrap();
    for text in t.split("\n") {
        println!("{}", text);
        let start = Instant::now();
        let dtext = assembling::disassemble(text);
        parse::init_parse();
        let k = parse::parse(&dtext, hm_tokipona["main"].build(&Vec::new()), &hm_tokipona, &dictionary);
        for _ in 1 .. 100 {
            parse::init_parse();
            parse::parse(&dtext, hm_tokipona["main"].build(&Vec::new()), &hm_tokipona, &dictionary);
        }
        let dur = start.elapsed();
        println!("{}", k.unwrap().0.collect_verbose());
        println!("==> {:?}\n", dur / 100);
    }

    let hm_korean = lanparser::load_lan(&lcs_korean).unwrap();

    let t = std::fs::read_to_string("sentences_korean.txt").unwrap();
    for text in t.split("\n") {
        println!("{}", text);
        let start = Instant::now();
        let dtext = assembling::disassemble(text);
        parse::init_parse();
        let k = parse::parse(&dtext, hm_korean["main"].build(&Vec::new()), &hm_korean, &dictionary);
        for _ in 1 .. 100 {
            parse::init_parse();
            parse::parse(&dtext, hm_korean["main"].build(&Vec::new()), &hm_korean, &dictionary);
        }
        let dur = start.elapsed();
        println!("{}", k.unwrap().0.collect_verbose());
        println!("==> {:?}\n", dur / 100);
    }
}