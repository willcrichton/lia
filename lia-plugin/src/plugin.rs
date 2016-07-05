use syntax::codemap::Span;
use syntax::parse::token::Token;
use syntax::parse::token;
use syntax::ast::*;
use syntax::ext::base::{ExtCtxt, MacResult, MacEager, Annotatable};
use syntax::ext::build::AstBuilder;
use syntax::fold;
use syntax::fold::Folder;
use syntax::util::small_vector::SmallVector as Svec;
use syntax::ptr::P;

use lia::token::LiaToken;
use lia::ast::prefix_ident;
use lia::codegen::gen_fn;
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

#[allow(unused_variables)]
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

    let fs: Vec<P<Item>> = ast.into_iter().map(|fun| gen_fn(cx, fun)).collect();
    MacEager::items(Svec::many(fs))
}

#[allow(unused_variables)]
pub fn expand_borrow_type(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree]) ->
    Box<MacResult + 'static>
{
    use syntax::parse::tts_to_parser;

    let mut parser = tts_to_parser(cx.parse_sess, args.to_vec(), Vec::new());
    let id = match parser.parse_ident() {
        Ok(id) => id,
        Err(_) => panic!("Invalid ident"),
    };

    let ty = match parser.parse_ty() {
        Ok(ty) => ty,
        Err(_) => panic!("Invalid ty"),
    };

    let ty_str = format!("Invalid cast to {:?}", ty);
    let expr = match &ty.node {
        &TyKind::Rptr(_, ref mutty) => {
            let sub_ty = mutty.ty.clone();
            quote_expr!(cx, {
                $id.downcast_mut::<$sub_ty>().expect($ty_str)
            })
        },
        &TyKind::Path(_, ref path) => {
            quote_expr!(cx, {
                $id.downcast_mut::<$ty>().expect($ty_str).clone()
            })
        },
        _ => panic!("Type isn't yet supported for casting with Lia"),
    };

    MacEager::expr(expr)
}

struct SelfRenamer {
    id: Ident
}

impl Folder for SelfRenamer {
    fn fold_ident(&mut self, id: Ident) -> Ident {
        if id.name.as_str() == "self" {
            self.id
        } else {
            id
        }
    }

    fn fold_mac(&mut self, mac: Mac) -> Mac {
        fold::noop_fold_mac(mac, self)
    }
}

#[allow(unused_variables)]
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
                            let bind = match &inputs[i].pat.node {
                                &PatKind::Ident(_, ref ident, _) => {
                                    let mut id = ident.node;
                                    let is_self = id.name.as_str() == "self";
                                    let ty = if is_self {
                                        let new_id = token::str_to_ident("_lia_self");
                                        let mut renamer = SelfRenamer {id: new_id};
                                        new_body = renamer.fold_block(new_body);

                                        id = new_id;
                                        let ty = impl_ty.clone();
                                        match sig.decl.get_self().unwrap().node.clone() {
                                            SelfKind::Region(life, muty) => P(Ty {
                                                id: ty.id,
                                                span: ty.span,
                                                node: TyKind::Rptr(life, MutTy {
                                                    ty: impl_ty.clone(),
                                                    mutbl: muty,
                                                })
                                            }),
                                            _ => ty
                                        }
                                    } else {
                                        inputs[i].ty.clone()
                                    };

                                    quote_block!(cx, {
                                        cast!(let $id: $ty = args.get($i).expect($s));
                                    }).unwrap().stmts
                                },
                                _ => panic!("#[lia_impl_glue] only supports methods with no pattern matching in the arguments")
                            };
                            binds.push(bind);
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
                            let new_decl = decl.clone().unwrap();
                            let mut new_item = impl_item.clone();
                            new_item.ident = prefix_ident(&new_item.ident, "_lia_");
                            new_item.node =
                                ImplItemKind::Method(MethodSig {
                                    unsafety: unsafety,
                                    constness: constness,
                                    abi: abi,
                                    decl: P(new_decl),
                                    generics: generics,
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
                cx.span_err(sp, "#[lia_impl_glue] must annotate an impl block");
                item.clone()
            }
        },
        _ => {
            cx.span_err(sp, "#[lia_impl_glue] must annotate an item");
            item
        }
    }
}
