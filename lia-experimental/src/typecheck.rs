//! Hindley-Milner type inference.
/// Largely copied from here: <insert link>

use rabbot::var::Var;
use mark::DUMMY;
use ast::{term, typ, TypPrimitive as TyPrim};
use ast::term::{Term, View as TermV};
use ast::typ::{Typ, View as TypV};
use pprint::typ_to_string;
use std::collections::HashMap;

type Constraint = (Typ, Typ);
type TypContext = HashMap<Var, Typ>; // term variables -> types
type Solution   = HashMap<Var, Typ>; // type variables -> types

macro_rules! err {
    ($($x:expr),*) => {
        return Err(format!($($x),*))
    }
}

fn fresh() -> Typ {
    typ::into_view(typ::var(typ::Meta {
        val: Var::new(),
        mark: Default::default()
    }))
}

fn apply_sol(sol: &Solution, ty: Typ) -> Typ {
    sol.iter().fold(ty, |ty, (var, sub)| {
        typ::subst(sub.clone(), var.clone(), ty)
    })
}

fn apply_sol_ctx(sol: &Solution, ctx: TypContext) -> TypContext {
    ctx.into_iter()
        .map(|(var, ty)| { (var, apply_sol(sol, ty)) })
        .collect::<TypContext>()
}

fn combine_sol(sol1: Solution, sol2: Solution) -> Solution {
    let sol1_it = sol1.clone().into_iter()
        .filter(|&(ref var, _)| !sol2.contains_key(&var));
    let mut sol2 = sol2.clone().into_iter()
        .map(|(var, ty)| (var, apply_sol(&sol1, ty)))
        .collect::<TypContext>();
    sol2.extend(sol1_it);
    sol2
}

fn add_sol(v: Var, ty: Typ, mut sol: Solution) -> Solution {
    let ty = apply_sol(&sol, ty);
    sol.insert(v, ty);
    sol
}

fn subst_constrs(ty: Typ, var: Var, cs: Vec<Constraint>) -> Vec<Constraint> {
    cs.into_iter().map(|(l, r)| {
        (typ::subst(ty.clone(), var.clone(), l),
         typ::subst(ty.clone(), var.clone(), r))
    }).collect::<Vec<Constraint>>()
}

fn generalize_monotype(ctx: TypContext, ty: Typ) -> Typ {
    let free = typ::free_vars(ty.clone());
    free.into_iter().fold(ty, |ty, var| {
        if !ctx.contains_key(&var) {
            typ::into(typ::Meta {
                val: TypV::ForAll((var, ty.clone())),
                mark: typ::out(ty).mark
            })
        } else {
            ty
        }
    })
}

// a <: b
fn is_subtype(a: Typ, b: Typ) -> bool {
    if typ::aequiv(a.clone(), b.clone()) {
        return true;
    }

    match (typ::out(a).val, typ::out(b).val) {
        (_, TypV::Var(_)) => true,
        (TypV::Arrow((l1, r1)), TypV::Arrow((l2, r2))) => {
            // contravariant          covariant
            is_subtype(l1, l2) && is_subtype(r2, r1)
        },
        (TypV::ForAll((_, t1)), TypV::ForAll((_, t2))) => {
            is_subtype(t1, t2)
        },
        _ => false
    }
}

fn unify(mut constraints: Vec<Constraint>) -> Solution {
    match constraints.pop() {
        Some((l, r)) => {
            let (lnode, rnode) = (typ::out(l.clone()), typ::out(r.clone()));
            match (lnode.val, rnode.val) {
                (TypV::Primitive(ref t1), TypV::Primitive(ref t2)) if t1 == t2 =>
                    unify(constraints),
                (TypV::Var(i), TypV::Var(j)) => {
                    let (i, j) = (typ::extract_var(i), typ::extract_var(j));
                    if i == j {
                        unify(constraints)
                    } else {
                        add_sol(
                            i.clone(), r.clone(),
                            unify(subst_constrs(r.clone(), i, constraints)))
                    }
                },
                (TypV::Var(i), ty) | (ty, TypV::Var(i)) => {
                    let i = typ::extract_var(i);
                    let ty = typ::into(typ::Meta {
                        val: ty.clone(), ..Default::default()
                    });
                    // TODO: need to check occurs in
                    add_sol(
                        i.clone(),
                        ty.clone(),
                        unify(subst_constrs(ty, i, constraints)))
                },
                (TypV::Arrow((l1, r1)), TypV::Arrow((l2, r2))) => {
                    constraints.push((l1, l2));
                    constraints.push((r1, r2));
                    unify(constraints)
                },
                _ => panic!("Unification error: {} != {}", typ_to_string(l), typ_to_string(r))
            }
        },
        None => HashMap::new()
    }
}

fn constrain(mut ctx: TypContext, t: Term, base_sol: &Solution)
             -> Result<(Typ, Solution), String>
{
    //println!("{:?}", t);
    let node = term::out(t);
    let (typ, sol) = match node.val.clone() {
        TermV::Number(_) => (typ::into_view(TypV::Primitive(TyPrim::Int32)), HashMap::new()),
        TermV::String(_) => (typ::into_view(TypV::Primitive(TyPrim::String)), HashMap::new()),
        TermV::Var(var) => {
            let var = term::extract_var(var);
            match ctx.get(&var) {
                Some(ty) => (match typ::out(ty.clone()).val {
                    TypV::ForAll((_, ty)) => ty,
                    _ => ty.clone()
                }, HashMap::new()),
                None => panic!("Unbound var {:?}", var)
            }
        },
        TermV::Lam((bind, body)) => {
            let arg_var = Var::new();
            let arg_ty = typ::into(typ::Meta {
                val: typ::var(typ::Meta {
                    val: arg_var.clone(),
                    mark: DUMMY.clone()
                }),
                mark: DUMMY.clone()
            });
            ctx.insert(bind.clone(), arg_ty.clone());
            let (ret_ty, sol) = constrain(ctx.clone(), body, base_sol)?;
            let arg_ty = apply_sol(&sol, arg_ty.clone());
            (typ::into_view(TypV::Arrow((arg_ty, ret_ty))), sol)
        },
        TermV::App((l, r)) => {
            let (domain_ty, range_ty) = (fresh(), fresh());
            let (fun_ty, sol1) = constrain(ctx.clone(), l, base_sol)?;
            let (arg_ty, sol2) = constrain(apply_sol_ctx(&sol1, ctx.clone()), r, base_sol)?;
            let sol = combine_sol(sol1, sol2);
            let sol = combine_sol(sol.clone(), unify(vec![
                (apply_sol(&sol, fun_ty),
                 apply_sol(&sol,
                           typ::into_view(TypV::Arrow((domain_ty.clone(), range_ty.clone()))))),
                (apply_sol(&sol, arg_ty),
                 apply_sol(&sol, domain_ty))]));
            (range_ty, sol)
        },
        TermV::Let((t, (var, body))) => {
            let (t_ty, sol1) = constrain(ctx.clone(), t, base_sol)?;
            let mut ctx = apply_sol_ctx(&sol1, ctx.clone());
            let t_ty = generalize_monotype(ctx.clone(), apply_sol(&sol1, t_ty));
            ctx.insert(var, t_ty.clone());
            let (r_ty, sol2) = constrain(ctx, body, base_sol)?;
            (r_ty, combine_sol(sol1, sol2))
        },
        TermV::TLet((t, (var, body))) => {
            let mut sol = base_sol.clone();
            let t = apply_sol(&sol, t);
            sol.insert(var, t);
            let (r_ty, rsol) = constrain(ctx.clone(), body, &sol)?;
            (r_ty, combine_sol(sol, rsol))
        },
        TermV::Plus((l, r)) => {
            let (l_ty, l_sol) = constrain(ctx.clone(), l, base_sol)?;
            let (r_ty, r_sol) = constrain(ctx.clone(), r, base_sol)?;
            let num_ty = typ::into_view(TypV::Primitive(TyPrim::Int32));
            let sol = combine_sol(l_sol, r_sol);
            let sol = combine_sol(sol.clone(), unify(vec![
                (apply_sol(&sol, l_ty), num_ty.clone()),
                (apply_sol(&sol, r_ty), num_ty.clone())]));
            (num_ty, sol)
        },
        TermV::Quote(_) => {
            let (a, b) = (fresh(), fresh());
            let ty = typ::into_view(TypV::Arrow((a, b)));
            (ty, HashMap::new())
        },
        TermV::Annot((t, annotated)) => {
            let (inferred, sol) = constrain(ctx.clone(), t, base_sol)?;
            if !is_subtype(
                apply_sol(&sol, annotated.clone()),
                apply_sol(&sol, inferred.clone())) {
                return Err(format!("Annotation error: {} != {}",
                                   typ_to_string(annotated),
                                   typ_to_string(inferred)));
            }
            (annotated, sol)
        },
        TermV::Product(fields) => {
            let mut expected = None;
            for (ty_name, ty) in base_sol.iter() {
                if let TypV::Product(ty_fields) = typ::out(ty.clone()).val {
                    expected = Some((ty.clone(), ty_name.clone(), ty_fields));
                }
            }

            match expected {
                Some((ty, ty_name, ty_fields)) => {
                    let mut sol = base_sol.clone();
                    for (key, val) in fields.into_iter() {
                        let mut found = false;
                        for &(ref ty_key, ref ty_val) in ty_fields.iter() {
                            if key == *ty_key {
                                found = true;
                                let (ty_term_val, new_sol) =
                                    constrain(ctx.clone(), val, &sol)?;
                                sol = combine_sol(sol, new_sol);

                                if !typ::aequiv(ty_term_val.clone(), ty_val.clone()) {
                                    return Err(format!(
                                        "{:?}: {} != {}",
                                        key,
                                        typ_to_string(ty_val.clone()),
                                        typ_to_string(ty_term_val)));
                                }

                                break;
                            }
                        }

                        if !found {
                            err!("Unexpected field {:?}", key);
                        }
                    }

                    (ty, sol)
                }
                None => err!("No matching product type")
            }
        },
        TermV::Dot((prod, key)) => {
            let (prod_ty, sol) = constrain(ctx.clone(), prod, base_sol)?;
            match typ::out(prod_ty).val {
                TypV::Product(fields) => {
                    match fields.iter().find(|&&(ref ty_key, _)| key == *ty_key) {
                        Some(&(_, ref ty_val)) => (ty_val.clone(), sol),
                        None => err!("Key `{}` does not exist", key)
                    }
                },
                _ => err!("Key access `{}` on non-product type", key)
            }
        },
        TermV::Var_(_) | TermV::Dummy => unreachable!(),
    };

    let sol = combine_sol(base_sol.clone(), sol);

    //let typ = resolve_alias(&sol, typ);
    //println!("{:?} --> {:?}", node.val, typ);

    Ok((typ, sol))
}

pub fn infer(t: Term) -> Result<Typ, String> {
    let mut base_sol = HashMap::new();
    base_sol.insert(
        Var::from_string("i32".to_string()),
        typ::into_view(TypV::Primitive(TyPrim::Int32)));

    let (ty, sol) = constrain(HashMap::new(), t, &base_sol)?;
    Ok(generalize_monotype(HashMap::new(), apply_sol(&sol, ty)))
}
