use std::convert::TryFrom;

#[derive(Clone, Eq, Debug, PartialEq)]
pub enum TypPrimitive {
    Int32,
    String
}

impl TryFrom<String> for TypPrimitive {
    type Err = ();
    fn try_from(t: String) -> Result<TypPrimitive, ()> {
        use ast::TypPrimitive::*;
        match t.as_str() {
            "i32" => Ok(Int32),
            "String" => Ok(String),
            _ => Err(())
        }
    }
}

rabbot! {
    use mark::Mark;
    use ast::TypPrimitive;

    enum Typ {mark: Mark} {
        Hole,
        Primitive(TypPrimitive),
        Arrow((Typ, Typ)),
        Product(Vec<(String, Typ)>),
        ForAll(Binding<Typ> . Typ),
        Exists(Binding<Typ> . Typ)
    }

    enum Term {mark: Mark, typ: Typ} {
        Dummy,
        Number(i32),
        String(String),
        Quote(Vec<Term>),
        Plus((Term, Term)),
        Lam(Binding<Term> . Term),
        Let((Term, Binding<Term> . Term)),
        TLet((Typ, Binding<Typ>. Term)),
        App((Term, Term))
    }
}
