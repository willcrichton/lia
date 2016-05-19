use syntax::codemap::Span;
use syntax::parse::token;
use syntax::ast::TokenTree;
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacEager};
use syntax::ext::build::AstBuilder;  // trait for expr_usize

pub fn expand_lia(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree]) ->
    Box<MacResult + 'static>
{
    // if args.len() != 1 {
    //     cx.span_err(sp, "only takes one argument");
    //     return DummyResult::any(sp);
    // }

    let tokens: Vec<token::Token> = args.into_iter().map(|tok| {
        match tok {
            &TokenTree::Token(_, ref t) => t.clone(),
            _ => panic!("TokenTree has non-tokens?")
        }
    }).collect();

    let ast = super::grammar::parse_expr(tokens).unwrap_or_else(|err| {
        match err {
            _ => panic!("Parse error")
        }
    });

    super::codegen::gen_expr(cx, sp, ast)
}
