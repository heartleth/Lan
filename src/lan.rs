pub mod dictionary;
pub mod lanparser;
pub mod concrete;
pub mod parse;

pub use crate::assembling;

use self::dictionary::add_dictionaryrc;

#[derive(Debug)]
pub enum LanError<E> {
    NoDictionaryError,
    LanSyntaxError,
    ParsingError,
    NoMainError,
    Error(E),
    SimpleError
}

pub struct Parser {
    lcs :Vec<String>,
    dictionary :Option<dictionary::DictionaryRc>
}

pub struct RefParser<'p> {
    dict: &'p dictionary::Dictionary<'p>,
    lan: &'p lanparser::LanRules<'p>
}

impl Parser {
    pub fn new() -> Result<Parser, ()> {
        Ok(Parser {
            lcs: Vec::new(),
            dictionary: None
        })
    }
    
    pub fn open(lan_path :&str, dictionary_path :&str) -> Result<Parser, LanError<usize>> {
        let dict = dictionary::load_dictionaryrc(&std::fs::read_to_string(dictionary_path).ok().ok_or(LanError::NoDictionaryError)?[..]);
        Ok(Parser {
            lcs: vec![ std::fs::read_to_string(lan_path).unwrap() ],
            dictionary: Some(dict)
        })
    }
    
    pub fn open_lan(lan_path :&str) -> Result<Parser, ()> {
        Ok(Parser {
            lcs: vec![ std::fs::read_to_string(lan_path).unwrap() ],
            dictionary: None
        })
    }

    // pub fn context<F, E:std::error::Error>(&self, block :F) -> Result<(), LanError<E>> where F:(Fn(lanparser::LanRules, dictionary::Dictionary) -> Result<(), E>) {
    pub fn with_parser<F>(&self, block :F) -> Result<(), LanError<()>> where F:(Fn(RefParser) -> Option<()>) {
        let lan = lanparser::load_lan(&self.lcs).map_err(|_| LanError::LanSyntaxError)?;
        let dict = dictionary::dictrc_to_dict(&&self.dictionary.as_ref().ok_or(LanError::NoDictionaryError)?);
        let rp = RefParser { dict: &dict, lan: &lan };
        block(rp).ok_or(LanError::SimpleError)?;
        Ok(())
    }

    pub fn load_lan(&mut self, lan_path :&str) {
        self.lcs.push(std::fs::read_to_string(lan_path).unwrap());
    }

    pub fn load_dict(&mut self, dictionary_path :&str) -> Result<(), LanError<()>> {
        let dict = &std::fs::read_to_string(dictionary_path).ok().ok_or(LanError::NoDictionaryError)?[..];
        let s = self.dictionary.as_mut().unwrap();
        add_dictionaryrc(s, dict);
        Ok(())
    }
}

pub struct ParseResult {
    pub tree :parse::syntree::SyntaxTreeNode,
    pub length :usize
}

impl<'p> RefParser<'p> {
    pub fn reparse_raw(&self, v :&Vec<char>) -> Result<Option<(parse::syntree::SyntaxTreeNode, usize)>, LanError<()>> {
        Ok(parse::parse(v, self.lan.get("main").ok_or(LanError::NoMainError)?.build(Vec::new()), self.lan, self.dict))
    }
    
    // pub fn reparse_raw_notree(&self, v :&Vec<char>) -> Result<Option<usize>, LanError<()>> {
    //     Ok(parse::parse_notree(v, self.lan.get("main").ok_or(LanError::NoMainError)?.build(Vec::new()), self.lan, self.dict))
    // }

    pub fn parse_raw(&self, v :&Vec<char>) -> Result<Option<(parse::syntree::SyntaxTreeNode, usize)>, LanError<()>> {
        parse::init_parse();
        self.reparse_raw(v)
    }

    pub fn parse<T>(&self, text :T) -> Result<ParseResult, LanError<()>> where String: From<T> {
        let v = assembling::disassemble(&String::from(text)[..]);
        self.parse_raw(&v)?.ok_or(LanError::ParsingError).map(|e| ParseResult {
            length: e.1,
            tree: e.0
        })
    }
    
    // pub fn parse_check<T>(&self, text: T) -> Result<usize, LanError<()>> where String: From<T> {
    //     let v = assembling::disassemble(&String::from(text)[..]);
    //     parse::init_parse();
    //     Ok(self.reparse_raw_notree(&v)?.ok_or(LanError::ParsingError)?)
    // }
}