use super::ast;
use syntax::codemap::{Span, ExpnId, BytePos};
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacEager};
use syntax::util::small_vector::SmallVector as Svec;
use syntax::ast::Expr;
use syntax::ptr::P;

fn _gen_expr(cx: &mut ExtCtxt, expr: ast::Expr) -> P<Expr> {
    match expr {
        ast::Expr::Binop(op, box e1, box e2) => {
            let s1 = _gen_expr(cx, e1);
            let s2 = _gen_expr(cx, e2);
            quote_expr!(cx, $s1 + $s2)
        },
        ast::Expr::Literal(lit) => {
            use syntax::errors::Handler;
            use syntax::ext::tt::transcribe::new_tt_reader;
            use syntax::ast::TokenTree;
            use syntax::parse::token::Token;
            use syntax::parse::parser::Parser;
            use syntax::parse::ParseSess;
            use syntax::errors::json::JsonEmitter;

            let emitter = JsonEmitter::basic();
            let handler = Handler::with_emitter(false, true, Box::new(emitter));
            let span = Span {
                lo: BytePos(0),
                hi: BytePos(0),
                expn_id: ExpnId::from_u32(0)
            };
            let reader =
                new_tt_reader(&handler, None, None,
                              vec![TokenTree::Token(span, Token::Literal(lit, None))]);
            let parse_sess = ParseSess::new();
            let mut parser = Parser::new(&parse_sess, vec![], Box::new(reader));
            let parsed_lit = parser.parse_lit().unwrap();
            quote_expr!(cx, $parsed_lit)
        }
    }
}

pub fn gen_expr(cx: &mut ExtCtxt, sp: Span, expr: ast::Expr) -> Box<MacResult+ 'static>
{
    MacEager::expr(_gen_expr(cx, expr))
}
