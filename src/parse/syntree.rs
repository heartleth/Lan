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
            name: String::from(name),
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

    pub fn collect(&self)->String {
        match self {
            Self::Category(c) => {
                let c :Vec<_> = c.children.iter().map(|e| e.collect()).collect();
                c.join(" ")
            },
            Self::Vocab(v) => {
                String::from(&v.text)
            }
        }
    }
    
    pub fn collect_verbose(&self)->String {
        match self {
            Self::Category(c) => {
                let cc :Vec<_> = c.children.iter().map(|e| e.collect_verbose()).collect();
                format!("{}[ {} ]", &c.name, cc.join(" "))
            },
            Self::Vocab(v) => {
                format!("{}", &v.text)
            }
        }
    }
}