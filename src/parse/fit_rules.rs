use crate::lanparser::Templates::*;
use super::PhraseRulesCollection;
use super::SyntaxTreeNode;
use super::ParsingRule;
use crate::Dictionary;
use super::nexts;
use super::parse;

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
                                
                                if let Some(conds) = p.condition {
                                    let mut r = true;
                                    for cond in conds.split("&") {
                                        r &= if let Some((a, b)) = &cond[1..].split_once('=') {
                                            let argn :usize = a.parse().unwrap();
                                            &s[reading..reading+e.text.len()] == &e.text[..]
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