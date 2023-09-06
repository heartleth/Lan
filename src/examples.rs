use super::lan::Parser;
use std::time::Instant;

pub fn korean() {
    let mut parser_korean = Parser::open("examples/lans_korean/main.lan", "examples/dictionary/dictionary.dic").unwrap();
    parser_korean.load_lan("examples/lans_korean/substantives.lan");
    parser_korean.load_lan("examples/lans_korean/agglutinate.lan");
    parser_korean.with_parser(|p| {
        println!("{}\n\n", p.parse("인생은 아름답다").unwrap().tree.collect_verbose(" "));

        let t = std::fs::read_to_string("examples/sentences_korean.txt").unwrap();
        for text in t.split("\n") {
            println!("{}", text);
            // let start = Instant::now();
            let result = p.parse(text).unwrap();
            // let dur = start.elapsed();
            println!("{}", result.tree.collect_verbose(" "));
            // println!("==> {:?}\n", dur);
        }
        Some(())
    }).unwrap();
}

pub fn english() {
    let mut parser_english = Parser::open("examples/lans_english/main.lan", "examples/dictionary/dictionary.dic").unwrap();
    parser_english.load_dict("examples/dictionary/dicteng_noun.dic").unwrap();
    parser_english.load_dict("examples/dictionary/dicteng_verb.dic").unwrap();
    parser_english.load_lan("examples/lans_english/verb.lan");
    parser_english.load_lan("examples/lans_english/socnp.lan");
    
    parser_english.with_parser(|p| {
        let t = std::fs::read_to_string("examples/sentences_english.txt").unwrap();
        for text in t.split("\n") {
            if text.starts_with('#') {
                // println!("{}", &text[2..]);
                continue;
            }
            let text = text.trim();
            println!("{}", text);
            let start = Instant::now();
            let result = p.parse(text);
            let dur = start.elapsed();
            if let Ok(res) = result {
                if res.length == text.len() {
                    println!("문법 오류 없음");
                    println!("문장분석: {}", res.tree.collect_verbose(" "));
                }
                else {
                    println!("문법 오류 있음");
                }
            }
            else {
                println!("문법 오류 있음");
            }
            println!("==> {:?}\n", dur);
        }
        Some(())
    }).unwrap();
}