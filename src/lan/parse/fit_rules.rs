mod expect;
use crate::lan::dictionary::SearchDictResult;
use crate::lan::lanparser::Templates::*;
use crate::lan::dictionary::search_dict;
use crate::lan::dictionary::Dictionary;
use super::PhraseRulesCollection;
use std::collections::HashMap;
pub use expect::Expectation;
use super::SyntaxTreeNode;
use super::ParsingRule;
use super::SyntaxTree;
use crate::SKIP_WS;
use expect::nexts;
use super::parse;

static mut PARSE_DP :Option<HashMap<(usize, String), Option<(SyntaxTreeNode, usize)>>> = None;
static mut PARSE_VISIT :Option<HashMap<(usize, String), u8>> = None;
static STACK_LIMIT :u8 = 2;

pub fn init_parse() {
    unsafe {
        PARSE_VISIT = Some(HashMap::new());
        PARSE_DP = Some(HashMap::new());
    }
}

fn parsingrule_tostr<'p, 't>(p :ParsingRule<'p>) -> String {
    let v :Vec<_> = p.iter().map(|x| x.to_string()).collect();
    return v.join("");
}

fn cache_view(l :usize, rulehash :&String) -> Result<(SyntaxTreeNode, usize), bool> {
    unsafe {
        let key = (l, rulehash.clone());
        if let Some(k) = &PARSE_DP {
            if let Some(x) = k.get(&key) {
                return x.clone().ok_or(true);
            }
        }

        if let Some(k) = &mut PARSE_VISIT {
            if let Some(x) = k.get(&key) {
                if x < &STACK_LIMIT {
                    k.insert(key, x + 1);
                }
                else {
                    return Err(true);
                }
            }
            else {
                k.insert(key, 0);
            }
        }
    }
    return Err(false);
}

fn cache_register(l :usize, rulehash :String, tree :Option<(SyntaxTreeNode, usize)>) {
    unsafe {
        if let Some(k) = &mut PARSE_DP {
            k.insert((l, rulehash), tree);
        }
    }
}

pub fn fit_rules<'p, 't>(s :&'t [char], name :&'p str, rule :ParsingRule<'p>, rules :PhraseRulesCollection<'p>, dict :&'p Dictionary<'p>, cargs :&Vec<&'p str>, is_trap :bool) -> Option<(SyntaxTreeNode, usize)> {
    let rulehash = parsingrule_tostr(rule) + &cargs.join("")[..];
    let cachev = cache_view(s.len(), &rulehash);
    if let Err(true) = cachev {
        return None;
    }
    else if let Ok(x) = cachev {
        return Some(x);
    }

    let mut expections = nexts(name, rule);
    let mut winners = Vec::new();
    let mut i = 0;
    while !expections.is_empty() {
        for expect in &mut expections {
            let reading = expect.reading;
            let oread = reading;
            let rule = expect.rule;
            if let Some(prule) = &rule.first() {
                if reading >= s.len() {
                    if let Text(t) = &prule.template {
                        if t.text == vec!['>'] && t.name == ">" {
                            expect.next_rule();
                        }
                        else {
                            expect.kill();
                        }
                    }
                    else {
                        expect.kill();
                    }
                }
                else {
                    unsafe {
                        if !SKIP_WS {
                            while s[expect.reading] == ' ' || s[expect.reading] == '\r' || s[expect.reading] == '\n' || s[expect.reading] == '\t' {
                                expect.read(1);
                            }
                        }
                    }
                    let reading = expect.reading;
                    match &prule.template {
                        Text(t) => {
                            if t.text == vec!['<'] && t.name == "<" {
                                if reading == 0 {
                                    expect.next_rule();
                                }
                                else if s[reading - 1] == '\n' {
                                    expect.next_rule();
                                }
                                else {
                                    expect.kill();
                                }
                            }
                            else if t.text == vec!['>'] && t.name == ">" {
                                if reading == s.len() {
                                    expect.next_rule();
                                }
                                else if s[reading + 1] == '\n' {
                                    expect.read(1);
                                    expect.next_rule();
                                    expect.push_category(SyntaxTreeNode::new_morpheme(String::from(t.name), s[reading..reading+1].iter().collect::<String>()));
                                }
                                else if s[reading + 1] == '\r' {
                                    expect.read(2);
                                    expect.next_rule();
                                    expect.push_category(SyntaxTreeNode::new_morpheme(String::from(t.name), s[reading..reading+2].iter().collect::<String>()));
                                }
                                else {
                                    expect.kill();
                                }
                            }
                            else if t.text.len() == 0 {
                                let mut j = 0;
                                while s[expect.reading+j] == ' ' || s[expect.reading+j] == '\r' || s[expect.reading+j] == '\n' || s[expect.reading+j] == '\t' {
                                    j += 1;
                                }
                                if expect.reading + j > oread {
                                    expect.read(j);
                                    expect.next_rule();
                                    expect.push_category(SyntaxTreeNode::new_morpheme(String::from(t.name), s[oread..reading+j].iter().collect::<String>()));
                                }
                                else {
                                    expect.kill();
                                }
                            }
                            else if s[reading..].starts_with(&t.text) {
                                expect.read(t.text.len());
                                expect.next_rule();
                                expect.push_category(SyntaxTreeNode::new_morpheme(String::from(t.name), s[reading..reading+t.text.len()].iter().collect::<String>()));
                            }
                            else {
                                expect.kill();
                            }
                        },
                        ShortPart(p) => {
                            let res = search_dict(dict, p, s, reading, expect, cargs);
                            if let SearchDictResult::Part(x) = res {
                                if x.is_empty() {
                                    expect.kill();
                                }
                                else {
                                    let mx = *x.iter().max_by_key(|t| t.text.len()).unwrap();
                                    expect.push_category(SyntaxTreeNode::new_morpheme(
                                        String::from(p.part_name),
                                        String::from_iter(&s[reading..reading+mx.text.len()])));
                                    expect.register_attr(&mx.argv);
                                    expect.read(mx.text.len());
                                    expect.next_rule();
                                }
                            }
                            else if let SearchDictResult::Trap(l) = res {
                                expect.read(l);
                                expect.next_rule();
                                expect.push_category(SyntaxTreeNode::new_morpheme(format!("until[{}]", p.part_name), s[reading..reading+l].iter().collect::<String>()));
                            }
                        },
                        Template(template) => {
                            let mut t2 = template.args.clone();
                            for e in t2.iter_mut() {
                                if &e[..1] == "@" {
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
                            let template_built = template.build(rules, t2);
                            let parsed = parse(&s[reading..], template_built, rules, dict);
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
        i += 1;
        if i > 200 {
            return None;
        }
        expections = new_expections;
    }

    if winners.is_empty() {
        cache_register(s.len(), rulehash, None);
        return None;
    }
    else {
        let best_winner = winners.iter().max_by_key(|x| x.1);
        cache_register(s.len(), rulehash, Some(best_winner.unwrap().clone()));
        let mut bw = best_winner.unwrap().clone();
        if is_trap {
            trap_reconstruct(&mut bw);
        }
        return Some(bw);
    }
}

fn trap_reconstruct(tr: &mut (SyntaxTreeNode, usize)) {
    if let SyntaxTreeNode::Category(c) = &mut tr.0 {
        if c.children.len() > 1 {
            let size = c.children.len();
            c.children.swap(0, size - 1);
            tr2(c);
        }
    }
}

fn tr2(c: &mut SyntaxTree) {
    let mut l2 = c.children.first().unwrap().clone();
    if match l2.get_category() { None => true, Some(cc) => c.name != cc.name } {
        let size = c.children.len();
        c.children.swap(0, size - 1);
        return;
    }
    let x = l2.get_category_mut().unwrap().children.last_mut().unwrap();
    std::mem::swap(x, c.children.last_mut().unwrap());
    tr2(l2.get_category_mut().unwrap());
    std::mem::swap(c.children.first_mut().unwrap(), &mut l2);
}