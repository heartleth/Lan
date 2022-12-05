use std::collections::HashMap;

use super::parse::PhraseRulesCollection;
use super::lanparser::ParamedPart;
use super::lanparser::{ * };

pub type ConcreteRules<'s> = Rule<'s>;

pub struct ConcretePart<'s ,'p> {
    pub rules :Vec<ConcreteRules<'s>>,
    pub part :&'p PhraseContext<'s>,
    pub id :String
}

impl<'n> PhraseContext<'n> {
    fn argrules<'p>(rule :&'p Rule<'n>, args :&'p Vec<&'n str>, vals :&HashMap<&'p str, ConcreteRules<'n>>) -> Vec<TemplateNode<'n>> {
        let mut ret :Vec<TemplateNode<'n>> = Vec::new();
        for r in &rule.rules {
            let qualified = if let Some(cond) = r.condition {
                if let Some((param, value)) = cond.split_once('=') {
                    let idx :usize = param[1..].parse().unwrap();
                    &args.get(idx).unwrap_or(&"0")[..] == value
                }
                else {
                    let (param, value) = cond.split_once('~').unwrap();
                    let idx :usize = param[1..].parse().unwrap();
                    &args.get(idx).unwrap_or(&"0")[..] != value
                }
            }
            else { true };

            if qualified {
                if let Templates::Embed(k) = &r.template {
                    let x = vals[k].rules.clone();
                    ret = [ret, x].concat();
                }
                else if let Templates::Template(n) = &r.template {
                    let concreteargs :Vec<&'n str> = n.args.iter().map(
                        |x| if x.starts_with(":") {
                            let idx :usize = x[1..].parse().unwrap();
                            &args.get(idx).unwrap_or(&"0")[..]
                        }
                        else { x }
                    ).collect();
                    ret.push(TemplateNode {
                        template: Templates::Template(ParamedPart {
                            name: n.name,
                            args: concreteargs
                        }),
                        is_optional: r.is_optional,
                        condition: r.condition,
                    });
                }
                else {
                    ret.push(r.clone());
                }
            }
        }
        return ret;
    }

    pub fn gain_rules<'p, 'v>(&self, cs :&'p Vec<PhraseRules<'n>>, args :&'p Vec<&'n str>, vals :&'v mut HashMap<&'p str, ConcreteRules<'n>>) -> Vec<ConcreteRules<'n>> {
        let mut res :Vec<ConcreteRules<'n>> = Vec::new();
        for pr in cs {
            if let PhraseRules::Rules(r) = pr {
                let argedrules = Self::argrules(r, args, vals);
                
                res.push(ConcreteRules {
                    name: r.name,
                    rules: argedrules
                });
            }
            else if let PhraseRules::SetVariable(r) = pr {
                let argedrules = Self::argrules(r, args, vals);
                
                vals.insert(r.name, ConcreteRules {
                    name: "",
                    rules: argedrules
                });
            }
            else if let PhraseRules::Ifs(cond) = pr {
                if cond.unless {
                    if &args.get(cond.parameter).unwrap_or(&"0")[..] != cond.value {
                        res.append(&mut self.gain_rules(&cond.children, args, vals));
                    }
                }
                else {
                    if &args.get(cond.parameter).unwrap_or(&"0")[..] == cond.value {
                        res.append(&mut self.gain_rules(&cond.children, args, vals));
                    }
                }
            }
        }
        return res;
    }
    
    pub fn build<'p>(&'p self, args :&'p Vec<&'n str>) -> ConcretePart<'n, 'p> {
        let mut vals = HashMap::new();
        ConcretePart {
            rules: self.gain_rules(&self.children, args, &mut vals),
            part: &self,
            id: format!("{}@{:?}", self.name, args)
        }
    }
}

impl<'n> ParamedPart<'n> {
    pub fn build<'p>(&'p self, rules :PhraseRulesCollection<'n>) -> ConcretePart<'n, 'p> {
        rules[self.name].build(&self.args)
    }
}