use crate::assembling::*;

#[derive(Clone, Debug)]
pub struct Morpheme {
    pub text: String,
    pub name: String
}

pub type Morphemes = Vec<Morpheme>;

#[derive(Clone)]
pub struct SyntaxTree {
    pub children: Vec<SyntaxTreeNode>,
    pub name: String,
}

#[derive(Clone)]
pub enum SyntaxTreeNode {
    Category(SyntaxTree),
    Vocab(Morpheme)
}

impl SyntaxTreeNode {
    pub fn new_category(name :&str)->SyntaxTreeNode {
        return SyntaxTreeNode::Category(SyntaxTree {
            name: String::from(name),
            children: Vec::new()
        });
    }

    pub fn new_morpheme(name :String, text :String)->SyntaxTreeNode {
        return SyntaxTreeNode::Vocab(Morpheme {
            name: name,
            text: String::from(text)
        });
    }

    pub fn push_category(&mut self, node :SyntaxTreeNode)->Option<()> {
        if let SyntaxTreeNode::Category(c) = self {
            c.children.push(node);
            return Some(());
        }
        else {
            return None;
        }
    }

    pub fn collect(&self, delim :&str)->String {
        match self {
            Self::Category(c) => {
                let c :Vec<_> = c.children.iter().map(|e| e.collect(delim)).collect();
                c.join(delim)
            },
            Self::Vocab(v) => {
                String::from(&v.text)
            }
        }
    }
    
    pub fn collect_verbose(&self, delim :&str)->String {
        match self {
            Self::Category(c) => {
                let cc :Vec<_> = c.children.iter().map(|e| e.collect_verbose(delim)).collect();
                format!("{}[ {} ]", &c.name, assemble(&cc.join(delim)))
            },
            Self::Vocab(v) => {
                if v.text == " " {
                    return String::new();
                }
                format!("{}", &v.text.trim())
            }
        }
    }

    pub fn is_category(&self) -> bool {
        match self {
            Self::Category(_) => true,
            _ => false
        }
    }

    pub fn is_vocab(&self) -> bool {
        match self {
            Self::Vocab(_) => true,
            _ => false
        }
    }

    pub fn get_vocab(&self) -> Option<&Morpheme> {
        match self {
            Self::Vocab(v) => Some(v),
            _ => None
        }
    }
    
    pub fn get_category(&self) -> Option<&SyntaxTree> {
        match self {
            Self::Category(v) => Some(v),
            _ => None
        }
    }
}