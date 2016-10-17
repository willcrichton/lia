use super::ast::term::{Term, View, out, into_view, subst};
use syntax::ext::base::ExtCtxt;
use llvm::JitEngine;
use lia_jit::{Jit, JitFun};

type Value = i32;

pub struct EvalState<'a, 'b> {
    pub cx: ExtCtxt<'a>,
    pub jit: Jit<'b, JitEngine>
}

// Thought about implementing small step semantics, but then I got lazy.
// Yay big steps!
pub fn eval<'a, 'b>(st: &mut EvalState<'a, 'b>, expr: Term) -> Term {
    println!("{:?}", expr);
    match out(expr).val {
        e @ View::Number(_) | e @ View::Lam(_) | e @ View::String(_) => into_view(e),
        View::Plus((l, r)) => {
            bind!(View::Number{x} = out(eval(st, l)).val);
            bind!(View::Number{y} = out(eval(st, r)).val);
            into_view(View::Number(x + y))
        },
        View::Let((binding, (var, body))) => {
            let e = eval(st, binding);
            eval(st, subst(e, var, body))
        },
        View::TLet((_, (_, body))) => eval(st, body),
        View::App((fun, arg)) => {
            match out(eval(st, fun)).val {
                View::Lam((var, body)) => {
                    let arg = eval(st, arg);
                    eval(st, subst(arg, var, body))
                },
                View::Quote(parts) => {
                    let s = parts.into_iter().map(|part| match out(part).val {
                        View::String(s) => s,
                        View::Number(n) => n.to_string(),
                        View::Lam(_) => panic!("Lambdas in quotes not implemented"),
                        View::Quote(_) => panic!("Quotes in quotes not implemented"),
                        _ => unreachable!()
                    }).collect::<Vec<String>>().join("");
                    let f: JitFun<i32, i32> = st.jit.gen_fun(s).unwrap();
                    bind!(View::Number{x} = out(arg).val);
                    into_view(View::Number(f(x)))
                },
                _ => unreachable!()
            }
        },
        View::Quote(parts) => {
            into_view(View::Quote(parts.into_iter().map(|part| eval(st, part)).collect()))
        },
        View::Var(_) | View::Var_(_) | View::Dummy => unreachable!(),
    }
}
