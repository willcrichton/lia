use super::ast::*;

// This doesn't do any meaningful elaboration right now. It just "typechecks" the
// LHS of an assign.

pub fn elaborate(funs: Vec<LiaFn>) -> Vec<LiaFn> {
    funs.into_iter().map(elaborate_fun).collect()
}

fn elaborate_fun(fun: LiaFn) -> LiaFn {
    LiaFn {
        name: fun.name,
        args: fun.args,
        body: fun.body.into_iter().map(elaborate_stmt).collect()
    }
}

fn elaborate_stmt(stmt: LiaStmt) -> LiaStmt {
    match stmt.clone() {
        LiaStmt::Assign(lhs, _) => {
            match lhs {
                LiaExpr::Var(_) | LiaExpr::Index(_, _) => stmt,
                _ => panic!("Invalid LHS of assign")
            }
        },
        _ => stmt
    }
}
