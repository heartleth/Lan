use std::fmt::Debug;

pub struct PhraseContext<'n> {
    pub name :&'n str,
    pub children :Vec<PhraseRules<'n>>,
    pub argc :usize
}

pub struct PhraseIf<'s> {
    pub parameter :usize,
    pub value :&'s str,
    pub unless :bool,
    
    pub children :Vec<PhraseRules<'s>>
}

pub enum PhraseRules<'s> {
    SetVariable(Rule<'s>),
    Rules(Rule<'s>),
    Ifs(PhraseIf<'s>)
}

#[derive(Clone)]
pub struct ParamedPart<'s> {
    pub name :&'s str,
    pub args :Vec<&'s str>
}

impl std::fmt::Debug for ParamedPart<'_> {
    fn fmt(&self, ft: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        ft.write_str(&format!("{}({})", self.name, self.args.join(", "))[..])
    }
}

#[derive(Debug, Clone)]
pub struct SignText<'s> {
    pub text :Vec<char>,
    pub name :&'s str
}

#[derive(Debug, Clone)]
pub struct TemplateNodeGen<'s, T> {
    pub condition :Option<&'s str>,
    pub is_optional :bool,
    pub template :T
}

#[derive(Clone)]
pub struct ShortWordPart<'s> {
    pub condition :Option<&'s str>,
    pub part_name :&'s str
}

impl Debug for ShortWordPart<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(c) = self.condition {
            write!(f, "{}[{}]", self.part_name, c)
        }
        else {
            write!(f, "{}", self.part_name)
        }
    }
}

impl ShortWordPart<'_> {
    pub fn from_conditional<'s>(p :&'s str, c :&'s str) -> ShortWordPart<'s> {
        ShortWordPart { condition: Some(c), part_name: p }
    }
    pub fn from<'s>(p :&'s str) -> ShortWordPart<'s> {
        ShortWordPart { condition: None, part_name: p }
    }
}

#[derive(Debug, Clone)]
pub enum Templates<'s> {
    Embed(&'s str),
    Text(SignText<'s>),
    ShortPart(ShortWordPart<'s>),
    Template(ParamedPart<'s>),
}

pub type TemplateNode<'s> = TemplateNodeGen<'s, Templates<'s>>;

impl<'s> TemplateNode<'s> {
    pub fn from<'p>(t :Templates<'p>) -> TemplateNode<'p> {
        TemplateNode {
            is_optional: false,
            condition: None,
            template: t
        }
    }

    pub fn from_opt<'p>(t :Templates<'p>) -> TemplateNode<'p> {
        TemplateNode {
            is_optional: true,
            condition: None,
            template: t,
        }
    }
    
    pub fn from_conditional<'p>(t :Templates<'p>, c :&'p str) -> TemplateNode<'p> {
        TemplateNode {
            is_optional: false,
            condition: Some(c),
            template: t
        }
    }
}  

pub struct Rule<'s> {
    pub rules: Vec<TemplateNode<'s>>,
    pub name: &'s str
}