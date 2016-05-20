use super::ast::{LiaExpr, LiaStmt, LiaFn};
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
            quote_expr!(cx, $s1 + $s2)
        },
        LiaExpr::Integer(n) => {
            quote_expr!(cx, $n)
        }
    }
}

fn gen_stmt(cx: &mut ExtCtxt, stmt: LiaStmt) -> Stmt {
    quote_stmt!(cx, let x = 1).unwrap()
}

fn gen_fn(cx: &mut ExtCtxt, fun: LiaFn) -> P<Item> {
    let LiaFn(id, stmt) = fun;
    let stmt_quot = gen_stmt(cx, stmt);
    quote_item!(cx,
        fn $id () -> lia::LiaAny {
            $stmt_quot;
            lia::alloc(())
        }
    ).unwrap()
}

pub fn top_level(cx: &mut ExtCtxt, sp: Span, fun: LiaFn) -> Box<MacResult+ 'static>
{
    MacEager::items(Svec::one(gen_fn(cx, fun)))
}
