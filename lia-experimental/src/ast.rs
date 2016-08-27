rabbot! {
    sort Term {
        Number(i32),
        Plus((Term, Term)),
        Lam(Binding<Term> . Term),
        Let((Term, Binding<Term> . Term)),
        App((Term, Term))
    }
}
