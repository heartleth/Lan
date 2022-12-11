pub mod assembling;
pub mod lan;

use std::time::Instant;
use lan::Parser;

fn main() {
    let mut parser_korean = Parser::open("lans_korean/main.lan", "dictionary.dic").unwrap();
    parser_korean.load_lan("lans_korean/substantives.lan");
    parser_korean.load_lan("lans_korean/agglutinate.lan");
    parser_korean.context(|p| {
        println!("{}\n\n", p.parse("인생은 아름답다").unwrap().tree.collect_verbose(" "));

        let t = std::fs::read_to_string("sentences_korean.txt").unwrap();
        for text in t.split("\n") {
            println!("{}", text);
            let start = Instant::now();
            let result = p.parse(text).unwrap();
            let dur = start.elapsed();
            println!("{}", result.tree.collect_verbose(" "));
            println!("==> {:?}\n", dur);
        }
        Some(())
    }).unwrap();

    let parser_chemistry = Parser::open("lans_chemistry/main.lan", "dictionary.dic").unwrap();
    parser_chemistry.context(|p| {
        let t = std::fs::read_to_string("sentences_chemistry.txt").unwrap();
        for text in t.split("\n") {
            println!("{}", text);
            let start = Instant::now();
            let result = p.parse(text).unwrap();
            let dur = start.elapsed();
            println!("{}", result.tree.collect_verbose(" "));
            println!("==> {:?}\n", dur);
        }
        Some(())
    }).unwrap();

    let parser_tokipona = Parser::open("lans_tokipona/main.lan", "dictionary.dic").unwrap();
    parser_tokipona.context(|p| {
        let t = std::fs::read_to_string("sentences_tokipona.txt").unwrap();
        for text in t.split("\n") {
            println!("{}", text);
            let start = Instant::now();
            let result = p.parse(text).unwrap();
            let dur = start.elapsed();
            println!("{}", result.tree.collect_verbose(" "));
            println!("==> {:?}\n", dur);
        }
        Some(())
    }).unwrap();
}