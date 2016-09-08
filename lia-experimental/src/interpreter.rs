use super::ast::term::{Term, View, out, into, subst};
use syntax::ext::base::ExtCtxt;
use lia_jit::Jit;

type Value = i32;

pub struct EvalState<'a, 'b> {
    pub cx: ExtCtxt<'a>,
    pub jit: Jit<'b>
}

pub fn eval<'a, 'b>(st: &mut EvalState<'a, 'b>, expr: Term) -> Term {
    println!("{:?}", expr);
    match out(expr) {
        e @ View::Number(_) | e @ View::Lam(_) | e @ View::Foreign => into(e),
        View::Plus((l, r)) => {
            bind!(View::Number{x} = out(eval(st, l)));
            bind!(View::Number{y} = out(eval(st, r)));
            into(View::Number(x + y))
        },
        View::Let((binding, (var, body))) => {
            let e = eval(st, binding);
            eval(st, subst(e, var, body))
        },
        View::App((fun, arg)) => {
            match out(eval(st, fun)) {
                View::Lam(((var, typ), body)) => {
                    let arg = eval(st, arg);
                    eval(st, subst(arg, var, body))
                },
                View::Foreign => {
                    let f = st.jit.run("#[no_mangle] fn foo() -> i32 { 1 + 1 }".to_string()).unwrap();
                    println!("{:?}", f(()));
                    panic!()
                },
                _ => unreachable!()
            }
        },
        View::Quote(_) => {
            into(View::Foreign)
        },
        View::Var(_) => unreachable!()
    }
}
