pub mod assembling;
pub mod examples;
pub mod lan;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// path of file to parse.
    #[arg(short = 't', long)]
    doc: String,
    
    /// Lan grammer paths, separated by ",".
    #[arg(short, long)]
    spec: String,
    
    /// Lan dictionary paths, separated by ",".
    #[arg(short, long)]
    dict: String
}

fn main() {
    let args = Args::parse();
    let specs :Vec<&str> = args.spec.split(',').collect();
    let dicts :Vec<&str> = args.dict.split(',').collect();
    
    println!("{:?}", specs);
    println!("{:?}", dicts);
    let mut lanparser = lan::Parser::open(specs[0], dicts[0]).unwrap();
    let mut i = 0;
    for dictf in dicts {
        if i > 0 {
            lanparser.load_dict(dictf).unwrap();
        }
        i += 1;
    }
    i = 0;
    for specf in specs {
        if i > 0 {
            lanparser.load_lan(specf);
        }
        i += 1;
    }
    

    lanparser.with_parser(|p| {
        let t = std::fs::read_to_string(&args.doc).unwrap();
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
                    println!("{}\n", res.tree.collect_verbose(" "));
                }
                else {
                    println!("INVALID\n");
                }
            }
            else {
                println!("INVALID\n");
            }
        }
        Some(())
    }).unwrap();
}