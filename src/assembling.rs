// pub fn disassemble<'s>(s :&'s str) -> Vec<char> {
//     let ret :Vec<char> = s.chars().collect();
//     return ret;
// }

// pub fn assemble<'s>(s :&'s str) -> String {
//     String::from(s)
// }

mod hangul;

pub use hangul::disassemble;
pub use hangul::assemble;