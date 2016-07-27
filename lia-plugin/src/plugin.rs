use syntax::codemap::Span;
use syntax::parse::token::{Token, str_to_ident};
use syntax::parse::token;
use syntax::ast::*;
use syntax::tokenstream::TokenTree;
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

fn tt_flatten(tt: &TokenTree) -> Vec<LiaToken> {
    match tt {
        &TokenTree::Token(_, ref t) => vec![LiaToken::from_rust_token(t.clone())],
        &TokenTree::Delimited(_, ref delim) => {
            let ref tokens = delim.tts;
            let mut quoted = Vec::new();
            let rs_id = str_to_ident("rust");
            let mut in_quote = false;
            let mut quote_start = 0;
            let mut i = 0;

            quoted.push(LiaToken::from_rust_token(Token::OpenDelim(delim.delim)));
            if tokens.len() > 0 {
                while i < tokens.len() - 1 {
                    let is_rust = |j| {
                        if let &TokenTree::Token(_, Token::Ident(rs_id2)) = &tokens[j] {
                        rs_id2.name.as_str() == rs_id.name.as_str()
                        } else {
                            false
                        }
                    };
                    match (is_rust(i), is_rust(i+1), &tokens[i], &tokens[i+1]) {
                        (true, _, _, &TokenTree::Token(_, Token::Pound)) => {
                            in_quote = true;
                            quote_start = i + 2;
                            i += 1;
                        },
                        (_, true, &TokenTree::Token(_, Token::Pound), _) => {
                            in_quote = false;
                            quoted.push(LiaToken::Quote(tokens[quote_start..i].to_vec()));
                            i += 1;
                        },
                        _ => {
                            if !in_quote {
                            quoted.append(&mut tt_flatten(&tokens[i]));
                            }
                        }
                    };
                    i += 1;
                }

                quoted.append(&mut tt_flatten(&tokens[i]));
            }
            quoted.push(LiaToken::from_rust_token(Token::CloseDelim(delim.delim)));

            quoted
        },
        _ => panic!("TokenTree has Sequence??: {:?}", tt)
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
        .collect();


    println!("tokens: {:?}", tokens);

    let ast =
        lia::grammar::parse_funs(tokens)
        .unwrap_or_else(|err| panic!("Parse error {:?}", err));

    let ast = lia::elaborate::elaborate(ast);

    // All instances of the identifier "this" in the codegen'd AST must
    // have the same token or the compiler will complain. I'm not sure how
    // else to ensure this besides folding over the AST as below.
    let this = token::str_to_ident("this");
    let mut renamer = Renamer { id: this, from: "this".to_string() };

    let fs: Vec<P<Item>> =
        ast.into_iter()
        .map(|fun| renamer.fold_item(gen_fn(cx, fun)).get(0).clone())
        .collect();

    MacEager::items(Svec::many(fs))
}

#[allow(unused_variables)]
pub fn expand_alloc(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree]) ->
    Box<MacResult + 'static>
{
    use syntax::parse::tts_to_parser;

    let mut parser = tts_to_parser(&cx.parse_sess, args.to_vec(), Vec::new());
    let ty = match parser.parse_ty() {
        Ok(ty) => ty,
        Err(_) => panic!("Invalid type"),
    };

    let expr = match parser.parse_expr() {
        Ok(expr) => expr,
        Err(_) => panic!("Invalid expr"),
    };

    MacEager::expr(match ty.node.clone() {
        TyKind::Path(_, path) => {
            let path_s = format!("{}", path);
            if path_s == "i32" || path_s == "LiaNumber" {
                quote_expr!(cx, alloc_number($expr))
            } else if path_s == "String" || path_s == "LiaString" {
                quote_expr!(cx, alloc_string($expr))
            } else if path_s == "bool" || path_s == "LiaBool" {
                quote_expr!(cx, alloc_bool($expr))
            } else if path_s == "LiaObject" {
                quote_expr!(cx, alloc_object($expr))
            } else if path_s == "LiaClosure" {
                quote_expr!(cx, alloc_closure($expr))
            } else {
                quote_expr!(cx, alloc_other($expr))
            }
        },
        TyKind::Tup(vec) => {
            assert!(vec.len() == 0);
            quote_expr!(cx, alloc_null($expr))
        },
        _ => panic!("Ty must be a path")
    })
}

#[allow(unused_variables)]
pub fn expand_borrow_type(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree]) ->
    Box<MacResult + 'static>
{
    use syntax::parse::tts_to_parser;

    let mut parser = tts_to_parser(&cx.parse_sess, args.to_vec(), Vec::new());

    let id = match parser.parse_ident() {
        Ok(id) => id,
        Err(_) => panic!("Invalid ident"),
    };

    let ty = match parser.parse_ty() {
        Ok(ty) => ty,
        Err(_) => panic!("Invalid ty"),
    };

    let (ty, is_ref) = match &ty.node {
        &TyKind::Rptr(_, ref mutty) => {
            let sub_ty = mutty.ty.clone();
            (sub_ty, true)
        },
        &TyKind::Path(_, ref path) => {
            (ty.clone(), false)
        },
        _ => panic!("Type isn't yet supported for casting with Lia"),
    };

    let is_mut_borrow = match parser.parse_expr() {
        Ok(expr) => if let &ExprKind::Lit(ref lit) = &expr.node {
            if let &LitKind::Bool(ref b) = &lit.node {
                *b
            } else {
                panic!("Bad bool")
            }
        } else {
            panic!("Bad bool")
        },
        Err(_) => {
            panic!("Bad bool")
        }
    };

    let make_caster = |path: Path, path_s: String| -> P<Expr> {
        let err_str = format!("Invalid cast: expected {}, found {{:?}}", path_s);
        if is_mut_borrow {
            quote_expr!(cx, {
                match *$id {
                    $path(ref mut x) => x,
                    ref other => panic!($err_str, other)
                }
            })
        } else {
            quote_expr!(cx, {
                match *$id {
                    $path(ref x) => x,
                    ref other => panic!($err_str, other)
                }
            })
        }
    };

    let ty_str = format!("Invalid cast: expected {:?}, found {{:?}}", ty);

    let default =
        make_caster(quote_path!(cx, LiaValue::Unknown), format!("{:?}", ty));
    let default =
        quote_expr!(cx, { $default.downcast_ref::<$ty>().expect($ty_str) });

    let expr = match ty.node.clone() {
        TyKind::Path(_, path) => {
            let path_s = format!("{}", path);
            if path_s == "i32" || path_s == "LiaNumber" {
                make_caster(quote_path!(cx, LiaValue::Number), path_s)
            } else if path_s == "String" || path_s == "LiaString" {
                make_caster(quote_path!(cx, LiaValue::String), path_s)
            } else if path_s == "bool" || path_s == "LiaBool" {
                make_caster(quote_path!(cx, LiaValue::Bool), path_s)
            } else if path_s == "LiaObject" {
                make_caster(quote_path!(cx, LiaValue::Object), path_s)
            } else if path_s == "LiaClosure" {
                make_caster(quote_path!(cx, LiaValue::Closure), path_s)
            } else {
                default
            }
        },
        _ => default
    };

    let expr = if !is_ref { quote_expr!(cx, { ($expr).clone() }) }
    else { expr };

    MacEager::expr(expr)
}



struct Renamer {
    from: String,
    id: Ident
}

impl Folder for Renamer {
    fn fold_ident(&mut self, id: Ident) -> Ident {
        if id.name.as_str() == self.from.as_str() {
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
                                        let mut renamer = Renamer {id: new_id, from: "self".to_string()};
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

                                    let i = i + 1; // shift right for first "this" arg
                                    quote_block!(cx, {
                                        cast!(let mut $id: $ty = args.get($i).expect($s));
                                    }).unwrap().stmts
                                },
                                _ => panic!("#[lia_impl_glue] only supports methods with no pattern matching in the arguments")
                            };
                            binds.push(bind);
                        }

                        let (ret_ty, new_body) = match sig.decl.output {
                            FunctionRetTy::Ty(ref ty) => (ty.clone(), new_body),
                            _ => (quote_ty!(cx, ()), quote_block!(cx, {
                                {$new_body};
                                return ();
                            }))
                        };

                        let binds: Vec<Stmt> = binds.into_iter().flat_map(|e| e).collect();
                        // TODO: attrs for ignoring warnings on fn?
                        let fun =
                            quote_item!(
                                cx,
                                fn _ignore_this_name (args: Vec<LiaPtr>) -> LiaPtr {
                                    $binds;
                                    alloc!($ret_ty, (move ||$new_body)())
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
