use rabbot::var::Var;
use mark::DUMMY;
use ast::{term, typ};
use ast::term::{Term, View as TermV};
use ast::typ::{Typ, View as TypV};
use std::collections::HashMap;

macro_rules! ok {
    ($e:expr) => { Ok(typ::into($e)) }
}

// pub fn typecheck(t: Term) -> Result<Typ, String> {
//     match term::out(t) {
//         TermV::Number(_) => ok!(TypV::Number),
//         TermV::String(_) => ok!(TypV::String),
//         TermV::Quote(_) => panic!(),
//         TermV::Plus((box l, box r)) => {
//             if let (TypV::Number, TypV::Number) =
//                 (typ::out(typecheck(l)?), typ::out(typecheck(r)?))
//             {
//                 ok!(TypV::Number)
//             } else {
//                 Err("Plus")
//             }
//         },
//     }
// }

type Constraint = (Typ, Typ);
type TypContext = HashMap<Var, Typ>;
type Solution = HashMap<Var, Typ>;

macro_rules! ty {
    ($p:ident { $e:expr }) => { typ::into(typ::Meta {
        val: TypV::$p($e),
        mark: DUMMY.clone()
    }) };
    ($p:ident) => { typ::into(typ::Meta {
        val: TypV::$p,
        mark: DUMMY.clone()
    }) };
}

fn fresh() -> Typ {
    typ::into(typ::Meta {
        val: typ::var(typ::Meta {
            val: Var::new(),
            mark: DUMMY.clone()
        }),
        mark: DUMMY.clone()
    })
}

fn apply_sol(sol: Solution, ty: Typ) -> Typ {
    sol.into_iter().fold(ty, |ty, (var, sub)| { typ::subst(sub, var, ty) })
}

fn apply_sol_ctx(sol: Solution, ctx: TypContext) -> TypContext {
    ctx.into_iter()
        .map(|(var, ty)| { (var, apply_sol(sol.clone(), ty)) })
        .collect::<TypContext>()
}

fn combine_sol(sol1: Solution, sol2: Solution) -> Solution {
    let sol1_it = sol1.clone().into_iter()
        .filter(|&(ref var, _)| !sol2.contains_key(&var));
    let mut sol2 = sol2.clone().into_iter()
        .map(|(var, ty)| (var, apply_sol(sol1.clone(), ty)))
        .collect::<TypContext>();
    sol2.extend(sol1_it);
    sol2
}

fn add_sol(v: Var, ty: Typ, mut sol: Solution) -> Solution {
    let sol_clone = sol.clone();
    sol.insert(v, apply_sol(sol_clone, ty));
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
        typ::into(typ::Meta {
            val: TypV::ForAll((var, ty.clone())),
            mark: typ::out(ty).mark
        })
    })
}

fn unify(mut constraints: Vec<Constraint>) -> Solution {
    match constraints.pop() {
        Some((l, r)) => {
            let (lnode, rnode) = (typ::out(l.clone()), typ::out(r.clone()));
            match (lnode.val, rnode.val) {
                (TypV::Number, TypV::Number) => unify(constraints),
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
                        val: ty.clone(),
                        mark: DUMMY.clone()
                    });
                    // need to check occurs in
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
                c => panic!("Unification error: {:?}", c)
            }
        },
        None => HashMap::new()
    }
}

fn constrain(mut ctx: TypContext, t: Term) -> (Typ, Solution) {
    match term::out(t).val {
        TermV::Number(_) => (ty!(Number), HashMap::new()),
        TermV::String(_) => (ty!(String), HashMap::new()),
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
            let (ret_ty, sol) = constrain(ctx, body);
            let arg_ty = apply_sol(sol.clone(), arg_ty.clone());
            (ty!(Arrow{(arg_ty, ret_ty)}), sol)
        },
        TermV::App((l, r)) => {
            let (domain_ty, range_ty) = (fresh(), fresh());
            let (fun_ty, sol1) = constrain(ctx.clone(), l);
            let (arg_ty, sol2) = constrain(apply_sol_ctx(sol1.clone(), ctx), r);
            let sol = combine_sol(sol1, sol2);
            let sol = combine_sol(sol.clone(), unify(vec![
                (apply_sol(sol.clone(), fun_ty),
                 apply_sol(sol.clone(),
                           ty!(Arrow{(domain_ty.clone(), range_ty.clone())}))),
                (apply_sol(sol.clone(), arg_ty),
                 apply_sol(sol.clone(), domain_ty))]));
            (range_ty, sol)
        },
        TermV::Let((t, (var, body))) => {
            let (t_ty, sol1) = constrain(ctx.clone(), t);
            let mut ctx = apply_sol_ctx(sol1.clone(), ctx);
            let t_ty = generalize_monotype(ctx.clone(), apply_sol(sol1.clone(), t_ty));
            ctx.insert(var, t_ty.clone());
            let (r_ty, sol2) = constrain(ctx, body);
            (r_ty, combine_sol(sol1, sol2))
        }
        _ => panic!("Unimplemented")
    }
}

pub fn infer(t: Term) -> Typ {
    let (ty, sol) = constrain(HashMap::new(), t);
    generalize_monotype(HashMap::new(), apply_sol(sol, ty))
}
