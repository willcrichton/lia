use syntax::parse::token::{Token, BinOpToken, Lit};

#[derive(Debug)]
pub enum Expr {
    Binop(BinOpToken, Box<Expr>, Box<Expr>),
    Literal(Lit),
}
