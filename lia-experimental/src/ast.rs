rabbot! {
    sort Typ {
        Number,
        Arrow((Typ, Typ)),
        ForAll(Binding<Typ> . Typ),
        Exists(Binding<Typ> . Typ)
    }

    sort Term {
        Number(i32),
        Quote(String),
        Plus((Term, Term)),
        Lam((Binding<Term>, Typ) . Term),
        Let((Term, Binding<Term> . Term)),
        App((Term, Term)),
        Foreign
    }
}
