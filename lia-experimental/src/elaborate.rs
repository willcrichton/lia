use std::collections::HashMap;
use super::ast::*;

type BindMap = HashMap<String, Vec<i32>>;

pub fn elaborate(expr: Expr) -> Expr {
    elab_expr(expr, &mut HashMap::new(), 0)
}

pub fn elab_expr(expr: Expr, binds: &mut BindMap, depth: i32) -> Expr {
    match expr {
        Expr::Let(id, box bind, box body) => {
            let bind_new = elab_expr(bind, binds, depth);
            {
                let depths = binds.entry(id.string()).or_insert(Vec::new());
                depths.push(depth);
            }
            let body_new = elab_expr(body, binds, depth + 1);
            {
                let depths = binds.entry(id.string()).or_insert(Vec::new());
                depths.pop();
            }
            Expr::Let(id, box bind_new, box body_new)
        },
        Expr::Plus(box l, box r) =>
            Expr::Plus(box elab_expr(l, binds, depth), box elab_expr(r, binds, depth)),
        Expr::Seq(box l, box r) =>
            Expr::Seq(box elab_expr(l, binds, depth), box elab_expr(r, binds, depth)),
        Expr::Id(id) =>
            Expr::Id(Id::Index(*(binds.get(&id.string()).unwrap().last().unwrap()))),
        _ => expr
    }
}
