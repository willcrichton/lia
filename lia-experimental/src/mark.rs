use token::Token;
use grammar;

#[derive(Clone, Debug)]
pub struct Mark {
    pub lo: u32,
    pub hi: u32
}

pub struct Marked<T> {
    pub node: T,
    pub mark: Mark
}

impl grammar::__ToTriple for Marked<Token> {
    type Error = ();
    fn to_triple(value: Self) -> Result<(u32,Token,u32), ()> {
        Ok((value.mark.lo, value.node, value.mark.hi))
    }
}

pub static DUMMY: Mark = Mark {
    lo: 0,
    hi: 0,
};
