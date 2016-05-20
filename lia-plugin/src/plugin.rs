use syntax::codemap::Span;
use syntax::parse::token::Token;
use syntax::ast::TokenTree;
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacEager};
use lia::token::LiaToken;
use lia;

fn tt_flatten(tt: &TokenTree) -> Vec<Token> {
    match tt {
        &TokenTree::Token(_, ref t) => vec![t.clone()],
        &TokenTree::Delimited(_, ref delim) => {
            let mut toks: Vec<Token> =
                delim.tts.iter().flat_map(tt_flatten).collect();
            toks.insert(0, Token::OpenDelim(delim.delim));
            toks.push(Token::CloseDelim(delim.delim));
            toks
        },
        _ => panic!("TokenTree has Sequence??, {:?}", tt)
    }
}

pub fn expand_lia(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree]) ->
    Box<MacResult + 'static>
{
    let tokens: Vec<LiaToken> =
        args
        .into_iter()
        .flat_map(tt_flatten)
        .map(|t| LiaToken::from_rust_token(t))
        .collect();

    //println!("tokens: {:?}", tokens);

    let ast =
        lia::grammar::parse_fun(tokens)
        .unwrap_or_else(|err| panic!("Parse error {:?}", err));

    super::codegen::top_level(cx, sp, ast)
}
