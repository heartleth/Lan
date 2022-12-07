use crate::lanparser::Templates::*;
use super::PhraseRulesCollection;
use std::collections::HashMap;
use super::SyntaxTreeNode;
use super::ParsingRule;
use crate::Dictionary;
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
    pub fn from<'tt, 'n>(name :&'n str, rule :ParsingRule<'n>, tree :SyntaxTreeNode, reading :usize)->Expectation<'n> {
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

    pub fn nexts(&self)->Vec<Expectation<'p>> {
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
                let mut p = Expectation::from(self.name, &self.rule[i..], self.tree.clone().unwrap(), self.reading);
                for attr in &self.voca_attrs {
                    p.register_attr(attr);
                }
                ret.push(p);
                return ret;
            }
            i = i + 1;
        }
        if self.rule.last().unwrap().is_optional {
            ret.push(Expectation::from(self.name, &self.rule[0..0], self.tree.clone().unwrap(), self.reading));
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

// pub fn fit_rules<'p, 't>(s :&'t [char], name :&'p str, rule :ParsingRule<'p>, rules :PhraseRulesCollection<'p>, dict :&Dictionary<'p>) -> Option<(SyntaxTreeNode, usize)> {
pub fn fit_rules<'p, 't>(s :&Vec<char>, name :&'p str, rule :ParsingRule<'p>, rules :PhraseRulesCollection<'p>, dict :&Dictionary<'p>, uts :&'t [char]) -> Option<(SyntaxTreeNode, usize)> {
    let rulehash = parsingrule_tostr(rule);
    unsafe {
        if let Some(k) = &PARSE_DP {
            if let Some(x) = k.get(&(uts.len(), rulehash.clone())) {
                return x.clone();
            }
        }
        if let Some(k) = &mut PARSE_VISIT {
            if let Some(x) = k.get(&(uts.len(), rulehash.clone())) {
                if x < &STACK_LIMIT {
                    k.insert((s.len(), rulehash.clone()), x + 1);
                }
                else {
                    return None;
                }
            }
            else {
                k.insert((uts.len(), rulehash.clone()), 0);
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
                    match &prule.template {
                        Text(t) => {
                            // if t.text == &['w', 's'] && (&String::from_iter(&s[reading..]).trim_start()[..].len() < &s[reading..].len()) {
                            //     let wss = &s[reading..].len() - &String::from_iter(&s[reading..]).trim_start()[..].len();
                            //     expect.read(wss);
                            //     expect.next_rule();
                            //     expect.tree.push_category(SyntaxTreeNode::new_morpheme(String::from(t.name), String::from(" ")));
                            //     if print_debug {
                            //         println!("OK Text: {}", t.name);
                            //     }
                            // }
                            // else
                            if s[reading..].starts_with(&t.text) {
                                expect.read(t.text.len());
                                expect.next_rule();
                                expect.push_category(SyntaxTreeNode::new_morpheme(String::from(t.name), String::from_iter(&t.text)));
                            }
                            else {
                                expect.kill();
                            }
                        },
                        ShortPart(p) => {
                            let mut x :Vec<_> =
                            if let Some(conds) = p.condition {
                                dict[p.part_name].iter().filter(|e| {
                                    if s.len() - reading < e.text.len() {
                                        return false;
                                    }
                                    let mut r = true;
                                    for cond in conds.split('&') {
                                        r &= if let Some((a, b)) = &cond[1..].split_once('=') {
                                            let argn :usize = a.parse().unwrap();
                                            &s[reading..reading+e.text.len()] == &e.text[..]
                                            // && e.argv.get(argn).unwrap_or(&"0") == b
                                            && e.argv.get(argn).unwrap_or(&"0") == b
                                        }
                                        else {
                                            let (a, b) = &cond[1..].split_once('~').unwrap();
                                            let argn :usize = a.parse().unwrap();
                                            &s[reading..reading+e.text.len()] == &e.text[..]
                                            && e.argv.get(argn).unwrap_or(&"0") == b
                                        };
                                    }
                                    r   
                                }).filter(|e| &s[reading..reading+e.text.len()] == &e.text[..]).collect()
                            }
                            else {
                                dict[p.part_name].iter().filter(|e| {
                                    if s.len() - reading < e.text.len() {
                                        return false;
                                    }
                                    else {
                                        &s[reading..reading+e.text.len()] == &e.text[..]
                                    }
                                }).collect()
                            };

                            if x.is_empty() {
                                expect.kill();
                            }
                            else {
                                x.sort_by_key(|x| 9999999 - x.text.len());
                                expect.push_category(SyntaxTreeNode::new_morpheme(
                                    String::from(p.part_name),
                                    String::from_iter(&s[reading..reading+x[0].text.len()])));
                                expect.register_attr(&x[0].argv);
                                expect.read(x[0].text.len());
                                expect.next_rule();
                            }
                        },
                        Template(template) => {
                            let mut t2 = template.clone();
                            for e in t2.args.iter_mut() {
                                if e.starts_with("@") {
                                    let (a, b) = e[1..].split_once(':').unwrap();
                                    let a :usize = a.parse().unwrap();
                                    let b :usize = b.parse().unwrap();
                                    *e = expect.voca_attrs.get(a).unwrap_or(&&Vec::new()).get(b).unwrap_or(&"0");
                                }
                            }
                            let parsed = parse(&s[reading..], t2.build(rules), rules, dict);
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
                if s[0] == uts[0] {
                    winners.push((expect.take_tree(), reading));
                    expect.kill();
                }
                else {
                    winners.push((expect.take_tree(), reading + 1));
                    expect.kill();
                }
            }
        }
        let mut new_expections = Vec::new();
        for ex in &expections {
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
        let best_winner = winners.iter().max_by(|x, y| x.1.cmp(&y.1));
        
        unsafe {
            if let Some(k) = &mut PARSE_DP {
                k.insert((s.len(), rulehash), Some(best_winner.unwrap().clone()));
            }
        }
        
        return Some(best_winner.unwrap().clone());
    }
}