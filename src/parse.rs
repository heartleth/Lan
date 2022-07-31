pub mod syntree;

use syntree::*;

use super::lanparser::Templates::*;
use super::concrete::ConcretePart;
use super::lanparser::*;

use super::Dictionary;
use std::collections::HashMap;

pub type PhraseRulesCollection<'p> = &'p HashMap<&'p str, PhraseContext<'p>>;
pub type ParsingRule<'p> = &'p [TemplateNode<'p>];

pub fn parse<'p, 't>(s :&'t [char], part :ConcretePart<'t, 'p>, rules :PhraseRulesCollection<'p>, dict :&Dictionary<'p>) -> Option<(Morphemes, usize)> {
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
        return None;
    }
    else {
        return Some((mp.unwrap(), m));
    }
}

pub struct Expectation<'p> {
    pub voca_attrs :Vec<&'p Vec<&'p str>>,
    pub rule :ParsingRule<'p>,
    pub morphemes :Morphemes,
    pub reading :usize,
    pub name :&'p str,
    pub alive :bool,
}

impl<'p> Expectation<'p> {
    pub fn from<'tt, 'n>(name :&'n str, rule :ParsingRule<'n>)->Expectation<'n> {
        Expectation {
            voca_attrs: Vec::new(),
            morphemes: Vec::new(),
            alive: true,
            reading: 0,
            rule: rule,
            name: name
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
}

pub fn nexts<'s, 'p>(name: &'p str, rule :ParsingRule<'p>) -> Vec<Expectation<'p>> {
    let mut ret :Vec<Expectation<'p>> = Vec::new();
    let mut i = 0;
    for r in rule {
        if r.is_optional {
            ret.push(Expectation::from(name, &rule[i..]));
        }
        else {
            ret.push(Expectation::from(name, &rule[i..]));
            return ret;
        }
        i = i + 1;
    }
    return ret;
}

pub fn fit_rules<'p, 't>(uts :&'t [char], name :&'p str, rule :ParsingRule<'p>, rules :PhraseRulesCollection<'p>, dict :&Dictionary<'p>) -> Option<(Morphemes, usize)> {
    let print_debug = false;
    let s :Vec<_> = String::from_iter(uts).trim().chars().collect();
    let mut expections = nexts(name, rule);
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
                            if s[reading..].starts_with(&t.text) {
                                expect.read(t.text.len());
                                expect.next_rule();
                                expect.morphemes.push(Morpheme {
                                    text: String::from_iter(&t.text),
                                    name: String::from(t.name)
                                });
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
                            let x :Vec<_> = dict[p.part_name].iter()
                            .filter(|e| {
                                if s.len() - reading < e.text.len() {
                                    return false;
                                }
                                if let Some(cond) = p.condition {
                                    if let Some((a, b)) = &cond[1..].split_once('=') {
                                        let argn :usize = a.parse().unwrap();
                                        &s[reading..reading+e.text.len()] == &e.text[..]
                                        && &e.argv[argn] == b
                                    }
                                    else { panic!() }
                                }
                                else {
                                    &s[reading..reading+e.text.len()] == &e.text[..]
                                }
                            }).collect();

                            if x.is_empty() {
                                expect.kill();
                            }
                            else {
                                if print_debug {
                                    println!("OK ShortPart: {:?}", p);
                                }
                                expect.morphemes.push(Morpheme {
                                    text: String::from_iter(&s[reading..reading+x[0].text.len()]),
                                    name: String::from(p.part_name)
                                });
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
                                    *e = expect.voca_attrs[a].get(b).unwrap_or(&"0");
                                }
                            }
                            let parsed = parse(&s[reading..], t2.build(rules), rules, dict);
                            
                            if let Some((mut morphemes, x)) = parsed {
                                if print_debug {
                                    println!("OK Template: {:?}", template);
                                }
                                expect.read(x);
                                expect.next_rule();
                                expect.morphemes.append(&mut morphemes);
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
                    return Some((expect.morphemes.clone(), reading));
                }
                else {
                    return Some((expect.morphemes.clone(), reading + 1));
                }
            }
        }
        expections.retain(|x| x.alive);
    }
    return None;
}