use super::ast::term::{Term, View, out, into, subst};

type Value = i32;

pub fn eval(expr: Term) -> Term {
    match out(expr) {
        e @ View::Number(_) | e @ View::Lam(_) => into(e),
        View::Plus((l, r)) =>
            match (out(eval(l)), out(eval(r))) {
                (View::Number(x), View::Number(y)) => into(View::Number(x + y)),
                _ => unreachable!()
            },
        View::Let((binding, (var, body))) => {
            let e = eval(binding);
            eval(subst(e, var, body))
        },
        View::App((fun, arg)) =>
            match out(eval(fun)) {
                View::Lam((var, body)) => {
                    eval(subst(eval(arg), var, body))
                },
                _ => unreachable!()
            },
        View::Var(_) => unreachable!()
    }
}
