pub mod assembling;
pub mod lan;

use std::time::Instant;
use lan::Parser;

fn main() {
    // let mut parser_korean = Parser::open("lans_korean/main.lan", "dictionary.dic").unwrap();
    // parser_korean.load_lan("lans_korean/substantives.lan");
    // parser_korean.load_lan("lans_korean/agglutinate.lan");
    // parser_korean.context(|p| {
    //     println!("{}\n\n", p.parse("인생은 아름답다").unwrap().tree.collect_verbose(" "));
    // 
    //     let t = std::fs::read_to_string("sentences_korean.txt").unwrap();
    //     for text in t.split("\n") {
    //         println!("{}", text);
    //         let start = Instant::now();
    //         let result = p.parse(text).unwrap();
    //         let dur = start.elapsed();
    //         println!("{}", result.tree.collect_verbose(" "));
    //         println!("==> {:?}\n", dur);
    //     }
    //     Some(())
    // }).unwrap();

    let mut parser_english = Parser::open("lans_english/main.lan", "dictionary/dictionary.dic").unwrap();
    parser_english.load_dict("dictionary/dicteng_noun.dic").unwrap();
    parser_english.load_dict("dictionary/dicteng_verb.dic").unwrap();
    parser_english.load_lan("lans_english/verb.lan");
    parser_english.load_lan("lans_english/socnp.lan");
    
    parser_english.with_parser(|p| {
        let t = std::fs::read_to_string("sentences_english.txt").unwrap();
        for text in t.split("\n") {
            if text.starts_with('#') {
                println!("{}", &text[2..]);
                continue;
            }
            
            let text = text.trim();
            println!("{}", text);
            let start = Instant::now();
            let result = p.parse(text);
            let dur = start.elapsed();
            if let Ok(res) = result {
                if res.length == text.len() {
                    println!("VALID");
                    // println!("{}", res.tree.collect_verbose(" "));
                }
                else {
                    println!("INVALID");
                    // println!("{}", res.tree.collect_verbose(" "));
                    // println!("PARSE FAILED");
                }
            }
            else {
                println!("INVALID");
                // println!("PARSE FAILED");
            }
            println!("==> {:?}\n", dur);
        }
        Some(())
    }).unwrap();
}