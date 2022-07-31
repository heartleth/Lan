#[derive(Clone, Debug)]
pub struct Morpheme {
    pub text: String,
    pub name: String
}

pub type Morphemes = Vec<Morpheme>;