use ast::typ;
use ast::typ::{Typ, View as TypV};

pub fn typ_to_string(typ: Typ) -> String {
    match typ::out(typ).val {
        TypV::Hole => "hole".to_string(),
        TypV::Primitive(prim) => format!("{:?}", prim),
        TypV::Arrow((t1, t2)) =>
            format!("{} -> {}", typ_to_string(t1), typ_to_string(t2)),
        TypV::Product(typs) =>
            format!("{{{}}}",
                    typs.into_iter()
                    .map(|(s, ty)| format!("{}: {}", s, typ_to_string(ty)))
                    .collect::<Vec<String>>()
                    .join(", ")),
        TypV::ForAll((var, body)) =>
            format!("∀{}. ({})", var, typ_to_string(body)),
        TypV::Exists((var, body)) =>
            format!("∃{}. ({})", var, typ_to_string(body)),
        TypV::Var(var) => {
            let var = typ::extract_var(var);
            format!("{}", var)
        }
        TypV::Var_(_) => unreachable!()
    }
}
