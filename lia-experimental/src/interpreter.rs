use super::ast::term::{Term, View, out, into, subst};
use syntax::ext::base::ExtCtxt;
use llvm::JitEngine;
use lia_jit::{Jit, JitFun};

type Value = i32;

pub struct EvalState<'a, 'b> {
    pub cx: ExtCtxt<'a>,
    pub jit: Jit<'b, JitEngine>
}

pub fn eval<'a, 'b>(st: &mut EvalState<'a, 'b>, expr: Term) -> Term {
    match out(expr) {
        e @ View::Number(_) | e @ View::Lam(_) | e @ View::Quote(_) => into(e),
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
                View::Quote(s) => {
                    let f: JitFun<i32, i32> = st.jit.gen_fun(s).unwrap();
                    bind!(View::Number{x} = out(arg));
                    into(View::Number(f(x)))
                },
                _ => unreachable!()
            }
        },
        View::Var(_) => unreachable!()
    }
}
