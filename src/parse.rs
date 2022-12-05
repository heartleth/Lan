pub mod syntree;

use syntree::*;

use super::lanparser::Templates::*;
use super::concrete::ConcretePart;
use super::lanparser::*;

use super::Dictionary;
use std::collections::HashMap;
use std::iter::FromIterator;

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
        if let Some((morphemes, x)) = fit_rules(s, r.name, &r.rules, rules, dict) {
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

pub fn fit_rules<'p, 't>(uts :&'t [char], name :&'p str, rule :ParsingRule<'p>, rules :PhraseRulesCollection<'p>, dict :&Dictionary<'p>) -> Option<(SyntaxTreeNode, usize)> {
    let print_debug = false;
    let s :Vec<_> = String::from_iter(uts).trim().chars().collect();
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
                    match &prule.template {
                        Text(t) => {
                            if print_debug {
                                println!("Text: {}", t.name);
                            }
                            if t.text == &['w', 's'] && (&String::from_iter(&s[reading..]).trim_start()[..].len() < &s[reading..].len()) {
                                let wss = &s[reading..].len() - &String::from_iter(&s[reading..]).trim_start()[..].len();
                                expect.read(wss);
                                expect.next_rule();
                                expect.tree.push_category(SyntaxTreeNode::new_morpheme(String::from(t.name), String::from(" ")));
                                if print_debug {
                                    println!("OK Text: {}", t.name);
                                }
                            }
                            else if s[reading..].starts_with(&t.text) {
                                expect.read(t.text.len());
                                expect.next_rule();
                                expect.tree.push_category(SyntaxTreeNode::new_morpheme(String::from(t.name), String::from_iter(&t.text)));
                                if print_debug {
                                    println!("OK Text: {}", t.name);
                                }
                            }
                            else {
                                expect.kill();
                            }
                        },
                        ShortPart(p) => {
                            if print_debug {
                                println!("ShortPart: {:?}", p);
                            }
                            let mut x :Vec<_> = dict[p.part_name].iter()
                            .filter(|e| {
                                if s.len() - reading < e.text.len() {
                                    return false;
                                }
                                if let Some(cond) = p.condition {
                                    if let Some((a, b)) = &cond[1..].split_once('=') {
                                        let argn :usize = a.parse().unwrap();
                                        &s[reading..reading+e.text.len()] == &e.text[..]
                                        && e.argv.get(argn).unwrap_or(&"0") == b
                                    }
                                    else {
                                        let (a, b) = &cond[1..].split_once('~').unwrap();
                                        let argn :usize = a.parse().unwrap();
                                        &s[reading..reading+e.text.len()] == &e.text[..]
                                        && e.argv.get(argn).unwrap_or(&"0") == b
                                    }
                                }
                                else {
                                    &s[reading..reading+e.text.len()] == &e.text[..]
                                }
                            }).collect();

                            if x.is_empty() {
                                expect.kill();
                            }
                            else {
                                x.sort_by_key(|x| x.text.len());
                                x.reverse();
                                if print_debug {
                                    println!("OK ShortPart: {:?} @ {:?}", p, x[0].text);
                                }
                                expect.tree.push_category(SyntaxTreeNode::new_morpheme(
                                    String::from(p.part_name),
                                    String::from_iter(&s[reading..reading+x[0].text.len()])));
                                expect.register_attr(&x[0].argv);
                                expect.read(x[0].text.len());
                                expect.next_rule();
                            }
                        },
                        Template(template) => {
                            if print_debug {
                                println!("> Template: {:?}", template);
                            }

                            let mut t2 = template.clone();
                            for e in t2.args.iter_mut() {
                                if e.starts_with("@") {
                                    let (a, b) = e[1..].split_once(':').unwrap();
                                    let a :usize = a.parse().unwrap();
                                    let b :usize = b.parse().unwrap();
                                    // *e = expect.voca_attrs[a].get(b).unwrap_or(&"0");
                                    *e = expect.voca_attrs.get(a).unwrap_or(&&Vec::new()).get(b).unwrap_or(&"0");
                                }
                            }
                            let parsed = parse(&s[reading..], t2.build(rules), rules, dict);
                            if let Some((stn, x)) = parsed {
                                if print_debug {
                                    println!("OK Template: {:?}", template);
                                }
                                expect.read(x);
                                expect.next_rule();
                                
                                expect.tree.push_category(stn);
                            }
                            else {
                                if print_debug {
                                    println!("- FAIL Template: {:?}", template);
                                }
                                expect.kill();
                            }
                        },
                        _ => ()
                    };
                }
            }
            else {
                if s[0] == uts[0] {
                    winners.push((expect.tree.clone(), reading));
                    expect.kill();
                }
                else {
                    winners.push((expect.tree.clone(), reading + 1));
                    expect.kill();
                }
            }
        }
        expections.retain(|x| x.alive);
        let mut new_expections = Vec::new();
        for ex in &expections {
            new_expections = [new_expections, ex.nexts()].concat();
        }
        expections = new_expections;
    }

    if winners.is_empty() {
        return None;
    }
    else {
        let best_winner = winners.iter().max_by(|x, y| x.1.cmp(&y.1));
        return Some(best_winner.unwrap().clone());
    }
}