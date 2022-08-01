mod declaration;

use std::collections::HashMap;
pub use declaration::*;
use super::assembling;
use Templates::*;

type TN<'p> = TemplateNode<'p>;

enum Context<'s> {
    Phrase(PhraseContext<'s>), If(PhraseIf<'s>)
}

pub fn parse_rules<'s>(words :&[&'s str]) -> Result<Vec<TemplateNode<'s>>, usize> {
    let mut res :Vec<TemplateNode<'s>> = Vec::new();
    let mut inargs = false;
    let mut aropt = false;
    let mut args :Vec<&str> = Vec::new();
    let mut cond :Option<&str> = None;
    let mut name :Option<&str> = None;

    for sword in words {
        let word :Vec<char> = sword.chars().collect();
        if inargs {
            if sword == &")" {
                inargs = false;
                if aropt {
                    let into = args;
                    res.push(TN::from_opt(Template(ParamedPart {
                        name: name.unwrap(),
                        args: into
                    })));
                    args = Vec::new();
                }
                else if let Some(c) = cond {
                    let into = args;
                    res.push(TN::from_conditional(Template(ParamedPart {
                        name: name.unwrap(),
                        args: into
                    }), c));
                    args = Vec::new();
                }
                else {
                    let into = args;
                    res.push(TN::from(Template(ParamedPart {
                        name: name.unwrap(),
                        args: into
                    })));
                    args = Vec::new();
                }
            }
            else {
                args.push(sword);
            }
        }
        else {
            if word.get(0) == Some(&'$') {
                if word.last() == Some(&'(') {
                    inargs = true;
                    aropt = false;
                    name = Some(&sword[1..sword.len()-1]);
                }
                else {
                    res.push(TN::from(Templates::Template(ParamedPart {
                        args: Vec::new(),
                        name: &sword[1..]
                    })));
                }
            }
            else if word.get(0) == Some(&'*') {
                if let Some((a, b)) = &sword[1..].split_once('[') {
                    res.push(TN::from(Templates::ShortPart(ShortWordPart::from_conditional(a, &b[..b.len()-1]))));
                }
                else {
                    res.push(TN::from(Templates::ShortPart(ShortWordPart::from(&sword[1..]))));
                }
            }
            else if word.get(0) == Some(&'%') {
                res.push(TN::from(Templates::Embed(&sword[1..])));
            }
            else if word.get(0) == Some(&'?') {
                if word.get(1) == Some(&'$') {
                    if word.last() == Some(&'(') {
                        inargs = true;
                        aropt = true;
                        name = Some(&sword[2..sword.len()-1]);
                    }
                    else {
                        res.push(TN::from_opt(Templates::Template(ParamedPart {
                            args: Vec::new(),
                            name: &sword[2..]
                        })));
                    }
                }
                else if word.get(1) == Some(&'*') {
                    if let Some((a, b)) = &sword[2..].split_once('[') {
                        res.push(TN::from_opt(Templates::ShortPart(ShortWordPart::from_conditional(a, &b[..b.len()-1]))));
                    }
                    else {
                        res.push(TN::from_opt(Templates::ShortPart(ShortWordPart::from(&sword[2..]))));
                    }
                }
                else {
                    let t :Vec<&str> = sword[1..].split('-').collect();
                    res.push(TN::from_opt(Templates::Text(SignText {
                        text: assembling::disassemble(t[0]),
                        name: t[1]
                    })));
                }
            }
            else if word.get(0) == Some(&'[') {
                let (condition, content) = &sword[1..].split_once(']').unwrap();
                if content.starts_with("$") {
                    if word.last() == Some(&'(') {
                        inargs = true;
                        aropt = false;
                        name = Some(&content[1..content.len()-1]);
                        cond = Some(condition);
                    }
                    else {
                        res.push(TN::from_conditional(Templates::Template(ParamedPart {
                            args: Vec::new(),
                            name: &content[1..]
                        }), condition));
                    }
                }
                else if content.starts_with("*") {
                    if let Some((a, b)) = &content[1..].split_once('[') {
                        res.push(TN::from_conditional(Templates::ShortPart(ShortWordPart::from_conditional(a, &b[..b.len()-1])), &condition));
                    }
                    else {
                        res.push(TN::from_conditional(Templates::ShortPart(ShortWordPart::from(&content[1..])), &condition));
                    }
                }
                else if content.starts_with("%") {
                    res.push(TN::from(Templates::Embed(&content[1..])));
                }
                else {
                    let t :Vec<&str> = content.split('-').collect();
                    res.push(TN::from_conditional(Templates::Text(SignText {
                        text: assembling::disassemble(t[0]),
                        name: t[1]
                    }), condition));
                }
            }
            else {
                let t :Vec<&str> = sword.split('-').collect();
                res.push(TN::from(Templates::Text(SignText {
                    text: assembling::disassemble(t[0]),
                    name: t[1]
                })));
            }
        }
    }

    return Ok(res);
}

pub fn parse<'s>(lines :&Vec<&'s str>) -> Result<HashMap<&'s str, PhraseContext<'s>>, usize> {
    let mut res :HashMap<&str, PhraseContext> = HashMap::new();
    let mut st :Vec<Context> = Vec::new();

    for line in lines {
        let words :Vec<&str> = line.split_whitespace().collect();
        if words[0] == "PART" {
            let argc :usize = words.get(2).unwrap_or(&"0").parse().unwrap();
            let p = PhraseContext {
                name: words[1],
                children: Vec::new(),
                argc: argc
            };
            st.push(Context::Phrase(p));
        }
        else if words[0] == "RULE" {
            let r = Rule { 
                rules: parse_rules(&words[2..])?,
                name: words[1]
            };
            if let Context::Phrase(sup) = st.last_mut().unwrap() {
                sup.children.push(PhraseRules::Rules(r));
            }
            else if let Context::If(sup) = st.last_mut().unwrap() {
                sup.children.push(PhraseRules::Rules(r));
            }
        }
        else if words[0] == "SET" {
            let r = Rule { 
                rules: parse_rules(&words[2..])?,
                name: words[1]
            };
            if let Context::Phrase(sup) = st.last_mut().unwrap() {
                sup.children.push(PhraseRules::SetVariable(r));
            }
            else if let Context::If(sup) = st.last_mut().unwrap() {
                sup.children.push(PhraseRules::SetVariable(r));
            }
        }
        else if words[0] == "IF" {
            let argn :usize = words[1].parse().unwrap();
            let p = PhraseIf {
                parameter: argn,
                children: Vec::new(),
                value: words[2],
                unless: false
            };
            st.push(Context::If(p));
        }
        else if words[0] == "UNLESS" {
            let argn :usize = words[1].parse().unwrap();
            let p = PhraseIf {
                parameter: argn,
                children: Vec::new(),
                value: words[2],
                unless: true
            };
            st.push(Context::If(p));
        }
        else if words[0] == "END" {
            let p = st.pop().unwrap();
            if let Context::Phrase(ph) = p {
                res.insert(ph.name, ph);
            }
            else if let Context::If(i) = p {
                if let Context::Phrase(sup) = st.last_mut().unwrap() {
                    sup.children.push(PhraseRules::Ifs(i));
                }
                else if let Context::If(sup) = st.last_mut().unwrap() {
                    sup.children.push(PhraseRules::Ifs(i));
                }
            }
        }
        else if words[0] == "SET" {
            
        }
    };

    return Ok(res);
}