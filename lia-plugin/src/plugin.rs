use syntax::codemap::{Span, Spanned};
use syntax::parse::token::Token;
use syntax::ast::*;
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, Annotatable};
use syntax::ext::build::AstBuilder;
use lia::token::LiaToken;
use lia::ast::prefix_ident;
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
        lia::grammar::parse_funs(tokens)
        .unwrap_or_else(|err| panic!("Parse error {:?}", err));

    let ast = lia::elaborate::elaborate(ast);

    super::codegen::top_level(cx, sp, ast)
}

pub fn impl_glue(cx: &mut ExtCtxt, sp: Span, mitem: &MetaItem, item: Annotatable)
                 -> Annotatable
{
    match item {
        Annotatable::Item(ref it) => {
            if let ItemKind::Impl(unsafety, polarity, ref generics,
                                  ref traitref, ref impl_ty, ref items)
                = it.node
            {
                let new_items: Vec<ImplItem> = items.iter().flat_map(|impl_item| {
                    let mut items = vec![impl_item.clone()];
                    if let ImplItemKind::Method(ref sig, ref body) = impl_item.node {
                        let ref inputs = sig.decl.inputs;
                        let mut binds = vec![];
                        for i in 0..inputs.len() {
                            let s = format!("Arg {}", i);
                            binds.push(match &inputs[i].pat.node {
                                &PatKind::Ident(_, ref ident, _) => {
                                    let mut id = ident.node;
                                    let ty = if id.name.as_str() == "self" {
                                        id = prefix_ident(&id, "_lia_");
                                        impl_ty
                                    } else {
                                        &inputs[i].ty
                                    };
                                    quote_block!(
                                        cx,
                                        {cast!(let $id: $ty = args.get($i).expect($s));}
                                        ).unwrap().stmts
                                },
                                _ => panic!("#[lia_impl_glue] only supports methods with no pattern matching in the arguments")
                            });
                        }
                        let binds: Vec<Stmt> = binds.into_iter().flat_map(|e| e).collect();
                        let fun =
                            quote_item!(
                                cx,
                                fn _ignore_this_name (args: Vec<LiaAny>) -> LiaAny {
                                    $binds;
                                    alloc((||$body)())
                                }).unwrap();
                        if let ItemKind::Fn(decl, unsafety, constness, abi,
                                            generics, block)
                            = fun.node.clone()
                        {
                            let mut new_item = impl_item.clone();
                            new_item.ident = prefix_ident(&new_item.ident, "_lia_");
                            new_item.node =
                                ImplItemKind::Method(MethodSig {
                                    unsafety: unsafety,
                                    constness: constness,
                                    abi: abi,
                                    decl: decl,
                                    generics: generics,
                                    explicit_self: Spanned {
                                        node: SelfKind::Static,
                                        span: fun.span,
                                    },
                                }, block);
                            items.push(new_item);
                        } else {
                            unreachable!()
                        }
                    }
                    items
                }).collect();
                let newimpl = ItemKind::Impl(
                    unsafety, polarity, generics.clone(), traitref.clone(),
                    impl_ty.clone(), new_items);
                Annotatable::Item(cx.item(sp, it.ident, it.attrs.clone(), newimpl))
            } else {
                cx.span_err(sp, "balls 2: electric boogaloo");
                item.clone()
            }
        },
        _ => {
            cx.span_err(sp, "balls");
            item
        }
    }
}
