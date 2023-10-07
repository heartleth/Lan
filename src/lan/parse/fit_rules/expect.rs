use super::SyntaxTreeNode;
use super::ParsingRule;

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
        if let Some(r) = self.rule.get(0) {
            if !r.is_free {
                self.rule = &self.rule[1..];
            }
        }
        else {
            self.rule = &self.rule[1..];
        }
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
                p.voca_attrs = self.voca_attrs.clone();
                ret.push(p);
            }
            else if r.is_free {
                let mut p = Expectation::from(self.name, &self.rule[i..], self.tree.clone().unwrap(), self.reading);
                p.voca_attrs = self.voca_attrs.clone();
                ret.push(p);
            }
            else {
                let mut p = Expectation::from(self.name, &self.rule[i..], self.tree.take().unwrap(), self.reading);
                p.voca_attrs = self.voca_attrs.clone();
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
        if r.is_optional || r.is_free {
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