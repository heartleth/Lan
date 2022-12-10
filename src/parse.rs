pub mod fit_rules;
pub mod syntree;

use syntree::*;

use super::concrete::ConcretePart;
use super::lanparser::*;

use super::dictionary::Dictionary;
use std::collections::HashMap;

pub type PhraseRulesCollection<'p> = &'p HashMap<&'p str, PhraseContext<'p>>;
pub type ParsingRule<'p> = &'p [TemplateNode<'p>];

static mut PARSE_DP :Option<HashMap<(usize, String), Option<(SyntaxTreeNode, usize)>>> = None;

pub fn init_parse() {
    fit_rules::init_parse();
    unsafe {
        PARSE_DP = Some(HashMap::new());
    }
}

pub fn trim_front_iter<'t>(s :&'t [char])->&'t [char] {
    let mut i = 0;
    while s[i] == ' ' || s[i] == '\t' || s[i] == '\n' {
        i += 1;
    }
    &s[i..]
}

pub fn parse<'p, 't>(s :&'t [char], part :ConcretePart<'t, 'p>, rules :PhraseRulesCollection<'p>, dict :&Dictionary<'p>) -> Option<(SyntaxTreeNode, usize)> {
    let key = (s.len(), part.id.clone());
    unsafe {
        if let Some(k) = &PARSE_DP {
            if let Some(x) = k.get(&key) {
                return x.clone();
            }
        }
    }

    let mut m = 0;
    let mut mp = None;
    let s2 :&[char] = trim_front_iter(s);
    for r in part.rules {
        if let Some((morphemes, x)) = fit_rules::fit_rules(&s2, r.name, &r.rules, rules, dict, s) {
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
                if let Some(x) = k.get(&key) {
                    if let Some(x2) = x {
                        if x2.1 < m {
                            k.insert(key, Some((mp.clone().unwrap(), m)));
                        }
                    }
                }
            }
        }
        return Some((mp.unwrap(), m));
    }
}