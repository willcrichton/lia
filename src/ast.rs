use syntax::parse::token::{Token, BinOpToken};
use syntax::ast::Ident;

#[derive(Debug)]
pub enum LiaExpr {
    Binop(BinOpToken, Box<LiaExpr>, Box<LiaExpr>),
    Integer(i32),
}

#[derive(Debug)]
pub enum LiaStmt {
    Assign(Ident, LiaExpr)
}

#[derive(Debug)]
pub struct LiaFn(pub Ident, pub LiaStmt);
