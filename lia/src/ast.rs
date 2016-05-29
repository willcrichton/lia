use std::collections::{HashMap, HashSet};
use syntax::parse::token::{Token, BinOpToken, intern};
use syntax::ast::Ident;

#[derive(Debug, Clone)]
pub enum LiaExpr {
    Binop(BinOpToken, Box<LiaExpr>, Box<LiaExpr>),
    Integer(i32),
    Var(Ident),
    RsVar(Ident),
    Call(Box<LiaExpr>, Vec<LiaExpr>),
    Closure(Vec<LiaStmt>),
    Equals(Box<LiaExpr>, Box<LiaExpr>),
}

#[derive(Debug, Clone)]
pub enum LiaStmt {
    Declare(Ident),
    Assign(Ident, LiaExpr),
    Return(LiaExpr),
    Expr(LiaExpr),
    If(LiaExpr, Vec<LiaStmt>),
}

#[derive(Debug, Clone)]
pub struct LiaFn {
    pub name: Ident,
    pub args: Vec<Ident>,
    pub body: Vec<LiaStmt>,
}

fn get_mapping(mapping: &mut HashMap<Ident, Ident>, id: &Ident) -> Ident {
    if !mapping.contains_key(id) {
        let name = intern(format!("_{}_copy", id.name.as_str()).as_str());
        mapping.insert(id.clone(), Ident::with_empty_ctxt(name));
    };
    mapping.get(id).expect("Free mapping was invalid").clone()
}

impl LiaExpr {
    pub fn remap_free_vars(
        &mut self,
        bound: &mut HashSet<Ident>,
        mapping: &mut HashMap<Ident, Ident>)
    {
        use self::LiaExpr::*;
        match self {
            &mut Var(ref mut id) => {
                if !bound.contains(id) {
                    *id = get_mapping(mapping, id);
                }
            },
            &mut Closure(ref mut stmts) => {
                for mut s in stmts.iter_mut() {
                    s.remap_free_vars_aux(bound, mapping);
                }
            },
            &mut Binop(_, ref mut left, ref mut right) => {
                left.remap_free_vars(bound, mapping);
                right.remap_free_vars(bound, mapping);
            },
            &mut Call(ref mut fun, ref mut args) => {
                fun.remap_free_vars(bound, mapping);
                for arg in args.iter_mut() {
                    arg.remap_free_vars(bound, mapping);
                }
            },
            _ => ()
        }
    }
}

impl LiaStmt {
    pub fn remap_free_vars(&mut self) -> HashMap<Ident, Ident> {
        let mut bound = HashSet::new();
        let mut mapping = HashMap::new();
        self.remap_free_vars_aux(&mut bound, &mut mapping);
        mapping
    }

    pub fn remap_free_vars_aux(
        &mut self,
        bound: &mut HashSet<Ident>,
        mapping: &mut HashMap<Ident, Ident>)
    {
        use self::LiaStmt::*;
        match self {
            &mut Declare(id) => {
                bound.insert(id);
            },
            &mut Assign(ref mut id, ref mut expr) => {
                expr.remap_free_vars(bound, mapping);
                if !bound.contains(id) {
                    *id = get_mapping(mapping, id);
                }
            },
            &mut Return(ref mut expr) => {
                expr.remap_free_vars(bound, mapping);
            },
            &mut Expr(ref mut expr) => {
                expr.remap_free_vars(bound, mapping);
            },
            &mut If(ref mut expr, ref mut stmts) => {
                expr.remap_free_vars(bound, mapping);
                for s in stmts.iter_mut() {
                    s.remap_free_vars_aux(bound, mapping);
                }
            }
        }
    }
}
