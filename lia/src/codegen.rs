use syntax::codemap::{Span, ExpnId, BytePos, Pos};
use syntax::ext::base::ExtCtxt;
use syntax::ast::{Expr, Stmt, Item, Path, Ident, PathSegment, PathParameters};
use syntax::parse::token::{Token as RsToken, BinOpToken, str_to_ident};
use syntax::ptr::P;

use ast::{LiaExpr, LiaStmt, LiaFn, prefix_ident};

fn rs_ident_to_path(mut segs: Vec<Ident>) -> Path {
    {
        let len = segs.len();
        let last = &mut segs[len - 1];
        *last = prefix_ident(last, "_lia_");
    }
    Path {
        span: Span {
            lo: BytePos::from_usize(0),
            hi: BytePos::from_usize(0),
            expn_id: ExpnId::from_u32(0),
        },
        global: false,
        segments: segs.into_iter().map(|seg| PathSegment {
            identifier: seg,
            parameters: PathParameters::none()
        }).collect()
    }
}

fn gen_expr(cx: &mut ExtCtxt, expr: LiaExpr) -> P<Expr> {
    match expr {
        LiaExpr::BinOp(op, box e1, box e2) => {
            let s1 = gen_expr(cx, e1);
            let s2 = gen_expr(cx, e2);
            let (e, ty) = match op {
                RsToken::BinOp(BinOpToken::Plus) =>
                    (quote_expr!(cx, s1v + s2v), quote_ty!(cx, i32)),
                RsToken::BinOp(BinOpToken::Minus) =>
                    (quote_expr!(cx, s1v - s2v), quote_ty!(cx, i32)),
                RsToken::EqEq =>
                    (quote_expr!(cx, s1v == s2v), quote_ty!(cx, i32)),
                RsToken::Le =>
                    (quote_expr!(cx, s1v <= s2v), quote_ty!(cx, i32)),
                RsToken::Lt =>
                    (quote_expr!(cx, s1v < s2v), quote_ty!(cx, i32)),
                _ => panic!("Binop `{:?}` not yet implemented", op)
            };
            // Have to clone s1v and s2v because if s1 is the same variable
            // as s2, then it becomes a double borrow
            quote_expr!(cx, {
                let s1 = $s1;
                let s2 = $s2;
                let s1v = {
                    cast!(let s1v: $ty = s1);
                    s1v
                };
                let s2v = {
                    cast!(let s2v: $ty = s2);
                    s2v
                };
                alloc($e)
            })
        },
        LiaExpr::Integer(n) => {
            quote_expr!(cx, alloc($n))
        },
        LiaExpr::String(s) => {
            quote_expr!(cx, alloc(String::from($s)))
        },
        LiaExpr::Bool(b) => {
            quote_expr!(cx, alloc($b))
        },
        LiaExpr::Var(id) => {
            quote_expr!(cx, $id.clone())
        },
        LiaExpr::RsVar(id) => {
            let new_id = rs_ident_to_path(id);
            quote_expr!(cx, {
                let fun: LiaClosure = Box::new(move |args: Vec<LiaAny>| $new_id(args));
                alloc(fun)
            })
        },
        LiaExpr::Object(kvs) => {
            let kvs: Vec<P<Expr>> = kvs.into_iter().map(|(key, value)| {
                let ke = gen_expr(cx, key);
                let ve = gen_expr(cx, value);
                quote_expr!(cx, {
                    let _key = $ke;
                    let _tmp = _key.borrow();
                    let key = _tmp.borrow();
                    let _val = $ve;
                    let val = _val.borrow();
                    let slot = alloc(());
                    {
                        let mut _tmp = slot.borrow_mut();
                        *_tmp = val.clone();
                    }
                    ht.insert(key.downcast_ref::<String>().expect("Object key must be string").clone(), slot);
                })
            }).collect();
            quote_expr!(cx, {
                let mut ht= new_obj();
                $kvs;
                alloc(ht)
            })
        },
        LiaExpr::Index(box obj, box key) => {
            let obj = gen_expr(cx, obj);
            let key = gen_expr(cx, key);
            // Must get key before object to avoid conflicting borrows, i.e. x[x.y]
            quote_expr!(cx, {
                let key = $key;
                let s = val_to_key(key);
                let obj = $obj;
                let mut _tmp = obj.borrow_mut();
                let mut _tmp = _tmp.borrow_mut();
                let mut ht = _tmp.downcast_mut::<LiaObject>().expect("Can only index into objects");
                fn make_null() -> LiaAny { alloc(LiaNull) }
                (*ht.entry(s).or_insert_with(make_null)).clone()
            })
        },
        // TODO: make this a macro?
        LiaExpr::Array(exprs) => {
            let mut kvs = Vec::new();
            for i in 0..exprs.len() {
                kvs.push((LiaExpr::String(format!("{}", i)), exprs[i].clone()));
            }
            gen_expr(cx, LiaExpr::Object(kvs))
        },
        LiaExpr::Call(box fun, exprs) => {
            let mut exps: Vec<P<Expr>> =
                exprs.into_iter().map(|expr| {
                    let expr = gen_expr(cx, expr);
                    quote_expr!(cx, {args.push($expr)})
                }).collect();

            let call = match fun.clone() {
                LiaExpr::RsVar(id) => {
                    let new_id = rs_ident_to_path(id);
                    quote_expr!(cx, $new_id(args))
                },
                _ => {
                    let f = gen_expr(cx, fun.clone());
                    // Can't borrow_mut as this breaks recursive functions
                    quote_expr!(cx, {
                        let e = $f;
                        let _tmp = e.borrow();
                        let f = _tmp.borrow();
                        (f.downcast_ref::<LiaClosure>().expect("Invalid closure"))(args)
                    })
                }
            };

            match fun.clone() {
                LiaExpr::Index(box context, _) => {
                    let expr = gen_expr(cx, context);
                    exps.insert(0, quote_expr!(cx, {args.push($expr)}));
                },
                _ => {
                    exps.insert(0, quote_expr!(cx, {args.push({
                        alloc(new_obj())
                    })}));
                }
            };

            quote_expr!(cx, {
                let mut args = Vec::new();
                $exps
                $call
            })
        },
        LiaExpr::Closure(mut args, stmts) => {
            use std::collections::{HashMap, HashSet};
            args.insert(0, str_to_ident("this"));
            let mut copies = Vec::new();
            let stmts = {
                let mut bound = HashSet::new();
                let mut mapping = HashMap::new();
                let mut e = LiaExpr::Closure(args.clone(), stmts);
                e.remap_free_vars(&mut bound, &mut mapping);

                for (src, dst) in &mapping {
                    copies.push(quote_stmt!(cx, let $dst = $src.clone();)
                                .expect("Invalid stmt"));
                }

                match e {
                    LiaExpr::Closure(_, stmts) => stmts,
                    _ => unreachable!()
                }
            };

            let st: Vec<Stmt> = stmts.into_iter().flat_map(|stmt| gen_stmt(cx, stmt)).collect();
            let mut binds = vec![];
            for i in 0..args.len() {
                let arg_id = args[i];
                let s = format!("Arg {} missing", i);
                binds.push(quote_stmt!(cx, let $arg_id = args.get($i).expect($s).clone())
                           .expect("Invalid stmt"));
            }

            // Not clear what the type of the closure is by default. Have to explicitly cast it.
            quote_expr!(cx, {
                $copies;
                let fun: LiaClosure = Box::new(move |args: Vec<LiaAny>| {
                    $binds;
                    $st;
                    return alloc(());
                });
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
        LiaStmt::Assign(lhs, rhs) => {
            let lhs = gen_expr(cx, lhs);
            let rhs = gen_expr(cx, rhs);
            vec![quote_stmt!(cx, {
                let lhs = $lhs;
                let rhs = $rhs;
                let made_it = {
                    let _tmp = rhs.borrow_mut();
                    let src = _tmp.borrow_mut();
                    let _tmp = lhs.borrow_mut();
                    let mut dst = _tmp.borrow_mut();
                    if src.is::<i32>() {
                        *dst = Box::new(*src.downcast_ref::<i32>().expect("Invalid i32"));
                        true
                    } else if src.is::<bool>() {
                        *dst = Box::new(*src.downcast_ref::<bool>().expect("Invalid bool"));
                        true
                    } else {
                        false
                    }
                };
                if !made_it {
                    let mut dst = lhs.borrow_mut();
                    let src = rhs.borrow_mut();
                    *dst = src.clone();
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
        LiaStmt::If(cond, if_, else_) => {
            let e = gen_expr(cx, cond);
            let if_: Vec<Stmt> = if_.into_iter().flat_map(|stmt| gen_stmt(cx, stmt)).collect();
            let else_: Vec<Stmt> = match else_ {
                Some(else_) => else_.into_iter().flat_map(|stmt| gen_stmt(cx, stmt)).collect(),
                None => vec![]
            };
            vec![quote_stmt!(cx, {
                let e = $e;
                let _tmp = e.borrow_mut();
                let cond = _tmp.borrow_mut();
                let b = if cond.is::<bool>() {
                    *cond.downcast_ref::<bool>().unwrap()
                } else if cond.is::<i32>() {
                    let n = *cond.downcast_ref::<i32>().unwrap();
                    n != 0
                } else { !cond.is::<()>() && !cond.is::<LiaNull>() };
                if b { $if_; }
                else { $else_; }
            }).expect("Invalid if stmt")]
        },
        LiaStmt::While(guard, body) => {
            let guard = gen_expr(cx, guard);
            let body: Vec<Stmt> =
                body.into_iter().flat_map(|stmt| gen_stmt(cx, stmt)).collect();
            vec![quote_stmt!(cx, {
                while {
                    let e = $guard;
                    cast!(let b: bool = e);
                    b
                } { $body; }
            }).expect("Invalid while stmt")]
        },
        LiaStmt::ForObj(id, expr, body) => {
            let expr = gen_expr(cx, expr);
            let body: Vec<Stmt> =
                body.into_iter().flat_map(|stmt| gen_stmt(cx, stmt)).collect();
            vec![quote_stmt!(cx, {
                {
                    let e = $expr;
                    let keys = {
                        cast!(let obj: LiaObject = e);
                        let keys: Vec<String> = obj.keys().map(|s| s.clone()).collect();
                        keys
                    };
                    for $id in keys {
                        let $id = alloc($id);
                        $body;
                    }
                }
            }).expect("Invalid for stmt")]
        }
    }
}

pub fn gen_fn(cx: &mut ExtCtxt, mut fun: LiaFn) -> P<Item> {
    let st: Vec<Stmt> = fun.body.into_iter().flat_map(|stmt| gen_stmt(cx, stmt)).collect();
    let id = prefix_ident(&fun.name, "_lia_");
    let mut binds = vec![];
    fun.args.insert(0, str_to_ident("this"));
    for i in 0..fun.args.len() {
        let arg_id = fun.args[i];
        let s = format!("Arg {}", i);
        binds.push(quote_stmt!(cx, let $arg_id = args.get($i).expect($s).clone()).unwrap());
    }

    quote_item!(
        cx,
        #[allow(unreachable_code, dead_code, unused_mut, unused_assignments, unused_parens, unused_variables)]
        fn $id (args: Vec<LiaAny>) -> LiaAny {
            $binds;
            $st;
            return alloc(());
        }
    ).unwrap()
}
