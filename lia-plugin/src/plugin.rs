use syntax::codemap::{Span, Spanned};
use syntax::parse::token::Token;
use syntax::ast::*;
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, Annotatable};
use syntax::ext::build::AstBuilder;
use syntax::ext::mtwt;
use syntax::fold;
use syntax::fold::Folder;

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

// Almost identical copy of the one from syntax::ext::expand, copied because
// their `renames` field is private.
pub struct IdentRenamer {
    renames: mtwt::RenameList,
}

impl Folder for IdentRenamer {
    fn fold_ident(&mut self, id: Ident) -> Ident {
        Ident::new(id.name, mtwt::apply_renames(&self.renames, id.ctxt))
    }
    fn fold_mac(&mut self, mac: Mac) -> Mac {
        fold::noop_fold_mac(mac, self)
    }
}

/************************** BERRY FINDER **************************/
// fn perry(berry) {
//     find(maryberry);
//     if not find(maryberry){ panic!() }
//     if not find(willberry){kind_of_panic!() }
//     else {
//         panic_more!()
//     }
// }
// pub panic!()
//     hfxzlbkz n;lxz nzs

//     ;bnsvakbsvain28e9848tht49et6r7e8930rt8r9e098rtre;
// ckhvvk nk;`
//     panic!()  nvkd
//     }

// C++++++++++++++1

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

                        let mut new_body = body.clone();

                        for i in 0..inputs.len() {
                            let s = format!("Arg {}", i);
                            binds.push(match &inputs[i].pat.node {
                                &PatKind::Ident(_, ref ident, _) => {
                                    let mut id = ident.node;
                                    let is_self = id.name.as_str() == "self";
                                    let ty = if is_self {
                                        let new_id = prefix_ident(&id, "_lia_");
                                        let rename = (id.clone(), new_id.name);
                                        let mut renamer = IdentRenamer {
                                            renames: vec![rename]
                                        };
                                        new_body = renamer.fold_block(new_body);

                                        id = new_id;
                                        impl_ty.clone()
                                    } else {
                                        inputs[i].ty.clone()
                                    };


                                    let (ty, is_ref) = match ty.clone().node {
                                        TyKind::Rptr(_, ref mut_ty) => {
                                            (mut_ty.ty.clone(), true)
                                        },
                                        _ => (ty, false)
                                    };

                                    let cast = if is_ref || is_self {
                                        quote_stmt!(cx, let mut $id = $id).unwrap()
                                    } else {
                                        quote_stmt!(cx, let mut $id = *$id).unwrap()
                                    };

                                    quote_block!(
                                        cx,
                                        {
                                            cast!(let $id: $ty = args.get($i).expect($s));
                                            $cast;
                                        }
                                        ).unwrap().stmts
                                },
                                _ => panic!("#[lia_impl_glue] only supports methods with no pattern matching in the arguments")
                            });
                        }
                        let binds: Vec<Stmt> = binds.into_iter().flat_map(|e| e).collect();
                        // TODO: attrs for ignoring warnings on fun?
                        let fun =
                            quote_item!(
                                cx,
                                fn _ignore_this_name (args: Vec<LiaAny>) -> LiaAny {
                                    $binds;
                                    alloc((move ||$new_body)())
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
                cx.span_err(sp, "i pooped myself: excuse me my baby?");
                item.clone()
            }
        },
        _ => {
            cx.span_err(sp, "m&ms");
            item
        }
    }
}
