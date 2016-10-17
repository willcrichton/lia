use std::fmt;
use std::default::Default;
use token::Token;
use grammar;

#[derive(Clone, Debug)]
pub struct Mark {
    pub lo: u32,
    pub hi: u32
}

pub static DUMMY: Mark = Mark {
    lo: 0,
    hi: 0,
};

impl Default for Mark {
    fn default() -> Mark { DUMMY.clone() }
}

#[derive(Clone)]
pub struct Marked<T> {
    pub node: T,
    pub mark: Mark
}

impl<T: fmt::Debug> fmt::Debug for Marked<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.node.fmt(f)
    }
}

impl grammar::__ToTriple for Marked<Token> {
    type Error = ();
    fn to_triple(value: Self) -> Result<(u32,Token,u32), ()> {
        Ok((value.mark.lo, value.node, value.mark.hi))
    }
}
