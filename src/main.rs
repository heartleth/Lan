pub mod assembling;
pub mod examples;
pub mod lan;

use lan::Parser;

fn main() {
    // examples::english();
    let parser_english = Parser::open("examples/lans_c/main.lan", "examples/dictionary/dictionary.dic").unwrap();

    parser_english.with_parser(|p| {
        let t = std::fs::read_to_string("examples/sentences_c.txt").unwrap();
        for text in t.split("\n") {
            if text.starts_with('#') {
                println!("{}", &text[2..]);
                continue;
            }
            let text = text.trim();
            println!("{}", text);
            let result = p.parse(text);
            if let Ok(res) = result {
                if res.length == text.len() {
                    println!("VALID");
                    println!("{:#?}\n", res.tree);
                }
                else {
                    println!("INVALID\n");
                }
            }
            else {
                println!("INVALID");
            }
        }
        Some(())
    }).unwrap();
}