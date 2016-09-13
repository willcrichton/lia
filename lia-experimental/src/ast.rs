rabbot! {
    sort Typ {
        Number,
        String,
        Arrow((Typ, Typ)),
        ForAll(Binding<Typ> . Typ),
        Exists(Binding<Typ> . Typ)
    }

    sort Term {
        Number(i32),
        String(String),
        Quote(Vec<Term>),
        Plus((Term, Term)),
        Lam(Binding<Term> . Term),
        Let((Term, Binding<Term> . Term)),
        App((Term, Term))
    }
}
