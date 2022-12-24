use crate::lan::lanparser::Templates::*;
use crate::lan::lanparser::LanReference;
use crate::lan::dictionary::Dictionary;
use super::PhraseRulesCollection;
use std::collections::HashMap;
use super::SyntaxTreeNode;
use super::ParsingRule;
use super::parse;

static mut PARSE_DP :Option<HashMap<(usize, String), Option<(SyntaxTreeNode, usize)>>> = None;
static mut PARSE_VISIT :Option<HashMap<(usize, String), u8>> = None;
static STACK_LIMIT :u8 = 2;

#[derive(Clone)]
pub struct Expectation<'p> {
    pub voca_attrs :Vec<&'p Vec<&'p str>>,
    pub rule :ParsingRule<'p>,
    pub tree :Option<SyntaxTreeNode>,
    pub reading :usize,
    pub name :&'p str,
    pub alive :bool,
}

impl<'p> Expectation<'p> {
    pub fn from<'n>(name :&'n str, rule :ParsingRule<'n>, tree :SyntaxTreeNode, reading :usize)->Expectation<'n> {
        Expectation {
            voca_attrs: Vec::new(),
            tree: Some(tree),
            alive: true,
            reading,
            rule,
            name
        }
    }
    
    pub fn kill(&mut self) {
        self.alive = false;
    }
    
    pub fn next_rule(&mut self) {
        self.rule = &self.rule[1..];
    }

    pub fn take_tree(&mut self) -> SyntaxTreeNode {
        self.tree.take().unwrap()
    }
    
    pub fn read(&mut self, amount :usize) {
        self.reading += amount;
    }
    
    pub fn register_attr(&mut self, attr :&'p Vec<&'p str>) {
        self.voca_attrs.push(attr);
    }

    pub fn push_category(&mut self, stn :SyntaxTreeNode) {
        self.tree.as_mut().unwrap().push_category(stn);
    }

    pub fn nexts(&mut self)->Vec<Expectation<'p>> {
        if self.rule.is_empty() {
            return vec![ self.clone() ];
        }
        let mut ret :Vec<Expectation<'p>> = Vec::new();
        let mut i = 0;
        for r in self.rule {
            if r.is_optional {
                let mut p = Expectation::from(self.name, &self.rule[i..], self.tree.clone().unwrap(), self.reading);
                for attr in &self.voca_attrs {
                    p.register_attr(attr);
                }
                ret.push(p);
            }
            else {
                let mut p = Expectation::from(self.name, &self.rule[i..], self.tree.take().unwrap(), self.reading);
                for attr in &self.voca_attrs {
                    p.register_attr(attr);
                }
                ret.push(p);
                return ret;
            }
            i = i + 1;
        }
        if self.rule.last().unwrap().is_optional {
            ret.push(Expectation::from(self.name, &self.rule[0..0], self.tree.take().unwrap(), self.reading));
        }
        return ret;
    }
}

pub fn nexts<'p>(name: &'p str, rule :ParsingRule<'p>) -> Vec<Expectation<'p>> {
    let mut ret :Vec<Expectation<'p>> = Vec::new();
    let mut i = 0;
    for r in rule {
        if r.is_optional {
            ret.push(Expectation::from(name, &rule[i..], SyntaxTreeNode::new_category(name), 0));
        }
        else {
            ret.push(Expectation::from(name, &rule[i..], SyntaxTreeNode::new_category(name), 0));
            return ret;
        }
        i = i + 1;
    }
    return ret;
}

pub fn init_parse() {
    unsafe {
        PARSE_DP = Some(HashMap::new());
        PARSE_VISIT = Some(HashMap::new());
    }
}

fn parsingrule_tostr<'p, 't>(p :ParsingRule<'p>) -> String {
    let v :Vec<_> = p.iter().map(|x| x.to_string()).collect();
    return v.join("");
}

pub fn fit_rules<'p, 't>(s :&'t [char], name :&'p str, rule :ParsingRule<'p>, rules :PhraseRulesCollection<'p>, dict :&'p Dictionary<'p>, cargs :&Vec<&'p str>) -> Option<(SyntaxTreeNode, usize)> {
    let rulehash = parsingrule_tostr(rule) + &cargs.join("")[..];
    unsafe {
        let key = (s.len(), rulehash.clone());
        if let Some(k) = &PARSE_DP {
            if let Some(x) = k.get(&key) {
                return x.clone();
            }
        }

        if let Some(k) = &mut PARSE_VISIT {
            if let Some(x) = k.get(&key) {
                if x < &STACK_LIMIT {
                    k.insert(key, x + 1);
                }
                else {
                    return None;
                }
            }
            else {
                k.insert(key, 0);
            }
        }
    }

    let mut expections = nexts(name, rule);
    let mut winners = Vec::new();
    while !expections.is_empty() {
        for expect in &mut expections {
            let reading = expect.reading;
            let rule = expect.rule;
            if let Some(prule) = &rule.first() {
                if reading >= s.len() {
                    expect.kill();
                }
                else {
                    while s[expect.reading] == ' ' || s[expect.reading] == '\r' || s[expect.reading] == '\n' || s[expect.reading] == '\t' {
                        expect.read(1);
                    }
                    let reading = expect.reading;
                    match &prule.template {
                        Text(t) => {
                            if s[reading..].starts_with(&t.text) {
                                expect.read(t.text.len());
                                expect.next_rule();
                                expect.push_category(SyntaxTreeNode::new_morpheme(String::from(t.name), s[reading..reading+t.text.len()].iter().collect::<String>()));
                            }
                            else {
                                expect.kill();
                            }
                        },
                        ShortPart(p) => {
                            let x :Vec<_> = dict[p.part_name].iter().filter(|e| {
                                if s.len() - reading < e.text.len() {
                                    return false;
                                }
                                if p.condition.is_empty() {
                                    return &s[reading..reading+e.text.len()] == &e.text[..];
                                }
                                else {
                                    return &s[reading..reading+e.text.len()] == &e.text[..]
                                    && p.condition.iter().map(|c| c.neq ^ (match c.target {
                                        LanReference::PartAttr((a, b)) => e.argv.get(c.argn).unwrap_or(&"0") == expect.voca_attrs[a].get(b).unwrap_or(&"0"),
                                        LanReference::PartParam(pi) => e.argv.get(c.argn).unwrap_or(&"0") == cargs.get(pi).unwrap_or(&"0"),
                                        LanReference::Text(t) => e.argv.get(c.argn).unwrap_or(&"0") == &t
                                    })).fold(true, |a, b| a && b);
                                }
                            }).collect();

                            if x.is_empty() {
                                expect.kill();
                            }
                            else {
                                let mx = *x.iter().max_by_key(|t| t.text.len()).unwrap();
                                expect.push_category(SyntaxTreeNode::new_morpheme(
                                    String::from(p.part_name),
                                    String::from_iter(&s[reading..reading+mx.text.len()])));
                                    // s[reading..reading+mx.text.len()].iter().collect::<String>()));
                                expect.register_attr(&mx.argv);
                                expect.read(mx.text.len());
                                expect.next_rule();
                            }
                        },
                        Template(template) => {
                            let mut t2 = template.args.clone();
                            for e in t2.iter_mut() {
                                if e.starts_with('@') {
                                    let (a, b) = e[1..].split_once(':').unwrap();
                                    let a :usize = a.parse().unwrap();
                                    let b :usize = b.parse().unwrap();
                                    if let Some(k) = expect.voca_attrs.get(a).unwrap().get(b) {
                                        *e = k;
                                    }
                                    else {
                                        *e = "0";
                                    }
                                }
                            }
                            let parsed = parse(&s[reading..], template.build(rules, t2), rules, dict);
                            if let Some((stn, x)) = parsed {
                                expect.read(x);
                                expect.next_rule();
                                
                                expect.push_category(stn);
                            }
                            else {
                                expect.kill();
                            }
                        },
                        _ => ()
                    };
                }
            }
            else {
                winners.push((expect.take_tree(), reading));
                expect.kill();
            }
        }
        let mut new_expections = Vec::new();
        for ex in &mut expections {
            if ex.alive {
                for k in ex.nexts() {
                    new_expections.push(k);
                }
            }
        }
        expections = new_expections;
    }
    
    if winners.is_empty() {
        unsafe {
            if let Some(k) = &mut PARSE_DP {
                k.insert((s.len(), rulehash), None);
            }
        }
        
        return None;
    }
    else {
        let best_winner = winners.iter().max_by_key(|x| x.1);
        
        unsafe {
            if let Some(k) = &mut PARSE_DP {
                k.insert((s.len(), rulehash), Some(best_winner.unwrap().clone()));
            }
        }
        
        return Some(best_winner.unwrap().clone());
    }
}