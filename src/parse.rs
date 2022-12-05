pub mod fit_rules;
pub mod syntree;

use syntree::*;

use super::concrete::ConcretePart;
use super::lanparser::*;

use super::Dictionary;
use std::collections::HashMap;

pub type PhraseRulesCollection<'p> = &'p HashMap<&'p str, PhraseContext<'p>>;
pub type ParsingRule<'p> = &'p [TemplateNode<'p>];

static mut PARSE_DP :Option<HashMap<(usize, String), Option<(SyntaxTreeNode, usize)>>> = None;

pub fn init_parse() {
    unsafe { PARSE_DP = Some(HashMap::new()); }
}

pub fn parse<'p, 't>(s :&'t [char], part :ConcretePart<'t, 'p>, rules :PhraseRulesCollection<'p>, dict :&Dictionary<'p>) -> Option<(SyntaxTreeNode, usize)> {
    unsafe {
        if let Some(k) = &PARSE_DP {
            if let Some(x) = k.get(&(s.len(), part.id.clone())) {
                return x.clone();
            }
        }
    }
    let mut m = 0;
    let mut mp = None;
    for r in part.rules {
        if let Some((morphemes, x)) = fit_rules::fit_rules(s, r.name, &r.rules, rules, dict) {
            if x > m {
                mp = Some(morphemes);
                m = x;
            }
        }
    }

    if m == 0 {
        unsafe {
            if let Some(k) = &mut PARSE_DP {
                k.insert((s.len(), part.id), None);
            }
        }
        return None;
    }
    else {
        unsafe {
            if let Some(k) = &mut PARSE_DP {
                k.insert((s.len(), part.id), Some((mp.clone().unwrap(), m)));
            }
        }
        return Some((mp.unwrap(), m));
    }
}

#[derive(Clone)]
pub struct Expectation<'p> {
    pub voca_attrs :Vec<&'p Vec<&'p str>>,
    pub rule :ParsingRule<'p>,
    pub tree :SyntaxTreeNode,
    pub reading :usize,
    pub name :&'p str,
    pub alive :bool,
}

impl<'p> Expectation<'p> {
    pub fn from<'tt, 'n>(name :&'n str, rule :ParsingRule<'n>, tree :SyntaxTreeNode, reading :usize)->Expectation<'n> {
        Expectation {
            voca_attrs: Vec::new(),
            alive: true,
            tree,
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
    
    pub fn read(&mut self, amount :usize) {
        self.reading += amount;
    }
    
    pub fn register_attr(&mut self, attr :&'p Vec<&'p str>) {
        self.voca_attrs.push(attr);
    }

    pub fn nexts(&self)->Vec<Expectation<'p>> {
        if self.rule.is_empty() {
            return vec![ self.clone() ];
        }
        let mut ret :Vec<Expectation<'p>> = Vec::new();
        let mut i = 0;
        for r in self.rule {
            if r.is_optional {
                let mut p = Expectation::from(self.name, &self.rule[i..], self.tree.clone(), self.reading);
                for attr in &self.voca_attrs {
                    p.register_attr(attr);
                }
                ret.push(p);
            }
            else {
                let mut p = Expectation::from(self.name, &self.rule[i..], self.tree.clone(), self.reading);
                for attr in &self.voca_attrs {
                    p.register_attr(attr);
                }
                ret.push(p);
                return ret;
            }
            i = i + 1;
        }
        if self.rule.last().unwrap().is_optional {
            ret.push(Expectation::from(self.name, &self.rule[0..0], self.tree.clone(), self.reading));
        }
        return ret;
    }
}

pub fn nexts<'s, 'p>(name: &'p str, rule :ParsingRule<'p>) -> Vec<Expectation<'p>> {
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