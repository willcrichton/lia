use lia::ast::{LiaExpr, LiaStmt, LiaFn};
use syntax::codemap::{Span, ExpnId, BytePos};
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacEager};
use syntax::util::small_vector::SmallVector as Svec;
use syntax::ast::{Expr, Stmt, Item};
use syntax::ptr::P;

fn gen_expr(cx: &mut ExtCtxt, expr: LiaExpr) -> P<Expr> {
    match expr {
        LiaExpr::Binop(op, box e1, box e2) => {
            let s1 = gen_expr(cx, e1);
            let s2 = gen_expr(cx, e2);
            quote_expr!(cx, {
                let s1 = $s1;
                let s2 = $s2;
                let mut s1m = s1.borrow_mut();
                let mut s2m = s2.borrow_mut();
                lia::runtime::alloc(
                    *s1m.downcast_mut::<i32>().unwrap() +
                    *s2m.downcast_mut::<i32>().unwrap())
            })
        },
        LiaExpr::Integer(n) => {
            quote_expr!(cx, lia::runtime::alloc($n))
        },
        LiaExpr::Var(id) => {
            quote_expr!(cx, $id)
        },
    }
}

fn gen_stmt(cx: &mut ExtCtxt, stmt: LiaStmt) -> Stmt {
    match stmt {
        LiaStmt::Assign(id, expr) => {
            let e = gen_expr(cx, expr);
            quote_stmt!(cx, let $id = $e;).unwrap()
        },
        LiaStmt::Return(expr) => {
            let e = gen_expr(cx, expr);
            quote_stmt!(cx, return $e;).unwrap()
        },
    }
}

fn gen_fn(cx: &mut ExtCtxt, fun: LiaFn) -> P<Item> {
    let LiaFn(id, stmts) = fun;
    let st: Vec<Stmt> = stmts.into_iter().map(|stmt| gen_stmt(cx, stmt)).collect();
    quote_item!(cx,
        #[allow(unreachable_code, dead_code)]
        fn $id () -> lia::runtime::LiaAny {
            $st;
            return lia::runtime::alloc(());
        }
    ).unwrap()
}

pub fn top_level(cx: &mut ExtCtxt, sp: Span, fun: LiaFn) -> Box<MacResult+ 'static>
{
    MacEager::items(Svec::one(gen_fn(cx, fun)))
}
