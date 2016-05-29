use lia::ast::{LiaExpr, LiaStmt, LiaFn};
use syntax::codemap::{Span, ExpnId, BytePos};
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacEager};
use syntax::util::small_vector::SmallVector as Svec;
use syntax::ast::{Expr, Stmt, Item};
use syntax::parse::token::BinOpToken;
use syntax::ptr::P;

fn gen_expr(cx: &mut ExtCtxt, expr: LiaExpr) -> P<Expr> {
    match expr {
        LiaExpr::Binop(op, box e1, box e2) => {
            let s1 = gen_expr(cx, e1);
            let s2 = gen_expr(cx, e2);
            let e = match op {
                BinOpToken::Plus => quote_expr!(cx, *s1v + *s2v),
                BinOpToken::Minus => quote_expr!(cx, *s1v - *s2v),
                _ => panic!("Not yet implemented"),
            };
            quote_expr!(cx, {
                let s1 = $s1;
                let s2 = $s2;
                cast!(s1v, s1, i32);
                cast!(s2v, s2, i32);
                alloc($e)
            })
        },
        LiaExpr::Equals(box e1, box e2) => {
            let s1 = gen_expr(cx, e1);
            let s2 = gen_expr(cx, e2);
            quote_expr!(cx, {
                let s1 = $s1;
                let s2 = $s2;
                cast!(s1v, s1, i32);
                cast!(s2v, s2, i32);
                alloc(*s1v == *s2v)
            })
        },
        LiaExpr::Integer(n) => {
            quote_expr!(cx, alloc($n))
        },
        LiaExpr::Var(id) => {
            quote_expr!(cx, $id.clone())
        },
        LiaExpr::RsVar(id) => {
            quote_expr!(cx, {
                let fun: LiaClosure = Box::new(move |args: Vec<LiaAny>| $id(args));
                alloc(fun)
            })
        },
        LiaExpr::Call(box fun, exprs) => {
            let f = gen_expr(cx, fun);
            let exps: Vec<P<Expr>> =
                exprs.into_iter().map(|expr| gen_expr(cx, expr)).collect();
            quote_expr!(cx, {
                let e = $f;
                let f = e.lock().expect("Lock was invalid");
                (f.downcast_ref::<LiaClosure>().expect("Invalid closure"))(vec![$exps])
            })
         },
        LiaExpr::Closure(mut stmts) => {
            use std::collections::{HashMap, HashSet};
            let mut bound = HashSet::new();
            let mut mapping = HashMap::new();
            for mut s in stmts.iter_mut() {
                s.remap_free_vars_aux(&mut bound, &mut mapping);
            }

            let mut copies = Vec::new();
            for (src, dst) in &mapping {
                copies.push(quote_stmt!(cx, let $dst = $src.clone();).expect("Invalid stmt"));
            }

            let st: Vec<Stmt> = stmts.into_iter().flat_map(|stmt| gen_stmt(cx, stmt)).collect();
            // Not clear what the type of the closure is by default. Have to explicitly
            // cast it.
            quote_expr!(cx, {
                $copies;
                let fun: LiaClosure = Box::new(move |args: Vec<LiaAny>| { $st; return alloc(()); });
                alloc(fun)
            })
        }
    }
}


fn gen_stmt(cx: &mut ExtCtxt, stmt: LiaStmt) -> Vec<Stmt> {
    match stmt {
        LiaStmt::Declare(id) => {
            vec![quote_stmt!(cx, let $id = alloc(());).expect("Invalid stmt")]
        },
        LiaStmt::Assign(id, expr) => {
            let e = gen_expr(cx, expr);
            vec![quote_stmt!(cx, {
                let e = $e;
                let src = e.lock().expect("Invalid lock");
                let copy = $id.clone();
                let mut dst = copy.lock().expect("Invalid lock");
                // ints by value, all else by reference
                if src.is::<i32>() {
                    *dst = Box::new(*src.downcast_ref::<i32>().expect("Invalid i32"));
                } else {
                    *dst = Box::new(src.downcast_ref::<LiaAny>().expect("Invalid reference").clone());
                }
            }).unwrap()]
        },
        LiaStmt::Return(expr) => {
            let e = gen_expr(cx, expr);
            vec![quote_stmt!(cx, return $e;).expect("Invalid return stmt")]
        },
        LiaStmt::Expr(expr) => {
            let e = gen_expr(cx, expr);
            vec![quote_stmt!(cx, let _ = $e;).expect("Invalid expr stmt")]
        },
        LiaStmt::If(cond, body) => {
            let e = gen_expr(cx, cond);
            let st: Vec<Stmt> = body.into_iter().flat_map(|stmt| gen_stmt(cx, stmt)).collect();
            vec![quote_stmt!(
                cx,
                {
                    let e = $e;
                    let cond = e.lock().expect("Invalid lock");
                    let b = if cond.is::<i32>() {
                        let n = *cond.downcast_ref::<i32>().unwrap();
                        n != 0
                    } else if cond.is::<bool>() {
                        *cond.downcast_ref::<bool>().unwrap()
                    } else { !cond.is::<()>() };
                    if b { $st; }
                }).expect("Invalid if stmt")]
        }
    }
}

fn gen_fn(cx: &mut ExtCtxt, fun: LiaFn) -> P<Item> {
    let st: Vec<Stmt> = fun.body.into_iter().flat_map(|stmt| gen_stmt(cx, stmt)).collect();
    let id = fun.name;
    let mut binds = vec![];
    for i in 0..fun.args.len() {
        let arg_id = fun.args[i];
        binds.push(quote_stmt!(cx, let $arg_id = args.get($i).unwrap().clone()).expect("Invalid stmt"));
    }

    quote_item!(
        cx,
        #[allow(unreachable_code, dead_code, unused_mut, unused_assignments, unused_parens)]
        fn $id (args: Vec<LiaAny>) -> LiaAny {
            $binds;
            $st;
            return alloc(());
        }
    ).unwrap()
}

pub fn top_level(cx: &mut ExtCtxt, sp: Span, funs: Vec<LiaFn>) -> Box<MacResult+ 'static>
{
    let fs: Vec<P<Item>> = funs.into_iter().map(|fun| gen_fn(cx, fun)).collect();
    MacEager::items(Svec::many(fs))
}
