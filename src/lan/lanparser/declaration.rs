use std::fmt::Debug;

#[derive(Clone)]
pub struct PhraseContext<'n> {
    pub children :Vec<PhraseRules<'n>>,
    pub props :Option<&'n str>,
    pub is_trap: bool,
    pub name :&'n str
}

#[derive(Clone)]
pub struct PhraseIf<'s> {
    pub parameter :usize,
    pub value :&'s str,
    pub unless :bool,
    
    pub children :Vec<PhraseRules<'s>>
}

#[derive(Clone)]
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
    pub is_free :bool,
    pub template :T
}

#[derive(Clone)]
pub struct ShortWordPart<'s> {
    pub condition :Vec<ShortWordPartCondition<'s>>,
    pub part_name :&'s str
}

#[derive(Clone, Debug)]
pub enum LanReference<'s> {
    PartAttr((usize, usize)),
    PartParam(usize),
    Text(&'s str)
}

#[derive(Clone, Debug)]
pub struct ShortWordPartCondition<'s> {
    pub target :LanReference<'s>,
    pub argn :usize,
    pub neq :bool
}

impl Debug for ShortWordPart<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{:?}", self.part_name, self.condition)
    }
}

impl ShortWordPart<'_> {
    pub fn from_conditional<'s>(p :&'s str, c :&'s str) -> ShortWordPart<'s> {
        let mut condvec = Vec::new();
        for cond in c.split('&') {
            if let Some((a, b)) = &cond[1..].split_once('=') {
                let neq = a.ends_with('!');
                let argn :usize = 
                if a.ends_with('!') {
                    a[..a.len()-1].parse()
                }
                else {
                    a.parse()
                }.unwrap();
                
                if b.starts_with(':') {
                    let argn2 :usize = b[1..].parse().unwrap();
                    condvec.push(ShortWordPartCondition {
                        target: LanReference::PartParam(argn2),
                        argn, neq
                    });
                }
                else if b.starts_with('@') {
                    let (pa, pb) = b[1..].split_once(':').unwrap();
                    let pa = pa.parse::<usize>().unwrap();
                    let pb = pb.parse::<usize>().unwrap();
                    condvec.push(ShortWordPartCondition {
                        target: LanReference::PartAttr((pa, pb)),
                        argn, neq
                    });
                }
                else {
                    condvec.push(ShortWordPartCondition {
                        target: LanReference::Text(b),
                        argn, neq
                    });
                }
            }
        }
        ShortWordPart { condition: condvec, part_name: p }
    }
    
    pub fn from<'s>(p :&'s str) -> ShortWordPart<'s> {
        ShortWordPart { condition: Vec::new(), part_name: p }
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
    pub fn to_string(&self) -> String {
        match &self.template {
            Templates::Embed(e) => {
                String::from(*e)
            },
            Templates::ShortPart(s) => {
                format!("*{}{:?}", s.part_name, s.condition)
            },
            Templates::Template(t) => {
                if t.args.len() > 0 {
                    format!("${}{:?}", t.name, t.args)
                }
                else {
                    format!("${}", t.name)
                }
            },
            Templates::Text(t) => {
                format!("{}-{}", &t.text.iter().collect::<String>(), t.name)
            }
        }
    }

    pub fn from<'p>(t :Templates<'p>) -> TemplateNode<'p> {
        TemplateNode {
            is_optional: false,
            condition: None,
            is_free: false,
            template: t
        }
    }

    pub fn from_free<'p>(t :Templates<'p>) -> TemplateNode<'p> {
        TemplateNode {
            is_optional: false,
            condition: None,
            is_free: true,
            template: t
        }
    }
    
    pub fn from_opt<'p>(t :Templates<'p>) -> TemplateNode<'p> {
        TemplateNode {
            is_optional: true,
            condition: None,
            is_free: false,
            template: t
        }
    }
    
    
    pub fn from_conditional<'p>(t :Templates<'p>, c :&'p str) -> TemplateNode<'p> {
        TemplateNode {
            is_optional: false,
            condition: Some(c),
            is_free: false,
            template: t
        }
    }
}  

#[derive(Clone, Debug)]
pub struct Rule<'s> {
    pub rules: Vec<TemplateNode<'s>>,
    pub name: &'s str
}