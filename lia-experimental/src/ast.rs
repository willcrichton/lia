rabbot! {
    use mark::Mark;

    enum Typ {mark: Mark} {
        Hole,
        Number,
        String,
        Arrow((Typ, Typ)),
        ForAll(Binding<Typ> . Typ),
        Exists(Binding<Typ> . Typ)
    }

    enum Term {mark: Mark, typ: Typ} {
        Number(i32),
        String(String),
        Quote(Vec<Term>),
        Plus((Term, Term)),
        Lam(Binding<Term> . Term),
        Let((Term, Binding<Term> . Term)),
        App((Term, Term))
    }
}
