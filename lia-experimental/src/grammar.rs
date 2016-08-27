use std::str::FromStr;
use super::ast::term::{View, Term, into};
use rabbot::var::Var;
extern crate lalrpop_util as __lalrpop_util;

mod __parse__Block {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use super::super::ast::term::{View, Term, into};
    use rabbot::var::Var;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_2b_22(&'input str),
        Term_22_3b_22(&'input str),
        Term_22_3d_22(&'input str),
        Term_22_3d_3e_22(&'input str),
        Term_22fn_22(&'input str),
        Term_22let_22(&'input str),
        Term_22_7b_22(&'input str),
        Term_22_7d_22(&'input str),
        Termr_23_22_5b1_2d9_5d_5b0_2d9_5d_2a_22_23(&'input str),
        Termr_23_22_5ba_2dzA_2dZ_5d_5ba_2dzA_2dZ0_2d9_5d_2a_22_23(&'input str),
        NtAtom(Term),
        NtBlock(Term),
        NtExpr(Term),
        NtId(Var),
        Nt____Block(Term),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        6, // on "(", goto 5
        0, // on ")", error
        0, // on "+", error
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        7, // on "fn", goto 6
        8, // on "let", goto 7
        9, // on "{", goto 8
        0, // on "}", error
        10, // on r#"[1-9][0-9]*"#, goto 9
        11, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, goto 10
        // State 1
        -10, // on "(", reduce `Expr = Atom => ActionFn(5);`
        0, // on ")", error
        -10, // on "+", reduce `Expr = Atom => ActionFn(5);`
        -10, // on ";", reduce `Expr = Atom => ActionFn(5);`
        0, // on "=", error
        0, // on "=>", error
        -10, // on "fn", reduce `Expr = Atom => ActionFn(5);`
        0, // on "let", error
        -10, // on "{", reduce `Expr = Atom => ActionFn(5);`
        0, // on "}", error
        -10, // on r#"[1-9][0-9]*"#, reduce `Expr = Atom => ActionFn(5);`
        -10, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, reduce `Expr = Atom => ActionFn(5);`
        // State 2
        0, // on "(", error
        0, // on ")", error
        0, // on "+", error
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        0, // on "fn", error
        0, // on "let", error
        0, // on "{", error
        0, // on "}", error
        0, // on r#"[1-9][0-9]*"#, error
        0, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, error
        // State 3
        6, // on "(", goto 5
        0, // on ")", error
        13, // on "+", goto 12
        14, // on ";", goto 13
        0, // on "=", error
        0, // on "=>", error
        7, // on "fn", goto 6
        0, // on "let", error
        9, // on "{", goto 8
        0, // on "}", error
        10, // on r#"[1-9][0-9]*"#, goto 9
        11, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, goto 10
        // State 4
        -2, // on "(", reduce `Atom = Id => ActionFn(7);`
        0, // on ")", error
        -2, // on "+", reduce `Atom = Id => ActionFn(7);`
        -2, // on ";", reduce `Atom = Id => ActionFn(7);`
        0, // on "=", error
        0, // on "=>", error
        -2, // on "fn", reduce `Atom = Id => ActionFn(7);`
        0, // on "let", error
        -2, // on "{", reduce `Atom = Id => ActionFn(7);`
        0, // on "}", error
        -2, // on r#"[1-9][0-9]*"#, reduce `Atom = Id => ActionFn(7);`
        -2, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, reduce `Atom = Id => ActionFn(7);`
        // State 5
        18, // on "(", goto 17
        0, // on ")", error
        0, // on "+", error
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        19, // on "fn", goto 18
        0, // on "let", error
        20, // on "{", goto 19
        0, // on "}", error
        21, // on r#"[1-9][0-9]*"#, goto 20
        22, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, goto 21
        // State 6
        0, // on "(", error
        0, // on ")", error
        0, // on "+", error
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        0, // on "fn", error
        0, // on "let", error
        0, // on "{", error
        0, // on "}", error
        0, // on r#"[1-9][0-9]*"#, error
        24, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, goto 23
        // State 7
        0, // on "(", error
        0, // on ")", error
        0, // on "+", error
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        0, // on "fn", error
        0, // on "let", error
        0, // on "{", error
        0, // on "}", error
        0, // on r#"[1-9][0-9]*"#, error
        26, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, goto 25
        // State 8
        6, // on "(", goto 5
        0, // on ")", error
        0, // on "+", error
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        7, // on "fn", goto 6
        29, // on "let", goto 28
        9, // on "{", goto 8
        0, // on "}", error
        10, // on r#"[1-9][0-9]*"#, goto 9
        11, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, goto 10
        // State 9
        -1, // on "(", reduce `Atom = r#"[1-9][0-9]*"# => ActionFn(6);`
        0, // on ")", error
        -1, // on "+", reduce `Atom = r#"[1-9][0-9]*"# => ActionFn(6);`
        -1, // on ";", reduce `Atom = r#"[1-9][0-9]*"# => ActionFn(6);`
        0, // on "=", error
        0, // on "=>", error
        -1, // on "fn", reduce `Atom = r#"[1-9][0-9]*"# => ActionFn(6);`
        0, // on "let", error
        -1, // on "{", reduce `Atom = r#"[1-9][0-9]*"# => ActionFn(6);`
        0, // on "}", error
        -1, // on r#"[1-9][0-9]*"#, reduce `Atom = r#"[1-9][0-9]*"# => ActionFn(6);`
        -1, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, reduce `Atom = r#"[1-9][0-9]*"# => ActionFn(6);`
        // State 10
        -11, // on "(", reduce `Id = r#"[a-zA-Z][a-zA-Z0-9]*"# => ActionFn(11);`
        0, // on ")", error
        -11, // on "+", reduce `Id = r#"[a-zA-Z][a-zA-Z0-9]*"# => ActionFn(11);`
        -11, // on ";", reduce `Id = r#"[a-zA-Z][a-zA-Z0-9]*"# => ActionFn(11);`
        0, // on "=", error
        0, // on "=>", error
        -11, // on "fn", reduce `Id = r#"[a-zA-Z][a-zA-Z0-9]*"# => ActionFn(11);`
        0, // on "let", error
        -11, // on "{", reduce `Id = r#"[a-zA-Z][a-zA-Z0-9]*"# => ActionFn(11);`
        0, // on "}", error
        -11, // on r#"[1-9][0-9]*"#, reduce `Id = r#"[a-zA-Z][a-zA-Z0-9]*"# => ActionFn(11);`
        -11, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, reduce `Id = r#"[a-zA-Z][a-zA-Z0-9]*"# => ActionFn(11);`
        // State 11
        -9, // on "(", reduce `Expr = Expr, Atom => ActionFn(4);`
        0, // on ")", error
        -9, // on "+", reduce `Expr = Expr, Atom => ActionFn(4);`
        -9, // on ";", reduce `Expr = Expr, Atom => ActionFn(4);`
        0, // on "=", error
        0, // on "=>", error
        -9, // on "fn", reduce `Expr = Expr, Atom => ActionFn(4);`
        0, // on "let", error
        -9, // on "{", reduce `Expr = Expr, Atom => ActionFn(4);`
        0, // on "}", error
        -9, // on r#"[1-9][0-9]*"#, reduce `Expr = Expr, Atom => ActionFn(4);`
        -9, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, reduce `Expr = Expr, Atom => ActionFn(4);`
        // State 12
        6, // on "(", goto 5
        0, // on ")", error
        0, // on "+", error
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        7, // on "fn", goto 6
        0, // on "let", error
        9, // on "{", goto 8
        0, // on "}", error
        10, // on r#"[1-9][0-9]*"#, goto 9
        11, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, goto 10
        // State 13
        34, // on "(", goto 33
        0, // on ")", error
        0, // on "+", error
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        35, // on "fn", goto 34
        0, // on "let", error
        36, // on "{", goto 35
        0, // on "}", error
        37, // on r#"[1-9][0-9]*"#, goto 36
        38, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, goto 37
        // State 14
        -10, // on "(", reduce `Expr = Atom => ActionFn(5);`
        -10, // on ")", reduce `Expr = Atom => ActionFn(5);`
        -10, // on "+", reduce `Expr = Atom => ActionFn(5);`
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        -10, // on "fn", reduce `Expr = Atom => ActionFn(5);`
        0, // on "let", error
        -10, // on "{", reduce `Expr = Atom => ActionFn(5);`
        0, // on "}", error
        -10, // on r#"[1-9][0-9]*"#, reduce `Expr = Atom => ActionFn(5);`
        -10, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, reduce `Expr = Atom => ActionFn(5);`
        // State 15
        18, // on "(", goto 17
        40, // on ")", goto 39
        41, // on "+", goto 40
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        19, // on "fn", goto 18
        0, // on "let", error
        20, // on "{", goto 19
        0, // on "}", error
        21, // on r#"[1-9][0-9]*"#, goto 20
        22, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, goto 21
        // State 16
        -2, // on "(", reduce `Atom = Id => ActionFn(7);`
        -2, // on ")", reduce `Atom = Id => ActionFn(7);`
        -2, // on "+", reduce `Atom = Id => ActionFn(7);`
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        -2, // on "fn", reduce `Atom = Id => ActionFn(7);`
        0, // on "let", error
        -2, // on "{", reduce `Atom = Id => ActionFn(7);`
        0, // on "}", error
        -2, // on r#"[1-9][0-9]*"#, reduce `Atom = Id => ActionFn(7);`
        -2, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, reduce `Atom = Id => ActionFn(7);`
        // State 17
        18, // on "(", goto 17
        0, // on ")", error
        0, // on "+", error
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        19, // on "fn", goto 18
        0, // on "let", error
        20, // on "{", goto 19
        0, // on "}", error
        21, // on r#"[1-9][0-9]*"#, goto 20
        22, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, goto 21
        // State 18
        0, // on "(", error
        0, // on ")", error
        0, // on "+", error
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        0, // on "fn", error
        0, // on "let", error
        0, // on "{", error
        0, // on "}", error
        0, // on r#"[1-9][0-9]*"#, error
        24, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, goto 23
        // State 19
        6, // on "(", goto 5
        0, // on ")", error
        0, // on "+", error
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        7, // on "fn", goto 6
        29, // on "let", goto 28
        9, // on "{", goto 8
        0, // on "}", error
        10, // on r#"[1-9][0-9]*"#, goto 9
        11, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, goto 10
        // State 20
        -1, // on "(", reduce `Atom = r#"[1-9][0-9]*"# => ActionFn(6);`
        -1, // on ")", reduce `Atom = r#"[1-9][0-9]*"# => ActionFn(6);`
        -1, // on "+", reduce `Atom = r#"[1-9][0-9]*"# => ActionFn(6);`
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        -1, // on "fn", reduce `Atom = r#"[1-9][0-9]*"# => ActionFn(6);`
        0, // on "let", error
        -1, // on "{", reduce `Atom = r#"[1-9][0-9]*"# => ActionFn(6);`
        0, // on "}", error
        -1, // on r#"[1-9][0-9]*"#, reduce `Atom = r#"[1-9][0-9]*"# => ActionFn(6);`
        -1, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, reduce `Atom = r#"[1-9][0-9]*"# => ActionFn(6);`
        // State 21
        -11, // on "(", reduce `Id = r#"[a-zA-Z][a-zA-Z0-9]*"# => ActionFn(11);`
        -11, // on ")", reduce `Id = r#"[a-zA-Z][a-zA-Z0-9]*"# => ActionFn(11);`
        -11, // on "+", reduce `Id = r#"[a-zA-Z][a-zA-Z0-9]*"# => ActionFn(11);`
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        -11, // on "fn", reduce `Id = r#"[a-zA-Z][a-zA-Z0-9]*"# => ActionFn(11);`
        0, // on "let", error
        -11, // on "{", reduce `Id = r#"[a-zA-Z][a-zA-Z0-9]*"# => ActionFn(11);`
        0, // on "}", error
        -11, // on r#"[1-9][0-9]*"#, reduce `Id = r#"[a-zA-Z][a-zA-Z0-9]*"# => ActionFn(11);`
        -11, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, reduce `Id = r#"[a-zA-Z][a-zA-Z0-9]*"# => ActionFn(11);`
        // State 22
        0, // on "(", error
        0, // on ")", error
        0, // on "+", error
        0, // on ";", error
        0, // on "=", error
        45, // on "=>", goto 44
        0, // on "fn", error
        0, // on "let", error
        0, // on "{", error
        0, // on "}", error
        0, // on r#"[1-9][0-9]*"#, error
        0, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, error
        // State 23
        0, // on "(", error
        0, // on ")", error
        0, // on "+", error
        0, // on ";", error
        0, // on "=", error
        -11, // on "=>", reduce `Id = r#"[a-zA-Z][a-zA-Z0-9]*"# => ActionFn(11);`
        0, // on "fn", error
        0, // on "let", error
        0, // on "{", error
        0, // on "}", error
        0, // on r#"[1-9][0-9]*"#, error
        0, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, error
        // State 24
        0, // on "(", error
        0, // on ")", error
        0, // on "+", error
        0, // on ";", error
        46, // on "=", goto 45
        0, // on "=>", error
        0, // on "fn", error
        0, // on "let", error
        0, // on "{", error
        0, // on "}", error
        0, // on r#"[1-9][0-9]*"#, error
        0, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, error
        // State 25
        0, // on "(", error
        0, // on ")", error
        0, // on "+", error
        0, // on ";", error
        -11, // on "=", reduce `Id = r#"[a-zA-Z][a-zA-Z0-9]*"# => ActionFn(11);`
        0, // on "=>", error
        0, // on "fn", error
        0, // on "let", error
        0, // on "{", error
        0, // on "}", error
        0, // on r#"[1-9][0-9]*"#, error
        0, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, error
        // State 26
        0, // on "(", error
        0, // on ")", error
        0, // on "+", error
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        0, // on "fn", error
        0, // on "let", error
        0, // on "{", error
        47, // on "}", goto 46
        0, // on r#"[1-9][0-9]*"#, error
        0, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, error
        // State 27
        6, // on "(", goto 5
        0, // on ")", error
        13, // on "+", goto 12
        48, // on ";", goto 47
        0, // on "=", error
        0, // on "=>", error
        7, // on "fn", goto 6
        0, // on "let", error
        9, // on "{", goto 8
        0, // on "}", error
        10, // on r#"[1-9][0-9]*"#, goto 9
        11, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, goto 10
        // State 28
        0, // on "(", error
        0, // on ")", error
        0, // on "+", error
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        0, // on "fn", error
        0, // on "let", error
        0, // on "{", error
        0, // on "}", error
        0, // on r#"[1-9][0-9]*"#, error
        26, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, goto 25
        // State 29
        -8, // on "(", reduce `Expr = Expr, "+", Atom => ActionFn(3);`
        0, // on ")", error
        -8, // on "+", reduce `Expr = Expr, "+", Atom => ActionFn(3);`
        -8, // on ";", reduce `Expr = Expr, "+", Atom => ActionFn(3);`
        0, // on "=", error
        0, // on "=>", error
        -8, // on "fn", reduce `Expr = Expr, "+", Atom => ActionFn(3);`
        0, // on "let", error
        -8, // on "{", reduce `Expr = Expr, "+", Atom => ActionFn(3);`
        0, // on "}", error
        -8, // on r#"[1-9][0-9]*"#, reduce `Expr = Expr, "+", Atom => ActionFn(3);`
        -8, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, reduce `Expr = Expr, "+", Atom => ActionFn(3);`
        // State 30
        -10, // on "(", reduce `Expr = Atom => ActionFn(5);`
        0, // on ")", error
        -10, // on "+", reduce `Expr = Atom => ActionFn(5);`
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        -10, // on "fn", reduce `Expr = Atom => ActionFn(5);`
        0, // on "let", error
        -10, // on "{", reduce `Expr = Atom => ActionFn(5);`
        0, // on "}", error
        -10, // on r#"[1-9][0-9]*"#, reduce `Expr = Atom => ActionFn(5);`
        -10, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, reduce `Expr = Atom => ActionFn(5);`
        // State 31
        34, // on "(", goto 33
        0, // on ")", error
        51, // on "+", goto 50
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        35, // on "fn", goto 34
        0, // on "let", error
        36, // on "{", goto 35
        0, // on "}", error
        37, // on r#"[1-9][0-9]*"#, goto 36
        38, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, goto 37
        // State 32
        -2, // on "(", reduce `Atom = Id => ActionFn(7);`
        0, // on ")", error
        -2, // on "+", reduce `Atom = Id => ActionFn(7);`
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        -2, // on "fn", reduce `Atom = Id => ActionFn(7);`
        0, // on "let", error
        -2, // on "{", reduce `Atom = Id => ActionFn(7);`
        0, // on "}", error
        -2, // on r#"[1-9][0-9]*"#, reduce `Atom = Id => ActionFn(7);`
        -2, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, reduce `Atom = Id => ActionFn(7);`
        // State 33
        18, // on "(", goto 17
        0, // on ")", error
        0, // on "+", error
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        19, // on "fn", goto 18
        0, // on "let", error
        20, // on "{", goto 19
        0, // on "}", error
        21, // on r#"[1-9][0-9]*"#, goto 20
        22, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, goto 21
        // State 34
        0, // on "(", error
        0, // on ")", error
        0, // on "+", error
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        0, // on "fn", error
        0, // on "let", error
        0, // on "{", error
        0, // on "}", error
        0, // on r#"[1-9][0-9]*"#, error
        24, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, goto 23
        // State 35
        6, // on "(", goto 5
        0, // on ")", error
        0, // on "+", error
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        7, // on "fn", goto 6
        29, // on "let", goto 28
        9, // on "{", goto 8
        0, // on "}", error
        10, // on r#"[1-9][0-9]*"#, goto 9
        11, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, goto 10
        // State 36
        -1, // on "(", reduce `Atom = r#"[1-9][0-9]*"# => ActionFn(6);`
        0, // on ")", error
        -1, // on "+", reduce `Atom = r#"[1-9][0-9]*"# => ActionFn(6);`
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        -1, // on "fn", reduce `Atom = r#"[1-9][0-9]*"# => ActionFn(6);`
        0, // on "let", error
        -1, // on "{", reduce `Atom = r#"[1-9][0-9]*"# => ActionFn(6);`
        0, // on "}", error
        -1, // on r#"[1-9][0-9]*"#, reduce `Atom = r#"[1-9][0-9]*"# => ActionFn(6);`
        -1, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, reduce `Atom = r#"[1-9][0-9]*"# => ActionFn(6);`
        // State 37
        -11, // on "(", reduce `Id = r#"[a-zA-Z][a-zA-Z0-9]*"# => ActionFn(11);`
        0, // on ")", error
        -11, // on "+", reduce `Id = r#"[a-zA-Z][a-zA-Z0-9]*"# => ActionFn(11);`
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        -11, // on "fn", reduce `Id = r#"[a-zA-Z][a-zA-Z0-9]*"# => ActionFn(11);`
        0, // on "let", error
        -11, // on "{", reduce `Id = r#"[a-zA-Z][a-zA-Z0-9]*"# => ActionFn(11);`
        0, // on "}", error
        -11, // on r#"[1-9][0-9]*"#, reduce `Id = r#"[a-zA-Z][a-zA-Z0-9]*"# => ActionFn(11);`
        -11, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, reduce `Id = r#"[a-zA-Z][a-zA-Z0-9]*"# => ActionFn(11);`
        // State 38
        -9, // on "(", reduce `Expr = Expr, Atom => ActionFn(4);`
        -9, // on ")", reduce `Expr = Expr, Atom => ActionFn(4);`
        -9, // on "+", reduce `Expr = Expr, Atom => ActionFn(4);`
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        -9, // on "fn", reduce `Expr = Expr, Atom => ActionFn(4);`
        0, // on "let", error
        -9, // on "{", reduce `Expr = Expr, Atom => ActionFn(4);`
        0, // on "}", error
        -9, // on r#"[1-9][0-9]*"#, reduce `Expr = Expr, Atom => ActionFn(4);`
        -9, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, reduce `Expr = Expr, Atom => ActionFn(4);`
        // State 39
        -3, // on "(", reduce `Atom = "(", Expr, ")" => ActionFn(8);`
        0, // on ")", error
        -3, // on "+", reduce `Atom = "(", Expr, ")" => ActionFn(8);`
        -3, // on ";", reduce `Atom = "(", Expr, ")" => ActionFn(8);`
        0, // on "=", error
        0, // on "=>", error
        -3, // on "fn", reduce `Atom = "(", Expr, ")" => ActionFn(8);`
        0, // on "let", error
        -3, // on "{", reduce `Atom = "(", Expr, ")" => ActionFn(8);`
        0, // on "}", error
        -3, // on r#"[1-9][0-9]*"#, reduce `Atom = "(", Expr, ")" => ActionFn(8);`
        -3, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, reduce `Atom = "(", Expr, ")" => ActionFn(8);`
        // State 40
        18, // on "(", goto 17
        0, // on ")", error
        0, // on "+", error
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        19, // on "fn", goto 18
        0, // on "let", error
        20, // on "{", goto 19
        0, // on "}", error
        21, // on r#"[1-9][0-9]*"#, goto 20
        22, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, goto 21
        // State 41
        18, // on "(", goto 17
        56, // on ")", goto 55
        41, // on "+", goto 40
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        19, // on "fn", goto 18
        0, // on "let", error
        20, // on "{", goto 19
        0, // on "}", error
        21, // on r#"[1-9][0-9]*"#, goto 20
        22, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, goto 21
        // State 42
        0, // on "(", error
        0, // on ")", error
        0, // on "+", error
        0, // on ";", error
        0, // on "=", error
        57, // on "=>", goto 56
        0, // on "fn", error
        0, // on "let", error
        0, // on "{", error
        0, // on "}", error
        0, // on r#"[1-9][0-9]*"#, error
        0, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, error
        // State 43
        0, // on "(", error
        0, // on ")", error
        0, // on "+", error
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        0, // on "fn", error
        0, // on "let", error
        0, // on "{", error
        58, // on "}", goto 57
        0, // on r#"[1-9][0-9]*"#, error
        0, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, error
        // State 44
        0, // on "(", error
        0, // on ")", error
        0, // on "+", error
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        0, // on "fn", error
        0, // on "let", error
        59, // on "{", goto 58
        0, // on "}", error
        0, // on r#"[1-9][0-9]*"#, error
        0, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, error
        // State 45
        6, // on "(", goto 5
        0, // on ")", error
        0, // on "+", error
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        7, // on "fn", goto 6
        0, // on "let", error
        9, // on "{", goto 8
        0, // on "}", error
        10, // on r#"[1-9][0-9]*"#, goto 9
        11, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, goto 10
        // State 46
        -4, // on "(", reduce `Atom = "{", Block, "}" => ActionFn(9);`
        0, // on ")", error
        -4, // on "+", reduce `Atom = "{", Block, "}" => ActionFn(9);`
        -4, // on ";", reduce `Atom = "{", Block, "}" => ActionFn(9);`
        0, // on "=", error
        0, // on "=>", error
        -4, // on "fn", reduce `Atom = "{", Block, "}" => ActionFn(9);`
        0, // on "let", error
        -4, // on "{", reduce `Atom = "{", Block, "}" => ActionFn(9);`
        0, // on "}", error
        -4, // on r#"[1-9][0-9]*"#, reduce `Atom = "{", Block, "}" => ActionFn(9);`
        -4, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, reduce `Atom = "{", Block, "}" => ActionFn(9);`
        // State 47
        64, // on "(", goto 63
        0, // on ")", error
        0, // on "+", error
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        65, // on "fn", goto 64
        0, // on "let", error
        66, // on "{", goto 65
        0, // on "}", error
        67, // on r#"[1-9][0-9]*"#, goto 66
        68, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, goto 67
        // State 48
        0, // on "(", error
        0, // on ")", error
        0, // on "+", error
        0, // on ";", error
        69, // on "=", goto 68
        0, // on "=>", error
        0, // on "fn", error
        0, // on "let", error
        0, // on "{", error
        0, // on "}", error
        0, // on r#"[1-9][0-9]*"#, error
        0, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, error
        // State 49
        -9, // on "(", reduce `Expr = Expr, Atom => ActionFn(4);`
        0, // on ")", error
        -9, // on "+", reduce `Expr = Expr, Atom => ActionFn(4);`
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        -9, // on "fn", reduce `Expr = Expr, Atom => ActionFn(4);`
        0, // on "let", error
        -9, // on "{", reduce `Expr = Expr, Atom => ActionFn(4);`
        0, // on "}", error
        -9, // on r#"[1-9][0-9]*"#, reduce `Expr = Expr, Atom => ActionFn(4);`
        -9, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, reduce `Expr = Expr, Atom => ActionFn(4);`
        // State 50
        34, // on "(", goto 33
        0, // on ")", error
        0, // on "+", error
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        35, // on "fn", goto 34
        0, // on "let", error
        36, // on "{", goto 35
        0, // on "}", error
        37, // on r#"[1-9][0-9]*"#, goto 36
        38, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, goto 37
        // State 51
        18, // on "(", goto 17
        71, // on ")", goto 70
        41, // on "+", goto 40
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        19, // on "fn", goto 18
        0, // on "let", error
        20, // on "{", goto 19
        0, // on "}", error
        21, // on r#"[1-9][0-9]*"#, goto 20
        22, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, goto 21
        // State 52
        0, // on "(", error
        0, // on ")", error
        0, // on "+", error
        0, // on ";", error
        0, // on "=", error
        72, // on "=>", goto 71
        0, // on "fn", error
        0, // on "let", error
        0, // on "{", error
        0, // on "}", error
        0, // on r#"[1-9][0-9]*"#, error
        0, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, error
        // State 53
        0, // on "(", error
        0, // on ")", error
        0, // on "+", error
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        0, // on "fn", error
        0, // on "let", error
        0, // on "{", error
        73, // on "}", goto 72
        0, // on r#"[1-9][0-9]*"#, error
        0, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, error
        // State 54
        -8, // on "(", reduce `Expr = Expr, "+", Atom => ActionFn(3);`
        -8, // on ")", reduce `Expr = Expr, "+", Atom => ActionFn(3);`
        -8, // on "+", reduce `Expr = Expr, "+", Atom => ActionFn(3);`
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        -8, // on "fn", reduce `Expr = Expr, "+", Atom => ActionFn(3);`
        0, // on "let", error
        -8, // on "{", reduce `Expr = Expr, "+", Atom => ActionFn(3);`
        0, // on "}", error
        -8, // on r#"[1-9][0-9]*"#, reduce `Expr = Expr, "+", Atom => ActionFn(3);`
        -8, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, reduce `Expr = Expr, "+", Atom => ActionFn(3);`
        // State 55
        -3, // on "(", reduce `Atom = "(", Expr, ")" => ActionFn(8);`
        -3, // on ")", reduce `Atom = "(", Expr, ")" => ActionFn(8);`
        -3, // on "+", reduce `Atom = "(", Expr, ")" => ActionFn(8);`
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        -3, // on "fn", reduce `Atom = "(", Expr, ")" => ActionFn(8);`
        0, // on "let", error
        -3, // on "{", reduce `Atom = "(", Expr, ")" => ActionFn(8);`
        0, // on "}", error
        -3, // on r#"[1-9][0-9]*"#, reduce `Atom = "(", Expr, ")" => ActionFn(8);`
        -3, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, reduce `Atom = "(", Expr, ")" => ActionFn(8);`
        // State 56
        0, // on "(", error
        0, // on ")", error
        0, // on "+", error
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        0, // on "fn", error
        0, // on "let", error
        74, // on "{", goto 73
        0, // on "}", error
        0, // on r#"[1-9][0-9]*"#, error
        0, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, error
        // State 57
        -4, // on "(", reduce `Atom = "{", Block, "}" => ActionFn(9);`
        -4, // on ")", reduce `Atom = "{", Block, "}" => ActionFn(9);`
        -4, // on "+", reduce `Atom = "{", Block, "}" => ActionFn(9);`
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        -4, // on "fn", reduce `Atom = "{", Block, "}" => ActionFn(9);`
        0, // on "let", error
        -4, // on "{", reduce `Atom = "{", Block, "}" => ActionFn(9);`
        0, // on "}", error
        -4, // on r#"[1-9][0-9]*"#, reduce `Atom = "{", Block, "}" => ActionFn(9);`
        -4, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, reduce `Atom = "{", Block, "}" => ActionFn(9);`
        // State 58
        64, // on "(", goto 63
        0, // on ")", error
        0, // on "+", error
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        65, // on "fn", goto 64
        0, // on "let", error
        66, // on "{", goto 65
        0, // on "}", error
        67, // on r#"[1-9][0-9]*"#, goto 66
        68, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, goto 67
        // State 59
        6, // on "(", goto 5
        0, // on ")", error
        13, // on "+", goto 12
        76, // on ";", goto 75
        0, // on "=", error
        0, // on "=>", error
        7, // on "fn", goto 6
        0, // on "let", error
        9, // on "{", goto 8
        0, // on "}", error
        10, // on r#"[1-9][0-9]*"#, goto 9
        11, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, goto 10
        // State 60
        -10, // on "(", reduce `Expr = Atom => ActionFn(5);`
        0, // on ")", error
        -10, // on "+", reduce `Expr = Atom => ActionFn(5);`
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        -10, // on "fn", reduce `Expr = Atom => ActionFn(5);`
        0, // on "let", error
        -10, // on "{", reduce `Expr = Atom => ActionFn(5);`
        -10, // on "}", reduce `Expr = Atom => ActionFn(5);`
        -10, // on r#"[1-9][0-9]*"#, reduce `Expr = Atom => ActionFn(5);`
        -10, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, reduce `Expr = Atom => ActionFn(5);`
        // State 61
        64, // on "(", goto 63
        0, // on ")", error
        78, // on "+", goto 77
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        65, // on "fn", goto 64
        0, // on "let", error
        66, // on "{", goto 65
        -7, // on "}", reduce `Block = Expr, ";", Expr => ActionFn(2);`
        67, // on r#"[1-9][0-9]*"#, goto 66
        68, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, goto 67
        // State 62
        -2, // on "(", reduce `Atom = Id => ActionFn(7);`
        0, // on ")", error
        -2, // on "+", reduce `Atom = Id => ActionFn(7);`
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        -2, // on "fn", reduce `Atom = Id => ActionFn(7);`
        0, // on "let", error
        -2, // on "{", reduce `Atom = Id => ActionFn(7);`
        -2, // on "}", reduce `Atom = Id => ActionFn(7);`
        -2, // on r#"[1-9][0-9]*"#, reduce `Atom = Id => ActionFn(7);`
        -2, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, reduce `Atom = Id => ActionFn(7);`
        // State 63
        18, // on "(", goto 17
        0, // on ")", error
        0, // on "+", error
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        19, // on "fn", goto 18
        0, // on "let", error
        20, // on "{", goto 19
        0, // on "}", error
        21, // on r#"[1-9][0-9]*"#, goto 20
        22, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, goto 21
        // State 64
        0, // on "(", error
        0, // on ")", error
        0, // on "+", error
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        0, // on "fn", error
        0, // on "let", error
        0, // on "{", error
        0, // on "}", error
        0, // on r#"[1-9][0-9]*"#, error
        24, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, goto 23
        // State 65
        6, // on "(", goto 5
        0, // on ")", error
        0, // on "+", error
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        7, // on "fn", goto 6
        29, // on "let", goto 28
        9, // on "{", goto 8
        0, // on "}", error
        10, // on r#"[1-9][0-9]*"#, goto 9
        11, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, goto 10
        // State 66
        -1, // on "(", reduce `Atom = r#"[1-9][0-9]*"# => ActionFn(6);`
        0, // on ")", error
        -1, // on "+", reduce `Atom = r#"[1-9][0-9]*"# => ActionFn(6);`
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        -1, // on "fn", reduce `Atom = r#"[1-9][0-9]*"# => ActionFn(6);`
        0, // on "let", error
        -1, // on "{", reduce `Atom = r#"[1-9][0-9]*"# => ActionFn(6);`
        -1, // on "}", reduce `Atom = r#"[1-9][0-9]*"# => ActionFn(6);`
        -1, // on r#"[1-9][0-9]*"#, reduce `Atom = r#"[1-9][0-9]*"# => ActionFn(6);`
        -1, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, reduce `Atom = r#"[1-9][0-9]*"# => ActionFn(6);`
        // State 67
        -11, // on "(", reduce `Id = r#"[a-zA-Z][a-zA-Z0-9]*"# => ActionFn(11);`
        0, // on ")", error
        -11, // on "+", reduce `Id = r#"[a-zA-Z][a-zA-Z0-9]*"# => ActionFn(11);`
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        -11, // on "fn", reduce `Id = r#"[a-zA-Z][a-zA-Z0-9]*"# => ActionFn(11);`
        0, // on "let", error
        -11, // on "{", reduce `Id = r#"[a-zA-Z][a-zA-Z0-9]*"# => ActionFn(11);`
        -11, // on "}", reduce `Id = r#"[a-zA-Z][a-zA-Z0-9]*"# => ActionFn(11);`
        -11, // on r#"[1-9][0-9]*"#, reduce `Id = r#"[a-zA-Z][a-zA-Z0-9]*"# => ActionFn(11);`
        -11, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, reduce `Id = r#"[a-zA-Z][a-zA-Z0-9]*"# => ActionFn(11);`
        // State 68
        6, // on "(", goto 5
        0, // on ")", error
        0, // on "+", error
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        7, // on "fn", goto 6
        0, // on "let", error
        9, // on "{", goto 8
        0, // on "}", error
        10, // on r#"[1-9][0-9]*"#, goto 9
        11, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, goto 10
        // State 69
        -8, // on "(", reduce `Expr = Expr, "+", Atom => ActionFn(3);`
        0, // on ")", error
        -8, // on "+", reduce `Expr = Expr, "+", Atom => ActionFn(3);`
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        -8, // on "fn", reduce `Expr = Expr, "+", Atom => ActionFn(3);`
        0, // on "let", error
        -8, // on "{", reduce `Expr = Expr, "+", Atom => ActionFn(3);`
        0, // on "}", error
        -8, // on r#"[1-9][0-9]*"#, reduce `Expr = Expr, "+", Atom => ActionFn(3);`
        -8, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, reduce `Expr = Expr, "+", Atom => ActionFn(3);`
        // State 70
        -3, // on "(", reduce `Atom = "(", Expr, ")" => ActionFn(8);`
        0, // on ")", error
        -3, // on "+", reduce `Atom = "(", Expr, ")" => ActionFn(8);`
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        -3, // on "fn", reduce `Atom = "(", Expr, ")" => ActionFn(8);`
        0, // on "let", error
        -3, // on "{", reduce `Atom = "(", Expr, ")" => ActionFn(8);`
        0, // on "}", error
        -3, // on r#"[1-9][0-9]*"#, reduce `Atom = "(", Expr, ")" => ActionFn(8);`
        -3, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, reduce `Atom = "(", Expr, ")" => ActionFn(8);`
        // State 71
        0, // on "(", error
        0, // on ")", error
        0, // on "+", error
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        0, // on "fn", error
        0, // on "let", error
        83, // on "{", goto 82
        0, // on "}", error
        0, // on r#"[1-9][0-9]*"#, error
        0, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, error
        // State 72
        -4, // on "(", reduce `Atom = "{", Block, "}" => ActionFn(9);`
        0, // on ")", error
        -4, // on "+", reduce `Atom = "{", Block, "}" => ActionFn(9);`
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        -4, // on "fn", reduce `Atom = "{", Block, "}" => ActionFn(9);`
        0, // on "let", error
        -4, // on "{", reduce `Atom = "{", Block, "}" => ActionFn(9);`
        0, // on "}", error
        -4, // on r#"[1-9][0-9]*"#, reduce `Atom = "{", Block, "}" => ActionFn(9);`
        -4, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, reduce `Atom = "{", Block, "}" => ActionFn(9);`
        // State 73
        64, // on "(", goto 63
        0, // on ")", error
        0, // on "+", error
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        65, // on "fn", goto 64
        0, // on "let", error
        66, // on "{", goto 65
        0, // on "}", error
        67, // on r#"[1-9][0-9]*"#, goto 66
        68, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, goto 67
        // State 74
        64, // on "(", goto 63
        0, // on ")", error
        78, // on "+", goto 77
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        65, // on "fn", goto 64
        0, // on "let", error
        66, // on "{", goto 65
        85, // on "}", goto 84
        67, // on r#"[1-9][0-9]*"#, goto 66
        68, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, goto 67
        // State 75
        34, // on "(", goto 33
        0, // on ")", error
        0, // on "+", error
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        35, // on "fn", goto 34
        0, // on "let", error
        36, // on "{", goto 35
        0, // on "}", error
        37, // on r#"[1-9][0-9]*"#, goto 36
        38, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, goto 37
        // State 76
        -9, // on "(", reduce `Expr = Expr, Atom => ActionFn(4);`
        0, // on ")", error
        -9, // on "+", reduce `Expr = Expr, Atom => ActionFn(4);`
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        -9, // on "fn", reduce `Expr = Expr, Atom => ActionFn(4);`
        0, // on "let", error
        -9, // on "{", reduce `Expr = Expr, Atom => ActionFn(4);`
        -9, // on "}", reduce `Expr = Expr, Atom => ActionFn(4);`
        -9, // on r#"[1-9][0-9]*"#, reduce `Expr = Expr, Atom => ActionFn(4);`
        -9, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, reduce `Expr = Expr, Atom => ActionFn(4);`
        // State 77
        64, // on "(", goto 63
        0, // on ")", error
        0, // on "+", error
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        65, // on "fn", goto 64
        0, // on "let", error
        66, // on "{", goto 65
        0, // on "}", error
        67, // on r#"[1-9][0-9]*"#, goto 66
        68, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, goto 67
        // State 78
        18, // on "(", goto 17
        88, // on ")", goto 87
        41, // on "+", goto 40
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        19, // on "fn", goto 18
        0, // on "let", error
        20, // on "{", goto 19
        0, // on "}", error
        21, // on r#"[1-9][0-9]*"#, goto 20
        22, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, goto 21
        // State 79
        0, // on "(", error
        0, // on ")", error
        0, // on "+", error
        0, // on ";", error
        0, // on "=", error
        89, // on "=>", goto 88
        0, // on "fn", error
        0, // on "let", error
        0, // on "{", error
        0, // on "}", error
        0, // on r#"[1-9][0-9]*"#, error
        0, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, error
        // State 80
        0, // on "(", error
        0, // on ")", error
        0, // on "+", error
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        0, // on "fn", error
        0, // on "let", error
        0, // on "{", error
        90, // on "}", goto 89
        0, // on r#"[1-9][0-9]*"#, error
        0, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, error
        // State 81
        6, // on "(", goto 5
        0, // on ")", error
        13, // on "+", goto 12
        91, // on ";", goto 90
        0, // on "=", error
        0, // on "=>", error
        7, // on "fn", goto 6
        0, // on "let", error
        9, // on "{", goto 8
        0, // on "}", error
        10, // on r#"[1-9][0-9]*"#, goto 9
        11, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, goto 10
        // State 82
        64, // on "(", goto 63
        0, // on ")", error
        0, // on "+", error
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        65, // on "fn", goto 64
        0, // on "let", error
        66, // on "{", goto 65
        0, // on "}", error
        67, // on r#"[1-9][0-9]*"#, goto 66
        68, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, goto 67
        // State 83
        64, // on "(", goto 63
        0, // on ")", error
        78, // on "+", goto 77
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        65, // on "fn", goto 64
        0, // on "let", error
        66, // on "{", goto 65
        93, // on "}", goto 92
        67, // on r#"[1-9][0-9]*"#, goto 66
        68, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, goto 67
        // State 84
        -5, // on "(", reduce `Atom = "fn", Id, "=>", "{", Expr, "}" => ActionFn(10);`
        0, // on ")", error
        -5, // on "+", reduce `Atom = "fn", Id, "=>", "{", Expr, "}" => ActionFn(10);`
        -5, // on ";", reduce `Atom = "fn", Id, "=>", "{", Expr, "}" => ActionFn(10);`
        0, // on "=", error
        0, // on "=>", error
        -5, // on "fn", reduce `Atom = "fn", Id, "=>", "{", Expr, "}" => ActionFn(10);`
        0, // on "let", error
        -5, // on "{", reduce `Atom = "fn", Id, "=>", "{", Expr, "}" => ActionFn(10);`
        0, // on "}", error
        -5, // on r#"[1-9][0-9]*"#, reduce `Atom = "fn", Id, "=>", "{", Expr, "}" => ActionFn(10);`
        -5, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, reduce `Atom = "fn", Id, "=>", "{", Expr, "}" => ActionFn(10);`
        // State 85
        34, // on "(", goto 33
        0, // on ")", error
        51, // on "+", goto 50
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        35, // on "fn", goto 34
        0, // on "let", error
        36, // on "{", goto 35
        0, // on "}", error
        37, // on r#"[1-9][0-9]*"#, goto 36
        38, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, goto 37
        // State 86
        -8, // on "(", reduce `Expr = Expr, "+", Atom => ActionFn(3);`
        0, // on ")", error
        -8, // on "+", reduce `Expr = Expr, "+", Atom => ActionFn(3);`
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        -8, // on "fn", reduce `Expr = Expr, "+", Atom => ActionFn(3);`
        0, // on "let", error
        -8, // on "{", reduce `Expr = Expr, "+", Atom => ActionFn(3);`
        -8, // on "}", reduce `Expr = Expr, "+", Atom => ActionFn(3);`
        -8, // on r#"[1-9][0-9]*"#, reduce `Expr = Expr, "+", Atom => ActionFn(3);`
        -8, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, reduce `Expr = Expr, "+", Atom => ActionFn(3);`
        // State 87
        -3, // on "(", reduce `Atom = "(", Expr, ")" => ActionFn(8);`
        0, // on ")", error
        -3, // on "+", reduce `Atom = "(", Expr, ")" => ActionFn(8);`
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        -3, // on "fn", reduce `Atom = "(", Expr, ")" => ActionFn(8);`
        0, // on "let", error
        -3, // on "{", reduce `Atom = "(", Expr, ")" => ActionFn(8);`
        -3, // on "}", reduce `Atom = "(", Expr, ")" => ActionFn(8);`
        -3, // on r#"[1-9][0-9]*"#, reduce `Atom = "(", Expr, ")" => ActionFn(8);`
        -3, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, reduce `Atom = "(", Expr, ")" => ActionFn(8);`
        // State 88
        0, // on "(", error
        0, // on ")", error
        0, // on "+", error
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        0, // on "fn", error
        0, // on "let", error
        94, // on "{", goto 93
        0, // on "}", error
        0, // on r#"[1-9][0-9]*"#, error
        0, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, error
        // State 89
        -4, // on "(", reduce `Atom = "{", Block, "}" => ActionFn(9);`
        0, // on ")", error
        -4, // on "+", reduce `Atom = "{", Block, "}" => ActionFn(9);`
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        -4, // on "fn", reduce `Atom = "{", Block, "}" => ActionFn(9);`
        0, // on "let", error
        -4, // on "{", reduce `Atom = "{", Block, "}" => ActionFn(9);`
        -4, // on "}", reduce `Atom = "{", Block, "}" => ActionFn(9);`
        -4, // on r#"[1-9][0-9]*"#, reduce `Atom = "{", Block, "}" => ActionFn(9);`
        -4, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, reduce `Atom = "{", Block, "}" => ActionFn(9);`
        // State 90
        64, // on "(", goto 63
        0, // on ")", error
        0, // on "+", error
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        65, // on "fn", goto 64
        0, // on "let", error
        66, // on "{", goto 65
        0, // on "}", error
        67, // on r#"[1-9][0-9]*"#, goto 66
        68, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, goto 67
        // State 91
        64, // on "(", goto 63
        0, // on ")", error
        78, // on "+", goto 77
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        65, // on "fn", goto 64
        0, // on "let", error
        66, // on "{", goto 65
        96, // on "}", goto 95
        67, // on r#"[1-9][0-9]*"#, goto 66
        68, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, goto 67
        // State 92
        -5, // on "(", reduce `Atom = "fn", Id, "=>", "{", Expr, "}" => ActionFn(10);`
        -5, // on ")", reduce `Atom = "fn", Id, "=>", "{", Expr, "}" => ActionFn(10);`
        -5, // on "+", reduce `Atom = "fn", Id, "=>", "{", Expr, "}" => ActionFn(10);`
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        -5, // on "fn", reduce `Atom = "fn", Id, "=>", "{", Expr, "}" => ActionFn(10);`
        0, // on "let", error
        -5, // on "{", reduce `Atom = "fn", Id, "=>", "{", Expr, "}" => ActionFn(10);`
        0, // on "}", error
        -5, // on r#"[1-9][0-9]*"#, reduce `Atom = "fn", Id, "=>", "{", Expr, "}" => ActionFn(10);`
        -5, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, reduce `Atom = "fn", Id, "=>", "{", Expr, "}" => ActionFn(10);`
        // State 93
        64, // on "(", goto 63
        0, // on ")", error
        0, // on "+", error
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        65, // on "fn", goto 64
        0, // on "let", error
        66, // on "{", goto 65
        0, // on "}", error
        67, // on r#"[1-9][0-9]*"#, goto 66
        68, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, goto 67
        // State 94
        64, // on "(", goto 63
        0, // on ")", error
        78, // on "+", goto 77
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        65, // on "fn", goto 64
        0, // on "let", error
        66, // on "{", goto 65
        -6, // on "}", reduce `Block = "let", Id, "=", Expr, ";", Expr => ActionFn(1);`
        67, // on r#"[1-9][0-9]*"#, goto 66
        68, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, goto 67
        // State 95
        -5, // on "(", reduce `Atom = "fn", Id, "=>", "{", Expr, "}" => ActionFn(10);`
        0, // on ")", error
        -5, // on "+", reduce `Atom = "fn", Id, "=>", "{", Expr, "}" => ActionFn(10);`
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        -5, // on "fn", reduce `Atom = "fn", Id, "=>", "{", Expr, "}" => ActionFn(10);`
        0, // on "let", error
        -5, // on "{", reduce `Atom = "fn", Id, "=>", "{", Expr, "}" => ActionFn(10);`
        0, // on "}", error
        -5, // on r#"[1-9][0-9]*"#, reduce `Atom = "fn", Id, "=>", "{", Expr, "}" => ActionFn(10);`
        -5, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, reduce `Atom = "fn", Id, "=>", "{", Expr, "}" => ActionFn(10);`
        // State 96
        64, // on "(", goto 63
        0, // on ")", error
        78, // on "+", goto 77
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        65, // on "fn", goto 64
        0, // on "let", error
        66, // on "{", goto 65
        98, // on "}", goto 97
        67, // on r#"[1-9][0-9]*"#, goto 66
        68, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, goto 67
        // State 97
        -5, // on "(", reduce `Atom = "fn", Id, "=>", "{", Expr, "}" => ActionFn(10);`
        0, // on ")", error
        -5, // on "+", reduce `Atom = "fn", Id, "=>", "{", Expr, "}" => ActionFn(10);`
        0, // on ";", error
        0, // on "=", error
        0, // on "=>", error
        -5, // on "fn", reduce `Atom = "fn", Id, "=>", "{", Expr, "}" => ActionFn(10);`
        0, // on "let", error
        -5, // on "{", reduce `Atom = "fn", Id, "=>", "{", Expr, "}" => ActionFn(10);`
        -5, // on "}", reduce `Atom = "fn", Id, "=>", "{", Expr, "}" => ActionFn(10);`
        -5, // on r#"[1-9][0-9]*"#, reduce `Atom = "fn", Id, "=>", "{", Expr, "}" => ActionFn(10);`
        -5, // on r#"[a-zA-Z][a-zA-Z0-9]*"#, reduce `Atom = "fn", Id, "=>", "{", Expr, "}" => ActionFn(10);`
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0, // on EOF, error
        0, // on EOF, error
        -12, // on EOF, reduce `__Block = Block => ActionFn(0);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -10, // on EOF, reduce `Expr = Atom => ActionFn(5);`
        -7, // on EOF, reduce `Block = Expr, ";", Expr => ActionFn(2);`
        -2, // on EOF, reduce `Atom = Id => ActionFn(7);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -1, // on EOF, reduce `Atom = r#"[1-9][0-9]*"# => ActionFn(6);`
        -11, // on EOF, reduce `Id = r#"[a-zA-Z][a-zA-Z0-9]*"# => ActionFn(11);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -9, // on EOF, reduce `Expr = Expr, Atom => ActionFn(4);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -8, // on EOF, reduce `Expr = Expr, "+", Atom => ActionFn(3);`
        -3, // on EOF, reduce `Atom = "(", Expr, ")" => ActionFn(8);`
        0, // on EOF, error
        -4, // on EOF, reduce `Atom = "{", Block, "}" => ActionFn(9);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -6, // on EOF, reduce `Block = "let", Id, "=", Expr, ";", Expr => ActionFn(1);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -5, // on EOF, reduce `Atom = "fn", Id, "=>", "{", Expr, "}" => ActionFn(10);`
        0, // on EOF, error
        0, // on EOF, error
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        2, // on Atom, goto 1
        3, // on Block, goto 2
        4, // on Expr, goto 3
        5, // on Id, goto 4
        0, // on __Block, error
        // State 1
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 2
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 3
        12, // on Atom, goto 11
        0, // on Block, error
        0, // on Expr, error
        5, // on Id, goto 4
        0, // on __Block, error
        // State 4
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 5
        15, // on Atom, goto 14
        0, // on Block, error
        16, // on Expr, goto 15
        17, // on Id, goto 16
        0, // on __Block, error
        // State 6
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        23, // on Id, goto 22
        0, // on __Block, error
        // State 7
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        25, // on Id, goto 24
        0, // on __Block, error
        // State 8
        2, // on Atom, goto 1
        27, // on Block, goto 26
        28, // on Expr, goto 27
        5, // on Id, goto 4
        0, // on __Block, error
        // State 9
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 10
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 11
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 12
        30, // on Atom, goto 29
        0, // on Block, error
        0, // on Expr, error
        5, // on Id, goto 4
        0, // on __Block, error
        // State 13
        31, // on Atom, goto 30
        0, // on Block, error
        32, // on Expr, goto 31
        33, // on Id, goto 32
        0, // on __Block, error
        // State 14
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 15
        39, // on Atom, goto 38
        0, // on Block, error
        0, // on Expr, error
        17, // on Id, goto 16
        0, // on __Block, error
        // State 16
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 17
        15, // on Atom, goto 14
        0, // on Block, error
        42, // on Expr, goto 41
        17, // on Id, goto 16
        0, // on __Block, error
        // State 18
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        43, // on Id, goto 42
        0, // on __Block, error
        // State 19
        2, // on Atom, goto 1
        44, // on Block, goto 43
        28, // on Expr, goto 27
        5, // on Id, goto 4
        0, // on __Block, error
        // State 20
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 21
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 22
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 23
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 24
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 25
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 26
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 27
        12, // on Atom, goto 11
        0, // on Block, error
        0, // on Expr, error
        5, // on Id, goto 4
        0, // on __Block, error
        // State 28
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        49, // on Id, goto 48
        0, // on __Block, error
        // State 29
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 30
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 31
        50, // on Atom, goto 49
        0, // on Block, error
        0, // on Expr, error
        33, // on Id, goto 32
        0, // on __Block, error
        // State 32
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 33
        15, // on Atom, goto 14
        0, // on Block, error
        52, // on Expr, goto 51
        17, // on Id, goto 16
        0, // on __Block, error
        // State 34
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        53, // on Id, goto 52
        0, // on __Block, error
        // State 35
        2, // on Atom, goto 1
        54, // on Block, goto 53
        28, // on Expr, goto 27
        5, // on Id, goto 4
        0, // on __Block, error
        // State 36
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 37
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 38
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 39
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 40
        55, // on Atom, goto 54
        0, // on Block, error
        0, // on Expr, error
        17, // on Id, goto 16
        0, // on __Block, error
        // State 41
        39, // on Atom, goto 38
        0, // on Block, error
        0, // on Expr, error
        17, // on Id, goto 16
        0, // on __Block, error
        // State 42
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 43
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 44
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 45
        2, // on Atom, goto 1
        0, // on Block, error
        60, // on Expr, goto 59
        5, // on Id, goto 4
        0, // on __Block, error
        // State 46
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 47
        61, // on Atom, goto 60
        0, // on Block, error
        62, // on Expr, goto 61
        63, // on Id, goto 62
        0, // on __Block, error
        // State 48
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 49
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 50
        70, // on Atom, goto 69
        0, // on Block, error
        0, // on Expr, error
        33, // on Id, goto 32
        0, // on __Block, error
        // State 51
        39, // on Atom, goto 38
        0, // on Block, error
        0, // on Expr, error
        17, // on Id, goto 16
        0, // on __Block, error
        // State 52
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 53
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 54
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 55
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 56
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 57
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 58
        61, // on Atom, goto 60
        0, // on Block, error
        75, // on Expr, goto 74
        63, // on Id, goto 62
        0, // on __Block, error
        // State 59
        12, // on Atom, goto 11
        0, // on Block, error
        0, // on Expr, error
        5, // on Id, goto 4
        0, // on __Block, error
        // State 60
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 61
        77, // on Atom, goto 76
        0, // on Block, error
        0, // on Expr, error
        63, // on Id, goto 62
        0, // on __Block, error
        // State 62
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 63
        15, // on Atom, goto 14
        0, // on Block, error
        79, // on Expr, goto 78
        17, // on Id, goto 16
        0, // on __Block, error
        // State 64
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        80, // on Id, goto 79
        0, // on __Block, error
        // State 65
        2, // on Atom, goto 1
        81, // on Block, goto 80
        28, // on Expr, goto 27
        5, // on Id, goto 4
        0, // on __Block, error
        // State 66
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 67
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 68
        2, // on Atom, goto 1
        0, // on Block, error
        82, // on Expr, goto 81
        5, // on Id, goto 4
        0, // on __Block, error
        // State 69
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 70
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 71
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 72
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 73
        61, // on Atom, goto 60
        0, // on Block, error
        84, // on Expr, goto 83
        63, // on Id, goto 62
        0, // on __Block, error
        // State 74
        77, // on Atom, goto 76
        0, // on Block, error
        0, // on Expr, error
        63, // on Id, goto 62
        0, // on __Block, error
        // State 75
        31, // on Atom, goto 30
        0, // on Block, error
        86, // on Expr, goto 85
        33, // on Id, goto 32
        0, // on __Block, error
        // State 76
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 77
        87, // on Atom, goto 86
        0, // on Block, error
        0, // on Expr, error
        63, // on Id, goto 62
        0, // on __Block, error
        // State 78
        39, // on Atom, goto 38
        0, // on Block, error
        0, // on Expr, error
        17, // on Id, goto 16
        0, // on __Block, error
        // State 79
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 80
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 81
        12, // on Atom, goto 11
        0, // on Block, error
        0, // on Expr, error
        5, // on Id, goto 4
        0, // on __Block, error
        // State 82
        61, // on Atom, goto 60
        0, // on Block, error
        92, // on Expr, goto 91
        63, // on Id, goto 62
        0, // on __Block, error
        // State 83
        77, // on Atom, goto 76
        0, // on Block, error
        0, // on Expr, error
        63, // on Id, goto 62
        0, // on __Block, error
        // State 84
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 85
        50, // on Atom, goto 49
        0, // on Block, error
        0, // on Expr, error
        33, // on Id, goto 32
        0, // on __Block, error
        // State 86
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 87
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 88
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 89
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 90
        61, // on Atom, goto 60
        0, // on Block, error
        95, // on Expr, goto 94
        63, // on Id, goto 62
        0, // on __Block, error
        // State 91
        77, // on Atom, goto 76
        0, // on Block, error
        0, // on Expr, error
        63, // on Id, goto 62
        0, // on __Block, error
        // State 92
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 93
        61, // on Atom, goto 60
        0, // on Block, error
        97, // on Expr, goto 96
        63, // on Id, goto 62
        0, // on __Block, error
        // State 94
        77, // on Atom, goto 76
        0, // on Block, error
        0, // on Expr, error
        63, // on Id, goto 62
        0, // on __Block, error
        // State 95
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 96
        77, // on Atom, goto 76
        0, // on Block, error
        0, // on Expr, error
        63, // on Id, goto 62
        0, // on __Block, error
        // State 97
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
    ];
    pub fn parse_Block<
        'input,
    >(
        input: &'input str,
    ) -> Result<Term, __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        '__shift: loop {
            let __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            let __integer = match __lookahead {
                (_, (0, _), _) if true => 0,
                (_, (1, _), _) if true => 1,
                (_, (2, _), _) if true => 2,
                (_, (3, _), _) if true => 3,
                (_, (4, _), _) if true => 4,
                (_, (5, _), _) if true => 5,
                (_, (6, _), _) if true => 6,
                (_, (7, _), _) if true => 7,
                (_, (8, _), _) if true => 8,
                (_, (9, _), _) if true => 9,
                (_, (10, _), _) if true => 10,
                (_, (11, _), _) if true => 11,
                _ => {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            };
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 12 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Term_22_28_22(__tok0),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Term_22_29_22(__tok0),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Term_22_2b_22(__tok0),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22_3b_22(__tok0),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_3d_22(__tok0),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22_3d_3e_22(__tok0),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22fn_22(__tok0),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22let_22(__tok0),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22_7b_22(__tok0),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22_7d_22(__tok0),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Termr_23_22_5b1_2d9_5d_5b0_2d9_5d_2a_22_23(__tok0),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            (11, __tok0) => __Symbol::Termr_23_22_5ba_2dzA_2dZ_5d_5ba_2dzA_2dZ0_2d9_5d_2a_22_23(__tok0),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols) {
                        return r;
                    }
                } else {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(input, __action, None, &mut __states, &mut __symbols) {
                    return r;
                }
            } else {
                return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: vec![],
                });
            }
        }
    }
    pub fn __reduce<
        'input,
    >(
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
    ) -> Option<Result<Term,__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // Atom = r#"[1-9][0-9]*"# => ActionFn(6);
                let __sym0 = __pop_Termr_23_22_5b1_2d9_5d_5b0_2d9_5d_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                0
            }
            2 => {
                // Atom = Id => ActionFn(7);
                let __sym0 = __pop_NtId(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                0
            }
            3 => {
                // Atom = "(", Expr, ")" => ActionFn(8);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action8(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                0
            }
            4 => {
                // Atom = "{", Block, "}" => ActionFn(9);
                let __sym2 = __pop_Term_22_7d_22(__symbols);
                let __sym1 = __pop_NtBlock(__symbols);
                let __sym0 = __pop_Term_22_7b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action9(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                0
            }
            5 => {
                // Atom = "fn", Id, "=>", "{", Expr, "}" => ActionFn(10);
                let __sym5 = __pop_Term_22_7d_22(__symbols);
                let __sym4 = __pop_NtExpr(__symbols);
                let __sym3 = __pop_Term_22_7b_22(__symbols);
                let __sym2 = __pop_Term_22_3d_3e_22(__symbols);
                let __sym1 = __pop_NtId(__symbols);
                let __sym0 = __pop_Term_22fn_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym5.2.clone();
                let __nt = super::__action10(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
                let __states_len = __states.len();
                __states.truncate(__states_len - 6);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                0
            }
            6 => {
                // Block = "let", Id, "=", Expr, ";", Expr => ActionFn(1);
                let __sym5 = __pop_NtExpr(__symbols);
                let __sym4 = __pop_Term_22_3b_22(__symbols);
                let __sym3 = __pop_NtExpr(__symbols);
                let __sym2 = __pop_Term_22_3d_22(__symbols);
                let __sym1 = __pop_NtId(__symbols);
                let __sym0 = __pop_Term_22let_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym5.2.clone();
                let __nt = super::__action1(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
                let __states_len = __states.len();
                __states.truncate(__states_len - 6);
                __symbols.push((__start, __Symbol::NtBlock(__nt), __end));
                1
            }
            7 => {
                // Block = Expr, ";", Expr => ActionFn(2);
                let __sym2 = __pop_NtExpr(__symbols);
                let __sym1 = __pop_Term_22_3b_22(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action2(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtBlock(__nt), __end));
                1
            }
            8 => {
                // Expr = Expr, "+", Atom => ActionFn(3);
                let __sym2 = __pop_NtAtom(__symbols);
                let __sym1 = __pop_Term_22_2b_22(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action3(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                2
            }
            9 => {
                // Expr = Expr, Atom => ActionFn(4);
                let __sym1 = __pop_NtAtom(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action4(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                2
            }
            10 => {
                // Expr = Atom => ActionFn(5);
                let __sym0 = __pop_NtAtom(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                2
            }
            11 => {
                // Id = r#"[a-zA-Z][a-zA-Z0-9]*"# => ActionFn(11);
                let __sym0 = __pop_Termr_23_22_5ba_2dzA_2dZ_5d_5ba_2dzA_2dZ0_2d9_5d_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtId(__nt), __end));
                3
            }
            12 => {
                // __Block = Block => ActionFn(0);
                let __sym0 = __pop_NtBlock(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0(input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 5 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_29_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3d_3e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3d_3e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22fn_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22fn_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22let_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22let_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_7b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_7b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_7d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_7d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b1_2d9_5d_5b0_2d9_5d_2a_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b1_2d9_5d_5b0_2d9_5d_2a_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5ba_2dzA_2dZ_5d_5ba_2dzA_2dZ0_2d9_5d_2a_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5ba_2dzA_2dZ_5d_5ba_2dzA_2dZ0_2d9_5d_2a_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtAtom<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Term, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtAtom(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtBlock<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Term, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtBlock(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExpr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Term, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExpr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtId<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Var, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtId(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Block<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Term, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Block(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Block::parse_Block;
mod __intern_token {
    extern crate lalrpop_util as __lalrpop_util;
    pub struct __Matcher<'input> {
        text: &'input str,
        consumed: usize,
    }

    fn __tokenize(text: &str) -> Option<(usize, usize)> {
        let mut __chars = text.char_indices();
        let mut __current_match: Option<(usize, usize)> = None;
        let mut __current_state: usize = 0;
        loop {
            match __current_state {
                0 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        40 => /* '(' */ {
                            __current_match = Some((0, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        41 => /* ')' */ {
                            __current_match = Some((1, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        43 => /* '+' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        49 ... 57 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 4;
                            continue;
                        }
                        59 => /* ';' */ {
                            __current_match = Some((3, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        61 => /* '=' */ {
                            __current_match = Some((4, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((11, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        97 ... 101 => {
                            __current_match = Some((11, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        102 => /* 'f' */ {
                            __current_match = Some((11, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        103 ... 107 => {
                            __current_match = Some((11, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        108 => /* 'l' */ {
                            __current_match = Some((11, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        109 ... 122 => {
                            __current_match = Some((11, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        123 => /* '{' */ {
                            __current_match = Some((8, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        125 => /* '}' */ {
                            __current_match = Some((9, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                1 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                2 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                3 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                4 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 13;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                5 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                6 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        62 => /* '>' */ {
                            __current_match = Some((5, __index + 1));
                            __current_state = 14;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                7 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((11, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((11, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((11, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                8 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((11, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((11, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        97 ... 109 => {
                            __current_match = Some((11, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        110 => /* 'n' */ {
                            __current_match = Some((6, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        111 ... 122 => {
                            __current_match = Some((11, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                9 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((11, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((11, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        97 ... 100 => {
                            __current_match = Some((11, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_match = Some((11, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        102 ... 122 => {
                            __current_match = Some((11, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                10 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                11 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                12 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                13 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((10, __index + __ch.len_utf8()));
                            __current_state = 13;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                14 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                15 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((11, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((11, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((11, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                16 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((11, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((11, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((11, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                17 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((11, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((11, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        97 ... 115 => {
                            __current_match = Some((11, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        116 => /* 't' */ {
                            __current_match = Some((7, __index + 1));
                            __current_state = 18;
                            continue;
                        }
                        117 ... 122 => {
                            __current_match = Some((11, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                18 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((11, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((11, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((11, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                _ => { panic!("invalid state {}", __current_state); }
            }
        }
    }

    impl<'input> __Matcher<'input> {
        pub fn new(s: &'input str) -> __Matcher<'input> {
            __Matcher { text: s, consumed: 0 }
        }
    }

    impl<'input> Iterator for __Matcher<'input> {
        type Item = Result<(usize, (usize, &'input str), usize), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>;

        fn next(&mut self) -> Option<Self::Item> {
            let __text = self.text.trim_left();
            let __whitespace = self.text.len() - __text.len();
            let __start_offset = self.consumed + __whitespace;
            if __text.is_empty() {
                self.text = __text;
                self.consumed = __start_offset;
                None
            } else {
                match __tokenize(__text) {
                    Some((__index, __length)) => {
                        let __result = &__text[..__length];
                        let __remaining = &__text[__length..];
                        let __end_offset = __start_offset + __length;
                        self.text = __remaining;
                        self.consumed = __end_offset;
                        Some(Ok((__start_offset, (__index, __result), __end_offset)))
                    }
                    None => {
                        Some(Err(__lalrpop_util::ParseError::InvalidToken { location: __start_offset }))
                    }
                }
            }
        }
    }
}

#[allow(unused_variables)]
pub fn __action0<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Term, usize),
) -> Term
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action1<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, id, _): (usize, Var, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, bind, _): (usize, Term, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, body, _): (usize, Term, usize),
) -> Term
{
    into(View::Let((bind, (id, body))))
}

#[allow(unused_variables)]
pub fn __action2<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, Term, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, Term, usize),
) -> Term
{
    into(View::Let((l, (Var::new("dummy".to_string()), r))))
}

#[allow(unused_variables)]
pub fn __action3<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, Term, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, Term, usize),
) -> Term
{
    into(View::Plus((l, r)))
}

#[allow(unused_variables)]
pub fn __action4<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, Term, usize),
    (_, r, _): (usize, Term, usize),
) -> Term
{
    into(View::App((l, r)))
}

#[allow(unused_variables)]
pub fn __action5<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Term, usize),
) -> Term
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action6<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Term
{
    into(View::Number(i32::from_str(__0).unwrap()))
}

#[allow(unused_variables)]
pub fn __action7<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Var, usize),
) -> Term
{
    into(View::Var(__0))
}

#[allow(unused_variables)]
pub fn __action8<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, e, _): (usize, Term, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Term
{
    e
}

#[allow(unused_variables)]
pub fn __action9<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, e, _): (usize, Term, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Term
{
    e
}

#[allow(unused_variables)]
pub fn __action10<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, id, _): (usize, Var, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, e, _): (usize, Term, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Term
{
    into(View::Lam((id, e)))
}

#[allow(unused_variables)]
pub fn __action11<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Var
{
    Var::new((__0).to_string())
}

pub trait __ToTriple<'input, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),Self::Error>;
}

impl<'input, > __ToTriple<'input, > for (usize, (usize, &'input str), usize) {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, (usize, &'input str), usize),()> {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        value
    }
}
