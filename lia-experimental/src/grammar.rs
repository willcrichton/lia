use std::str::FromStr;
use super::token::Token;
use super::ast::{term, typ};
use super::ast::term::{Term, View as TermV};
use super::ast::typ::{Typ, View as TypV};
use rabbot::var::Var;
extern crate lalrpop_util as __lalrpop_util;

mod __parse__Block {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use super::super::token::Token;
    use super::super::ast::{term, typ};
    use super::super::ast::term::{Term, View as TermV};
    use super::super::ast::typ::{Typ, View as TypV};
    use rabbot::var::Var;
    extern crate lalrpop_util as __lalrpop_util;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub enum __Symbol<> {
        TermArrow(Token),
        TermEq(Token),
        TermFun(Token),
        TermIdT(String),
        TermInt(i32),
        TermLbrace(Token),
        TermLet(Token),
        TermLparen(Token),
        TermPlus(Token),
        TermQuote(String),
        TermRbrace(Token),
        TermRparen(Token),
        TermSemi(Token),
        NtAtom(Term),
        NtBlock(Term),
        NtExpr(Term),
        NtId(Var),
        Nt____Block(Term),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, // on Arrow, error
        0, // on Eq, error
        6, // on Fun, goto 5
        7, // on IdT, goto 6
        8, // on Int, goto 7
        9, // on Lbrace, goto 8
        10, // on Let, goto 9
        11, // on Lparen, goto 10
        0, // on Plus, error
        12, // on Quote, goto 11
        0, // on Rbrace, error
        0, // on Rparen, error
        0, // on Semi, error
        // State 1
        0, // on Arrow, error
        0, // on Eq, error
        -12, // on Fun, reduce `Expr = Atom => ActionFn(6);`
        -12, // on IdT, reduce `Expr = Atom => ActionFn(6);`
        -12, // on Int, reduce `Expr = Atom => ActionFn(6);`
        -12, // on Lbrace, reduce `Expr = Atom => ActionFn(6);`
        0, // on Let, error
        -12, // on Lparen, reduce `Expr = Atom => ActionFn(6);`
        -12, // on Plus, reduce `Expr = Atom => ActionFn(6);`
        -12, // on Quote, reduce `Expr = Atom => ActionFn(6);`
        0, // on Rbrace, error
        0, // on Rparen, error
        -12, // on Semi, reduce `Expr = Atom => ActionFn(6);`
        // State 2
        0, // on Arrow, error
        0, // on Eq, error
        0, // on Fun, error
        0, // on IdT, error
        0, // on Int, error
        0, // on Lbrace, error
        0, // on Let, error
        0, // on Lparen, error
        0, // on Plus, error
        0, // on Quote, error
        0, // on Rbrace, error
        0, // on Rparen, error
        13, // on Semi, goto 12
        // State 3
        0, // on Arrow, error
        0, // on Eq, error
        6, // on Fun, goto 5
        7, // on IdT, goto 6
        8, // on Int, goto 7
        9, // on Lbrace, goto 8
        0, // on Let, error
        11, // on Lparen, goto 10
        15, // on Plus, goto 14
        12, // on Quote, goto 11
        0, // on Rbrace, error
        0, // on Rparen, error
        -9, // on Semi, reduce `Block = Expr => ActionFn(3);`
        // State 4
        0, // on Arrow, error
        0, // on Eq, error
        -2, // on Fun, reduce `Atom = Id => ActionFn(8);`
        -2, // on IdT, reduce `Atom = Id => ActionFn(8);`
        -2, // on Int, reduce `Atom = Id => ActionFn(8);`
        -2, // on Lbrace, reduce `Atom = Id => ActionFn(8);`
        0, // on Let, error
        -2, // on Lparen, reduce `Atom = Id => ActionFn(8);`
        -2, // on Plus, reduce `Atom = Id => ActionFn(8);`
        -2, // on Quote, reduce `Atom = Id => ActionFn(8);`
        0, // on Rbrace, error
        0, // on Rparen, error
        -2, // on Semi, reduce `Atom = Id => ActionFn(8);`
        // State 5
        0, // on Arrow, error
        0, // on Eq, error
        0, // on Fun, error
        17, // on IdT, goto 16
        0, // on Int, error
        0, // on Lbrace, error
        0, // on Let, error
        0, // on Lparen, error
        0, // on Plus, error
        0, // on Quote, error
        0, // on Rbrace, error
        0, // on Rparen, error
        0, // on Semi, error
        // State 6
        0, // on Arrow, error
        0, // on Eq, error
        -13, // on Fun, reduce `Id = IdT => ActionFn(13);`
        -13, // on IdT, reduce `Id = IdT => ActionFn(13);`
        -13, // on Int, reduce `Id = IdT => ActionFn(13);`
        -13, // on Lbrace, reduce `Id = IdT => ActionFn(13);`
        0, // on Let, error
        -13, // on Lparen, reduce `Id = IdT => ActionFn(13);`
        -13, // on Plus, reduce `Id = IdT => ActionFn(13);`
        -13, // on Quote, reduce `Id = IdT => ActionFn(13);`
        0, // on Rbrace, error
        0, // on Rparen, error
        -13, // on Semi, reduce `Id = IdT => ActionFn(13);`
        // State 7
        0, // on Arrow, error
        0, // on Eq, error
        -1, // on Fun, reduce `Atom = Int => ActionFn(7);`
        -1, // on IdT, reduce `Atom = Int => ActionFn(7);`
        -1, // on Int, reduce `Atom = Int => ActionFn(7);`
        -1, // on Lbrace, reduce `Atom = Int => ActionFn(7);`
        0, // on Let, error
        -1, // on Lparen, reduce `Atom = Int => ActionFn(7);`
        -1, // on Plus, reduce `Atom = Int => ActionFn(7);`
        -1, // on Quote, reduce `Atom = Int => ActionFn(7);`
        0, // on Rbrace, error
        0, // on Rparen, error
        -1, // on Semi, reduce `Atom = Int => ActionFn(7);`
        // State 8
        0, // on Arrow, error
        0, // on Eq, error
        22, // on Fun, goto 21
        23, // on IdT, goto 22
        24, // on Int, goto 23
        25, // on Lbrace, goto 24
        26, // on Let, goto 25
        27, // on Lparen, goto 26
        0, // on Plus, error
        28, // on Quote, goto 27
        0, // on Rbrace, error
        0, // on Rparen, error
        0, // on Semi, error
        // State 9
        0, // on Arrow, error
        0, // on Eq, error
        0, // on Fun, error
        30, // on IdT, goto 29
        0, // on Int, error
        0, // on Lbrace, error
        0, // on Let, error
        0, // on Lparen, error
        0, // on Plus, error
        0, // on Quote, error
        0, // on Rbrace, error
        0, // on Rparen, error
        0, // on Semi, error
        // State 10
        0, // on Arrow, error
        0, // on Eq, error
        35, // on Fun, goto 34
        36, // on IdT, goto 35
        37, // on Int, goto 36
        38, // on Lbrace, goto 37
        39, // on Let, goto 38
        40, // on Lparen, goto 39
        0, // on Plus, error
        41, // on Quote, goto 40
        0, // on Rbrace, error
        0, // on Rparen, error
        0, // on Semi, error
        // State 11
        0, // on Arrow, error
        0, // on Eq, error
        -6, // on Fun, reduce `Atom = Quote => ActionFn(12);`
        -6, // on IdT, reduce `Atom = Quote => ActionFn(12);`
        -6, // on Int, reduce `Atom = Quote => ActionFn(12);`
        -6, // on Lbrace, reduce `Atom = Quote => ActionFn(12);`
        0, // on Let, error
        -6, // on Lparen, reduce `Atom = Quote => ActionFn(12);`
        -6, // on Plus, reduce `Atom = Quote => ActionFn(12);`
        -6, // on Quote, reduce `Atom = Quote => ActionFn(12);`
        0, // on Rbrace, error
        0, // on Rparen, error
        -6, // on Semi, reduce `Atom = Quote => ActionFn(12);`
        // State 12
        0, // on Arrow, error
        0, // on Eq, error
        6, // on Fun, goto 5
        7, // on IdT, goto 6
        8, // on Int, goto 7
        9, // on Lbrace, goto 8
        0, // on Let, error
        11, // on Lparen, goto 10
        0, // on Plus, error
        12, // on Quote, goto 11
        0, // on Rbrace, error
        0, // on Rparen, error
        0, // on Semi, error
        // State 13
        0, // on Arrow, error
        0, // on Eq, error
        -11, // on Fun, reduce `Expr = Expr, Atom => ActionFn(5);`
        -11, // on IdT, reduce `Expr = Expr, Atom => ActionFn(5);`
        -11, // on Int, reduce `Expr = Expr, Atom => ActionFn(5);`
        -11, // on Lbrace, reduce `Expr = Expr, Atom => ActionFn(5);`
        0, // on Let, error
        -11, // on Lparen, reduce `Expr = Expr, Atom => ActionFn(5);`
        -11, // on Plus, reduce `Expr = Expr, Atom => ActionFn(5);`
        -11, // on Quote, reduce `Expr = Expr, Atom => ActionFn(5);`
        0, // on Rbrace, error
        0, // on Rparen, error
        -11, // on Semi, reduce `Expr = Expr, Atom => ActionFn(5);`
        // State 14
        0, // on Arrow, error
        0, // on Eq, error
        6, // on Fun, goto 5
        7, // on IdT, goto 6
        8, // on Int, goto 7
        9, // on Lbrace, goto 8
        0, // on Let, error
        11, // on Lparen, goto 10
        0, // on Plus, error
        12, // on Quote, goto 11
        0, // on Rbrace, error
        0, // on Rparen, error
        0, // on Semi, error
        // State 15
        44, // on Arrow, goto 43
        0, // on Eq, error
        0, // on Fun, error
        0, // on IdT, error
        0, // on Int, error
        0, // on Lbrace, error
        0, // on Let, error
        0, // on Lparen, error
        0, // on Plus, error
        0, // on Quote, error
        0, // on Rbrace, error
        0, // on Rparen, error
        0, // on Semi, error
        // State 16
        -13, // on Arrow, reduce `Id = IdT => ActionFn(13);`
        0, // on Eq, error
        0, // on Fun, error
        0, // on IdT, error
        0, // on Int, error
        0, // on Lbrace, error
        0, // on Let, error
        0, // on Lparen, error
        0, // on Plus, error
        0, // on Quote, error
        0, // on Rbrace, error
        0, // on Rparen, error
        0, // on Semi, error
        // State 17
        0, // on Arrow, error
        0, // on Eq, error
        -12, // on Fun, reduce `Expr = Atom => ActionFn(6);`
        -12, // on IdT, reduce `Expr = Atom => ActionFn(6);`
        -12, // on Int, reduce `Expr = Atom => ActionFn(6);`
        -12, // on Lbrace, reduce `Expr = Atom => ActionFn(6);`
        0, // on Let, error
        -12, // on Lparen, reduce `Expr = Atom => ActionFn(6);`
        -12, // on Plus, reduce `Expr = Atom => ActionFn(6);`
        -12, // on Quote, reduce `Expr = Atom => ActionFn(6);`
        -12, // on Rbrace, reduce `Expr = Atom => ActionFn(6);`
        0, // on Rparen, error
        -12, // on Semi, reduce `Expr = Atom => ActionFn(6);`
        // State 18
        0, // on Arrow, error
        0, // on Eq, error
        0, // on Fun, error
        0, // on IdT, error
        0, // on Int, error
        0, // on Lbrace, error
        0, // on Let, error
        0, // on Lparen, error
        0, // on Plus, error
        0, // on Quote, error
        45, // on Rbrace, goto 44
        0, // on Rparen, error
        46, // on Semi, goto 45
        // State 19
        0, // on Arrow, error
        0, // on Eq, error
        22, // on Fun, goto 21
        23, // on IdT, goto 22
        24, // on Int, goto 23
        25, // on Lbrace, goto 24
        0, // on Let, error
        27, // on Lparen, goto 26
        48, // on Plus, goto 47
        28, // on Quote, goto 27
        -9, // on Rbrace, reduce `Block = Expr => ActionFn(3);`
        0, // on Rparen, error
        -9, // on Semi, reduce `Block = Expr => ActionFn(3);`
        // State 20
        0, // on Arrow, error
        0, // on Eq, error
        -2, // on Fun, reduce `Atom = Id => ActionFn(8);`
        -2, // on IdT, reduce `Atom = Id => ActionFn(8);`
        -2, // on Int, reduce `Atom = Id => ActionFn(8);`
        -2, // on Lbrace, reduce `Atom = Id => ActionFn(8);`
        0, // on Let, error
        -2, // on Lparen, reduce `Atom = Id => ActionFn(8);`
        -2, // on Plus, reduce `Atom = Id => ActionFn(8);`
        -2, // on Quote, reduce `Atom = Id => ActionFn(8);`
        -2, // on Rbrace, reduce `Atom = Id => ActionFn(8);`
        0, // on Rparen, error
        -2, // on Semi, reduce `Atom = Id => ActionFn(8);`
        // State 21
        0, // on Arrow, error
        0, // on Eq, error
        0, // on Fun, error
        17, // on IdT, goto 16
        0, // on Int, error
        0, // on Lbrace, error
        0, // on Let, error
        0, // on Lparen, error
        0, // on Plus, error
        0, // on Quote, error
        0, // on Rbrace, error
        0, // on Rparen, error
        0, // on Semi, error
        // State 22
        0, // on Arrow, error
        0, // on Eq, error
        -13, // on Fun, reduce `Id = IdT => ActionFn(13);`
        -13, // on IdT, reduce `Id = IdT => ActionFn(13);`
        -13, // on Int, reduce `Id = IdT => ActionFn(13);`
        -13, // on Lbrace, reduce `Id = IdT => ActionFn(13);`
        0, // on Let, error
        -13, // on Lparen, reduce `Id = IdT => ActionFn(13);`
        -13, // on Plus, reduce `Id = IdT => ActionFn(13);`
        -13, // on Quote, reduce `Id = IdT => ActionFn(13);`
        -13, // on Rbrace, reduce `Id = IdT => ActionFn(13);`
        0, // on Rparen, error
        -13, // on Semi, reduce `Id = IdT => ActionFn(13);`
        // State 23
        0, // on Arrow, error
        0, // on Eq, error
        -1, // on Fun, reduce `Atom = Int => ActionFn(7);`
        -1, // on IdT, reduce `Atom = Int => ActionFn(7);`
        -1, // on Int, reduce `Atom = Int => ActionFn(7);`
        -1, // on Lbrace, reduce `Atom = Int => ActionFn(7);`
        0, // on Let, error
        -1, // on Lparen, reduce `Atom = Int => ActionFn(7);`
        -1, // on Plus, reduce `Atom = Int => ActionFn(7);`
        -1, // on Quote, reduce `Atom = Int => ActionFn(7);`
        -1, // on Rbrace, reduce `Atom = Int => ActionFn(7);`
        0, // on Rparen, error
        -1, // on Semi, reduce `Atom = Int => ActionFn(7);`
        // State 24
        0, // on Arrow, error
        0, // on Eq, error
        22, // on Fun, goto 21
        23, // on IdT, goto 22
        24, // on Int, goto 23
        25, // on Lbrace, goto 24
        26, // on Let, goto 25
        27, // on Lparen, goto 26
        0, // on Plus, error
        28, // on Quote, goto 27
        0, // on Rbrace, error
        0, // on Rparen, error
        0, // on Semi, error
        // State 25
        0, // on Arrow, error
        0, // on Eq, error
        0, // on Fun, error
        30, // on IdT, goto 29
        0, // on Int, error
        0, // on Lbrace, error
        0, // on Let, error
        0, // on Lparen, error
        0, // on Plus, error
        0, // on Quote, error
        0, // on Rbrace, error
        0, // on Rparen, error
        0, // on Semi, error
        // State 26
        0, // on Arrow, error
        0, // on Eq, error
        35, // on Fun, goto 34
        36, // on IdT, goto 35
        37, // on Int, goto 36
        38, // on Lbrace, goto 37
        39, // on Let, goto 38
        40, // on Lparen, goto 39
        0, // on Plus, error
        41, // on Quote, goto 40
        0, // on Rbrace, error
        0, // on Rparen, error
        0, // on Semi, error
        // State 27
        0, // on Arrow, error
        0, // on Eq, error
        -6, // on Fun, reduce `Atom = Quote => ActionFn(12);`
        -6, // on IdT, reduce `Atom = Quote => ActionFn(12);`
        -6, // on Int, reduce `Atom = Quote => ActionFn(12);`
        -6, // on Lbrace, reduce `Atom = Quote => ActionFn(12);`
        0, // on Let, error
        -6, // on Lparen, reduce `Atom = Quote => ActionFn(12);`
        -6, // on Plus, reduce `Atom = Quote => ActionFn(12);`
        -6, // on Quote, reduce `Atom = Quote => ActionFn(12);`
        -6, // on Rbrace, reduce `Atom = Quote => ActionFn(12);`
        0, // on Rparen, error
        -6, // on Semi, reduce `Atom = Quote => ActionFn(12);`
        // State 28
        0, // on Arrow, error
        53, // on Eq, goto 52
        0, // on Fun, error
        0, // on IdT, error
        0, // on Int, error
        0, // on Lbrace, error
        0, // on Let, error
        0, // on Lparen, error
        0, // on Plus, error
        0, // on Quote, error
        0, // on Rbrace, error
        0, // on Rparen, error
        0, // on Semi, error
        // State 29
        0, // on Arrow, error
        -13, // on Eq, reduce `Id = IdT => ActionFn(13);`
        0, // on Fun, error
        0, // on IdT, error
        0, // on Int, error
        0, // on Lbrace, error
        0, // on Let, error
        0, // on Lparen, error
        0, // on Plus, error
        0, // on Quote, error
        0, // on Rbrace, error
        0, // on Rparen, error
        0, // on Semi, error
        // State 30
        0, // on Arrow, error
        0, // on Eq, error
        -12, // on Fun, reduce `Expr = Atom => ActionFn(6);`
        -12, // on IdT, reduce `Expr = Atom => ActionFn(6);`
        -12, // on Int, reduce `Expr = Atom => ActionFn(6);`
        -12, // on Lbrace, reduce `Expr = Atom => ActionFn(6);`
        0, // on Let, error
        -12, // on Lparen, reduce `Expr = Atom => ActionFn(6);`
        -12, // on Plus, reduce `Expr = Atom => ActionFn(6);`
        -12, // on Quote, reduce `Expr = Atom => ActionFn(6);`
        0, // on Rbrace, error
        -12, // on Rparen, reduce `Expr = Atom => ActionFn(6);`
        -12, // on Semi, reduce `Expr = Atom => ActionFn(6);`
        // State 31
        0, // on Arrow, error
        0, // on Eq, error
        0, // on Fun, error
        0, // on IdT, error
        0, // on Int, error
        0, // on Lbrace, error
        0, // on Let, error
        0, // on Lparen, error
        0, // on Plus, error
        0, // on Quote, error
        0, // on Rbrace, error
        54, // on Rparen, goto 53
        55, // on Semi, goto 54
        // State 32
        0, // on Arrow, error
        0, // on Eq, error
        35, // on Fun, goto 34
        36, // on IdT, goto 35
        37, // on Int, goto 36
        38, // on Lbrace, goto 37
        0, // on Let, error
        40, // on Lparen, goto 39
        57, // on Plus, goto 56
        41, // on Quote, goto 40
        0, // on Rbrace, error
        -9, // on Rparen, reduce `Block = Expr => ActionFn(3);`
        -9, // on Semi, reduce `Block = Expr => ActionFn(3);`
        // State 33
        0, // on Arrow, error
        0, // on Eq, error
        -2, // on Fun, reduce `Atom = Id => ActionFn(8);`
        -2, // on IdT, reduce `Atom = Id => ActionFn(8);`
        -2, // on Int, reduce `Atom = Id => ActionFn(8);`
        -2, // on Lbrace, reduce `Atom = Id => ActionFn(8);`
        0, // on Let, error
        -2, // on Lparen, reduce `Atom = Id => ActionFn(8);`
        -2, // on Plus, reduce `Atom = Id => ActionFn(8);`
        -2, // on Quote, reduce `Atom = Id => ActionFn(8);`
        0, // on Rbrace, error
        -2, // on Rparen, reduce `Atom = Id => ActionFn(8);`
        -2, // on Semi, reduce `Atom = Id => ActionFn(8);`
        // State 34
        0, // on Arrow, error
        0, // on Eq, error
        0, // on Fun, error
        17, // on IdT, goto 16
        0, // on Int, error
        0, // on Lbrace, error
        0, // on Let, error
        0, // on Lparen, error
        0, // on Plus, error
        0, // on Quote, error
        0, // on Rbrace, error
        0, // on Rparen, error
        0, // on Semi, error
        // State 35
        0, // on Arrow, error
        0, // on Eq, error
        -13, // on Fun, reduce `Id = IdT => ActionFn(13);`
        -13, // on IdT, reduce `Id = IdT => ActionFn(13);`
        -13, // on Int, reduce `Id = IdT => ActionFn(13);`
        -13, // on Lbrace, reduce `Id = IdT => ActionFn(13);`
        0, // on Let, error
        -13, // on Lparen, reduce `Id = IdT => ActionFn(13);`
        -13, // on Plus, reduce `Id = IdT => ActionFn(13);`
        -13, // on Quote, reduce `Id = IdT => ActionFn(13);`
        0, // on Rbrace, error
        -13, // on Rparen, reduce `Id = IdT => ActionFn(13);`
        -13, // on Semi, reduce `Id = IdT => ActionFn(13);`
        // State 36
        0, // on Arrow, error
        0, // on Eq, error
        -1, // on Fun, reduce `Atom = Int => ActionFn(7);`
        -1, // on IdT, reduce `Atom = Int => ActionFn(7);`
        -1, // on Int, reduce `Atom = Int => ActionFn(7);`
        -1, // on Lbrace, reduce `Atom = Int => ActionFn(7);`
        0, // on Let, error
        -1, // on Lparen, reduce `Atom = Int => ActionFn(7);`
        -1, // on Plus, reduce `Atom = Int => ActionFn(7);`
        -1, // on Quote, reduce `Atom = Int => ActionFn(7);`
        0, // on Rbrace, error
        -1, // on Rparen, reduce `Atom = Int => ActionFn(7);`
        -1, // on Semi, reduce `Atom = Int => ActionFn(7);`
        // State 37
        0, // on Arrow, error
        0, // on Eq, error
        22, // on Fun, goto 21
        23, // on IdT, goto 22
        24, // on Int, goto 23
        25, // on Lbrace, goto 24
        26, // on Let, goto 25
        27, // on Lparen, goto 26
        0, // on Plus, error
        28, // on Quote, goto 27
        0, // on Rbrace, error
        0, // on Rparen, error
        0, // on Semi, error
        // State 38
        0, // on Arrow, error
        0, // on Eq, error
        0, // on Fun, error
        30, // on IdT, goto 29
        0, // on Int, error
        0, // on Lbrace, error
        0, // on Let, error
        0, // on Lparen, error
        0, // on Plus, error
        0, // on Quote, error
        0, // on Rbrace, error
        0, // on Rparen, error
        0, // on Semi, error
        // State 39
        0, // on Arrow, error
        0, // on Eq, error
        35, // on Fun, goto 34
        36, // on IdT, goto 35
        37, // on Int, goto 36
        38, // on Lbrace, goto 37
        39, // on Let, goto 38
        40, // on Lparen, goto 39
        0, // on Plus, error
        41, // on Quote, goto 40
        0, // on Rbrace, error
        0, // on Rparen, error
        0, // on Semi, error
        // State 40
        0, // on Arrow, error
        0, // on Eq, error
        -6, // on Fun, reduce `Atom = Quote => ActionFn(12);`
        -6, // on IdT, reduce `Atom = Quote => ActionFn(12);`
        -6, // on Int, reduce `Atom = Quote => ActionFn(12);`
        -6, // on Lbrace, reduce `Atom = Quote => ActionFn(12);`
        0, // on Let, error
        -6, // on Lparen, reduce `Atom = Quote => ActionFn(12);`
        -6, // on Plus, reduce `Atom = Quote => ActionFn(12);`
        -6, // on Quote, reduce `Atom = Quote => ActionFn(12);`
        0, // on Rbrace, error
        -6, // on Rparen, reduce `Atom = Quote => ActionFn(12);`
        -6, // on Semi, reduce `Atom = Quote => ActionFn(12);`
        // State 41
        0, // on Arrow, error
        0, // on Eq, error
        6, // on Fun, goto 5
        7, // on IdT, goto 6
        8, // on Int, goto 7
        9, // on Lbrace, goto 8
        0, // on Let, error
        11, // on Lparen, goto 10
        15, // on Plus, goto 14
        12, // on Quote, goto 11
        0, // on Rbrace, error
        0, // on Rparen, error
        -8, // on Semi, reduce `Block = Block, Semi, Expr => ActionFn(2);`
        // State 42
        0, // on Arrow, error
        0, // on Eq, error
        -10, // on Fun, reduce `Expr = Expr, Plus, Atom => ActionFn(4);`
        -10, // on IdT, reduce `Expr = Expr, Plus, Atom => ActionFn(4);`
        -10, // on Int, reduce `Expr = Expr, Plus, Atom => ActionFn(4);`
        -10, // on Lbrace, reduce `Expr = Expr, Plus, Atom => ActionFn(4);`
        0, // on Let, error
        -10, // on Lparen, reduce `Expr = Expr, Plus, Atom => ActionFn(4);`
        -10, // on Plus, reduce `Expr = Expr, Plus, Atom => ActionFn(4);`
        -10, // on Quote, reduce `Expr = Expr, Plus, Atom => ActionFn(4);`
        0, // on Rbrace, error
        0, // on Rparen, error
        -10, // on Semi, reduce `Expr = Expr, Plus, Atom => ActionFn(4);`
        // State 43
        0, // on Arrow, error
        0, // on Eq, error
        0, // on Fun, error
        0, // on IdT, error
        0, // on Int, error
        62, // on Lbrace, goto 61
        0, // on Let, error
        0, // on Lparen, error
        0, // on Plus, error
        0, // on Quote, error
        0, // on Rbrace, error
        0, // on Rparen, error
        0, // on Semi, error
        // State 44
        0, // on Arrow, error
        0, // on Eq, error
        -4, // on Fun, reduce `Atom = Lbrace, Block, Rbrace => ActionFn(10);`
        -4, // on IdT, reduce `Atom = Lbrace, Block, Rbrace => ActionFn(10);`
        -4, // on Int, reduce `Atom = Lbrace, Block, Rbrace => ActionFn(10);`
        -4, // on Lbrace, reduce `Atom = Lbrace, Block, Rbrace => ActionFn(10);`
        0, // on Let, error
        -4, // on Lparen, reduce `Atom = Lbrace, Block, Rbrace => ActionFn(10);`
        -4, // on Plus, reduce `Atom = Lbrace, Block, Rbrace => ActionFn(10);`
        -4, // on Quote, reduce `Atom = Lbrace, Block, Rbrace => ActionFn(10);`
        0, // on Rbrace, error
        0, // on Rparen, error
        -4, // on Semi, reduce `Atom = Lbrace, Block, Rbrace => ActionFn(10);`
        // State 45
        0, // on Arrow, error
        0, // on Eq, error
        22, // on Fun, goto 21
        23, // on IdT, goto 22
        24, // on Int, goto 23
        25, // on Lbrace, goto 24
        0, // on Let, error
        27, // on Lparen, goto 26
        0, // on Plus, error
        28, // on Quote, goto 27
        0, // on Rbrace, error
        0, // on Rparen, error
        0, // on Semi, error
        // State 46
        0, // on Arrow, error
        0, // on Eq, error
        -11, // on Fun, reduce `Expr = Expr, Atom => ActionFn(5);`
        -11, // on IdT, reduce `Expr = Expr, Atom => ActionFn(5);`
        -11, // on Int, reduce `Expr = Expr, Atom => ActionFn(5);`
        -11, // on Lbrace, reduce `Expr = Expr, Atom => ActionFn(5);`
        0, // on Let, error
        -11, // on Lparen, reduce `Expr = Expr, Atom => ActionFn(5);`
        -11, // on Plus, reduce `Expr = Expr, Atom => ActionFn(5);`
        -11, // on Quote, reduce `Expr = Expr, Atom => ActionFn(5);`
        -11, // on Rbrace, reduce `Expr = Expr, Atom => ActionFn(5);`
        0, // on Rparen, error
        -11, // on Semi, reduce `Expr = Expr, Atom => ActionFn(5);`
        // State 47
        0, // on Arrow, error
        0, // on Eq, error
        22, // on Fun, goto 21
        23, // on IdT, goto 22
        24, // on Int, goto 23
        25, // on Lbrace, goto 24
        0, // on Let, error
        27, // on Lparen, goto 26
        0, // on Plus, error
        28, // on Quote, goto 27
        0, // on Rbrace, error
        0, // on Rparen, error
        0, // on Semi, error
        // State 48
        65, // on Arrow, goto 64
        0, // on Eq, error
        0, // on Fun, error
        0, // on IdT, error
        0, // on Int, error
        0, // on Lbrace, error
        0, // on Let, error
        0, // on Lparen, error
        0, // on Plus, error
        0, // on Quote, error
        0, // on Rbrace, error
        0, // on Rparen, error
        0, // on Semi, error
        // State 49
        0, // on Arrow, error
        0, // on Eq, error
        0, // on Fun, error
        0, // on IdT, error
        0, // on Int, error
        0, // on Lbrace, error
        0, // on Let, error
        0, // on Lparen, error
        0, // on Plus, error
        0, // on Quote, error
        66, // on Rbrace, goto 65
        0, // on Rparen, error
        46, // on Semi, goto 45
        // State 50
        0, // on Arrow, error
        67, // on Eq, goto 66
        0, // on Fun, error
        0, // on IdT, error
        0, // on Int, error
        0, // on Lbrace, error
        0, // on Let, error
        0, // on Lparen, error
        0, // on Plus, error
        0, // on Quote, error
        0, // on Rbrace, error
        0, // on Rparen, error
        0, // on Semi, error
        // State 51
        0, // on Arrow, error
        0, // on Eq, error
        0, // on Fun, error
        0, // on IdT, error
        0, // on Int, error
        0, // on Lbrace, error
        0, // on Let, error
        0, // on Lparen, error
        0, // on Plus, error
        0, // on Quote, error
        0, // on Rbrace, error
        68, // on Rparen, goto 67
        55, // on Semi, goto 54
        // State 52
        0, // on Arrow, error
        0, // on Eq, error
        72, // on Fun, goto 71
        73, // on IdT, goto 72
        74, // on Int, goto 73
        75, // on Lbrace, goto 74
        0, // on Let, error
        76, // on Lparen, goto 75
        0, // on Plus, error
        77, // on Quote, goto 76
        0, // on Rbrace, error
        0, // on Rparen, error
        0, // on Semi, error
        // State 53
        0, // on Arrow, error
        0, // on Eq, error
        -3, // on Fun, reduce `Atom = Lparen, Block, Rparen => ActionFn(9);`
        -3, // on IdT, reduce `Atom = Lparen, Block, Rparen => ActionFn(9);`
        -3, // on Int, reduce `Atom = Lparen, Block, Rparen => ActionFn(9);`
        -3, // on Lbrace, reduce `Atom = Lparen, Block, Rparen => ActionFn(9);`
        0, // on Let, error
        -3, // on Lparen, reduce `Atom = Lparen, Block, Rparen => ActionFn(9);`
        -3, // on Plus, reduce `Atom = Lparen, Block, Rparen => ActionFn(9);`
        -3, // on Quote, reduce `Atom = Lparen, Block, Rparen => ActionFn(9);`
        0, // on Rbrace, error
        0, // on Rparen, error
        -3, // on Semi, reduce `Atom = Lparen, Block, Rparen => ActionFn(9);`
        // State 54
        0, // on Arrow, error
        0, // on Eq, error
        35, // on Fun, goto 34
        36, // on IdT, goto 35
        37, // on Int, goto 36
        38, // on Lbrace, goto 37
        0, // on Let, error
        40, // on Lparen, goto 39
        0, // on Plus, error
        41, // on Quote, goto 40
        0, // on Rbrace, error
        0, // on Rparen, error
        0, // on Semi, error
        // State 55
        0, // on Arrow, error
        0, // on Eq, error
        -11, // on Fun, reduce `Expr = Expr, Atom => ActionFn(5);`
        -11, // on IdT, reduce `Expr = Expr, Atom => ActionFn(5);`
        -11, // on Int, reduce `Expr = Expr, Atom => ActionFn(5);`
        -11, // on Lbrace, reduce `Expr = Expr, Atom => ActionFn(5);`
        0, // on Let, error
        -11, // on Lparen, reduce `Expr = Expr, Atom => ActionFn(5);`
        -11, // on Plus, reduce `Expr = Expr, Atom => ActionFn(5);`
        -11, // on Quote, reduce `Expr = Expr, Atom => ActionFn(5);`
        0, // on Rbrace, error
        -11, // on Rparen, reduce `Expr = Expr, Atom => ActionFn(5);`
        -11, // on Semi, reduce `Expr = Expr, Atom => ActionFn(5);`
        // State 56
        0, // on Arrow, error
        0, // on Eq, error
        35, // on Fun, goto 34
        36, // on IdT, goto 35
        37, // on Int, goto 36
        38, // on Lbrace, goto 37
        0, // on Let, error
        40, // on Lparen, goto 39
        0, // on Plus, error
        41, // on Quote, goto 40
        0, // on Rbrace, error
        0, // on Rparen, error
        0, // on Semi, error
        // State 57
        80, // on Arrow, goto 79
        0, // on Eq, error
        0, // on Fun, error
        0, // on IdT, error
        0, // on Int, error
        0, // on Lbrace, error
        0, // on Let, error
        0, // on Lparen, error
        0, // on Plus, error
        0, // on Quote, error
        0, // on Rbrace, error
        0, // on Rparen, error
        0, // on Semi, error
        // State 58
        0, // on Arrow, error
        0, // on Eq, error
        0, // on Fun, error
        0, // on IdT, error
        0, // on Int, error
        0, // on Lbrace, error
        0, // on Let, error
        0, // on Lparen, error
        0, // on Plus, error
        0, // on Quote, error
        81, // on Rbrace, goto 80
        0, // on Rparen, error
        46, // on Semi, goto 45
        // State 59
        0, // on Arrow, error
        82, // on Eq, goto 81
        0, // on Fun, error
        0, // on IdT, error
        0, // on Int, error
        0, // on Lbrace, error
        0, // on Let, error
        0, // on Lparen, error
        0, // on Plus, error
        0, // on Quote, error
        0, // on Rbrace, error
        0, // on Rparen, error
        0, // on Semi, error
        // State 60
        0, // on Arrow, error
        0, // on Eq, error
        0, // on Fun, error
        0, // on IdT, error
        0, // on Int, error
        0, // on Lbrace, error
        0, // on Let, error
        0, // on Lparen, error
        0, // on Plus, error
        0, // on Quote, error
        0, // on Rbrace, error
        83, // on Rparen, goto 82
        55, // on Semi, goto 54
        // State 61
        0, // on Arrow, error
        0, // on Eq, error
        87, // on Fun, goto 86
        88, // on IdT, goto 87
        89, // on Int, goto 88
        90, // on Lbrace, goto 89
        0, // on Let, error
        91, // on Lparen, goto 90
        0, // on Plus, error
        92, // on Quote, goto 91
        0, // on Rbrace, error
        0, // on Rparen, error
        0, // on Semi, error
        // State 62
        0, // on Arrow, error
        0, // on Eq, error
        22, // on Fun, goto 21
        23, // on IdT, goto 22
        24, // on Int, goto 23
        25, // on Lbrace, goto 24
        0, // on Let, error
        27, // on Lparen, goto 26
        48, // on Plus, goto 47
        28, // on Quote, goto 27
        -8, // on Rbrace, reduce `Block = Block, Semi, Expr => ActionFn(2);`
        0, // on Rparen, error
        -8, // on Semi, reduce `Block = Block, Semi, Expr => ActionFn(2);`
        // State 63
        0, // on Arrow, error
        0, // on Eq, error
        -10, // on Fun, reduce `Expr = Expr, Plus, Atom => ActionFn(4);`
        -10, // on IdT, reduce `Expr = Expr, Plus, Atom => ActionFn(4);`
        -10, // on Int, reduce `Expr = Expr, Plus, Atom => ActionFn(4);`
        -10, // on Lbrace, reduce `Expr = Expr, Plus, Atom => ActionFn(4);`
        0, // on Let, error
        -10, // on Lparen, reduce `Expr = Expr, Plus, Atom => ActionFn(4);`
        -10, // on Plus, reduce `Expr = Expr, Plus, Atom => ActionFn(4);`
        -10, // on Quote, reduce `Expr = Expr, Plus, Atom => ActionFn(4);`
        -10, // on Rbrace, reduce `Expr = Expr, Plus, Atom => ActionFn(4);`
        0, // on Rparen, error
        -10, // on Semi, reduce `Expr = Expr, Plus, Atom => ActionFn(4);`
        // State 64
        0, // on Arrow, error
        0, // on Eq, error
        0, // on Fun, error
        0, // on IdT, error
        0, // on Int, error
        93, // on Lbrace, goto 92
        0, // on Let, error
        0, // on Lparen, error
        0, // on Plus, error
        0, // on Quote, error
        0, // on Rbrace, error
        0, // on Rparen, error
        0, // on Semi, error
        // State 65
        0, // on Arrow, error
        0, // on Eq, error
        -4, // on Fun, reduce `Atom = Lbrace, Block, Rbrace => ActionFn(10);`
        -4, // on IdT, reduce `Atom = Lbrace, Block, Rbrace => ActionFn(10);`
        -4, // on Int, reduce `Atom = Lbrace, Block, Rbrace => ActionFn(10);`
        -4, // on Lbrace, reduce `Atom = Lbrace, Block, Rbrace => ActionFn(10);`
        0, // on Let, error
        -4, // on Lparen, reduce `Atom = Lbrace, Block, Rbrace => ActionFn(10);`
        -4, // on Plus, reduce `Atom = Lbrace, Block, Rbrace => ActionFn(10);`
        -4, // on Quote, reduce `Atom = Lbrace, Block, Rbrace => ActionFn(10);`
        -4, // on Rbrace, reduce `Atom = Lbrace, Block, Rbrace => ActionFn(10);`
        0, // on Rparen, error
        -4, // on Semi, reduce `Atom = Lbrace, Block, Rbrace => ActionFn(10);`
        // State 66
        0, // on Arrow, error
        0, // on Eq, error
        72, // on Fun, goto 71
        73, // on IdT, goto 72
        74, // on Int, goto 73
        75, // on Lbrace, goto 74
        0, // on Let, error
        76, // on Lparen, goto 75
        0, // on Plus, error
        77, // on Quote, goto 76
        0, // on Rbrace, error
        0, // on Rparen, error
        0, // on Semi, error
        // State 67
        0, // on Arrow, error
        0, // on Eq, error
        -3, // on Fun, reduce `Atom = Lparen, Block, Rparen => ActionFn(9);`
        -3, // on IdT, reduce `Atom = Lparen, Block, Rparen => ActionFn(9);`
        -3, // on Int, reduce `Atom = Lparen, Block, Rparen => ActionFn(9);`
        -3, // on Lbrace, reduce `Atom = Lparen, Block, Rparen => ActionFn(9);`
        0, // on Let, error
        -3, // on Lparen, reduce `Atom = Lparen, Block, Rparen => ActionFn(9);`
        -3, // on Plus, reduce `Atom = Lparen, Block, Rparen => ActionFn(9);`
        -3, // on Quote, reduce `Atom = Lparen, Block, Rparen => ActionFn(9);`
        -3, // on Rbrace, reduce `Atom = Lparen, Block, Rparen => ActionFn(9);`
        0, // on Rparen, error
        -3, // on Semi, reduce `Atom = Lparen, Block, Rparen => ActionFn(9);`
        // State 68
        0, // on Arrow, error
        0, // on Eq, error
        -12, // on Fun, reduce `Expr = Atom => ActionFn(6);`
        -12, // on IdT, reduce `Expr = Atom => ActionFn(6);`
        -12, // on Int, reduce `Expr = Atom => ActionFn(6);`
        -12, // on Lbrace, reduce `Expr = Atom => ActionFn(6);`
        0, // on Let, error
        -12, // on Lparen, reduce `Expr = Atom => ActionFn(6);`
        -12, // on Plus, reduce `Expr = Atom => ActionFn(6);`
        -12, // on Quote, reduce `Expr = Atom => ActionFn(6);`
        0, // on Rbrace, error
        0, // on Rparen, error
        -12, // on Semi, reduce `Expr = Atom => ActionFn(6);`
        // State 69
        0, // on Arrow, error
        0, // on Eq, error
        72, // on Fun, goto 71
        73, // on IdT, goto 72
        74, // on Int, goto 73
        75, // on Lbrace, goto 74
        0, // on Let, error
        76, // on Lparen, goto 75
        96, // on Plus, goto 95
        77, // on Quote, goto 76
        0, // on Rbrace, error
        0, // on Rparen, error
        97, // on Semi, goto 96
        // State 70
        0, // on Arrow, error
        0, // on Eq, error
        -2, // on Fun, reduce `Atom = Id => ActionFn(8);`
        -2, // on IdT, reduce `Atom = Id => ActionFn(8);`
        -2, // on Int, reduce `Atom = Id => ActionFn(8);`
        -2, // on Lbrace, reduce `Atom = Id => ActionFn(8);`
        0, // on Let, error
        -2, // on Lparen, reduce `Atom = Id => ActionFn(8);`
        -2, // on Plus, reduce `Atom = Id => ActionFn(8);`
        -2, // on Quote, reduce `Atom = Id => ActionFn(8);`
        0, // on Rbrace, error
        0, // on Rparen, error
        -2, // on Semi, reduce `Atom = Id => ActionFn(8);`
        // State 71
        0, // on Arrow, error
        0, // on Eq, error
        0, // on Fun, error
        17, // on IdT, goto 16
        0, // on Int, error
        0, // on Lbrace, error
        0, // on Let, error
        0, // on Lparen, error
        0, // on Plus, error
        0, // on Quote, error
        0, // on Rbrace, error
        0, // on Rparen, error
        0, // on Semi, error
        // State 72
        0, // on Arrow, error
        0, // on Eq, error
        -13, // on Fun, reduce `Id = IdT => ActionFn(13);`
        -13, // on IdT, reduce `Id = IdT => ActionFn(13);`
        -13, // on Int, reduce `Id = IdT => ActionFn(13);`
        -13, // on Lbrace, reduce `Id = IdT => ActionFn(13);`
        0, // on Let, error
        -13, // on Lparen, reduce `Id = IdT => ActionFn(13);`
        -13, // on Plus, reduce `Id = IdT => ActionFn(13);`
        -13, // on Quote, reduce `Id = IdT => ActionFn(13);`
        0, // on Rbrace, error
        0, // on Rparen, error
        -13, // on Semi, reduce `Id = IdT => ActionFn(13);`
        // State 73
        0, // on Arrow, error
        0, // on Eq, error
        -1, // on Fun, reduce `Atom = Int => ActionFn(7);`
        -1, // on IdT, reduce `Atom = Int => ActionFn(7);`
        -1, // on Int, reduce `Atom = Int => ActionFn(7);`
        -1, // on Lbrace, reduce `Atom = Int => ActionFn(7);`
        0, // on Let, error
        -1, // on Lparen, reduce `Atom = Int => ActionFn(7);`
        -1, // on Plus, reduce `Atom = Int => ActionFn(7);`
        -1, // on Quote, reduce `Atom = Int => ActionFn(7);`
        0, // on Rbrace, error
        0, // on Rparen, error
        -1, // on Semi, reduce `Atom = Int => ActionFn(7);`
        // State 74
        0, // on Arrow, error
        0, // on Eq, error
        22, // on Fun, goto 21
        23, // on IdT, goto 22
        24, // on Int, goto 23
        25, // on Lbrace, goto 24
        26, // on Let, goto 25
        27, // on Lparen, goto 26
        0, // on Plus, error
        28, // on Quote, goto 27
        0, // on Rbrace, error
        0, // on Rparen, error
        0, // on Semi, error
        // State 75
        0, // on Arrow, error
        0, // on Eq, error
        35, // on Fun, goto 34
        36, // on IdT, goto 35
        37, // on Int, goto 36
        38, // on Lbrace, goto 37
        39, // on Let, goto 38
        40, // on Lparen, goto 39
        0, // on Plus, error
        41, // on Quote, goto 40
        0, // on Rbrace, error
        0, // on Rparen, error
        0, // on Semi, error
        // State 76
        0, // on Arrow, error
        0, // on Eq, error
        -6, // on Fun, reduce `Atom = Quote => ActionFn(12);`
        -6, // on IdT, reduce `Atom = Quote => ActionFn(12);`
        -6, // on Int, reduce `Atom = Quote => ActionFn(12);`
        -6, // on Lbrace, reduce `Atom = Quote => ActionFn(12);`
        0, // on Let, error
        -6, // on Lparen, reduce `Atom = Quote => ActionFn(12);`
        -6, // on Plus, reduce `Atom = Quote => ActionFn(12);`
        -6, // on Quote, reduce `Atom = Quote => ActionFn(12);`
        0, // on Rbrace, error
        0, // on Rparen, error
        -6, // on Semi, reduce `Atom = Quote => ActionFn(12);`
        // State 77
        0, // on Arrow, error
        0, // on Eq, error
        35, // on Fun, goto 34
        36, // on IdT, goto 35
        37, // on Int, goto 36
        38, // on Lbrace, goto 37
        0, // on Let, error
        40, // on Lparen, goto 39
        57, // on Plus, goto 56
        41, // on Quote, goto 40
        0, // on Rbrace, error
        -8, // on Rparen, reduce `Block = Block, Semi, Expr => ActionFn(2);`
        -8, // on Semi, reduce `Block = Block, Semi, Expr => ActionFn(2);`
        // State 78
        0, // on Arrow, error
        0, // on Eq, error
        -10, // on Fun, reduce `Expr = Expr, Plus, Atom => ActionFn(4);`
        -10, // on IdT, reduce `Expr = Expr, Plus, Atom => ActionFn(4);`
        -10, // on Int, reduce `Expr = Expr, Plus, Atom => ActionFn(4);`
        -10, // on Lbrace, reduce `Expr = Expr, Plus, Atom => ActionFn(4);`
        0, // on Let, error
        -10, // on Lparen, reduce `Expr = Expr, Plus, Atom => ActionFn(4);`
        -10, // on Plus, reduce `Expr = Expr, Plus, Atom => ActionFn(4);`
        -10, // on Quote, reduce `Expr = Expr, Plus, Atom => ActionFn(4);`
        0, // on Rbrace, error
        -10, // on Rparen, reduce `Expr = Expr, Plus, Atom => ActionFn(4);`
        -10, // on Semi, reduce `Expr = Expr, Plus, Atom => ActionFn(4);`
        // State 79
        0, // on Arrow, error
        0, // on Eq, error
        0, // on Fun, error
        0, // on IdT, error
        0, // on Int, error
        101, // on Lbrace, goto 100
        0, // on Let, error
        0, // on Lparen, error
        0, // on Plus, error
        0, // on Quote, error
        0, // on Rbrace, error
        0, // on Rparen, error
        0, // on Semi, error
        // State 80
        0, // on Arrow, error
        0, // on Eq, error
        -4, // on Fun, reduce `Atom = Lbrace, Block, Rbrace => ActionFn(10);`
        -4, // on IdT, reduce `Atom = Lbrace, Block, Rbrace => ActionFn(10);`
        -4, // on Int, reduce `Atom = Lbrace, Block, Rbrace => ActionFn(10);`
        -4, // on Lbrace, reduce `Atom = Lbrace, Block, Rbrace => ActionFn(10);`
        0, // on Let, error
        -4, // on Lparen, reduce `Atom = Lbrace, Block, Rbrace => ActionFn(10);`
        -4, // on Plus, reduce `Atom = Lbrace, Block, Rbrace => ActionFn(10);`
        -4, // on Quote, reduce `Atom = Lbrace, Block, Rbrace => ActionFn(10);`
        0, // on Rbrace, error
        -4, // on Rparen, reduce `Atom = Lbrace, Block, Rbrace => ActionFn(10);`
        -4, // on Semi, reduce `Atom = Lbrace, Block, Rbrace => ActionFn(10);`
        // State 81
        0, // on Arrow, error
        0, // on Eq, error
        72, // on Fun, goto 71
        73, // on IdT, goto 72
        74, // on Int, goto 73
        75, // on Lbrace, goto 74
        0, // on Let, error
        76, // on Lparen, goto 75
        0, // on Plus, error
        77, // on Quote, goto 76
        0, // on Rbrace, error
        0, // on Rparen, error
        0, // on Semi, error
        // State 82
        0, // on Arrow, error
        0, // on Eq, error
        -3, // on Fun, reduce `Atom = Lparen, Block, Rparen => ActionFn(9);`
        -3, // on IdT, reduce `Atom = Lparen, Block, Rparen => ActionFn(9);`
        -3, // on Int, reduce `Atom = Lparen, Block, Rparen => ActionFn(9);`
        -3, // on Lbrace, reduce `Atom = Lparen, Block, Rparen => ActionFn(9);`
        0, // on Let, error
        -3, // on Lparen, reduce `Atom = Lparen, Block, Rparen => ActionFn(9);`
        -3, // on Plus, reduce `Atom = Lparen, Block, Rparen => ActionFn(9);`
        -3, // on Quote, reduce `Atom = Lparen, Block, Rparen => ActionFn(9);`
        0, // on Rbrace, error
        -3, // on Rparen, reduce `Atom = Lparen, Block, Rparen => ActionFn(9);`
        -3, // on Semi, reduce `Atom = Lparen, Block, Rparen => ActionFn(9);`
        // State 83
        0, // on Arrow, error
        0, // on Eq, error
        -12, // on Fun, reduce `Expr = Atom => ActionFn(6);`
        -12, // on IdT, reduce `Expr = Atom => ActionFn(6);`
        -12, // on Int, reduce `Expr = Atom => ActionFn(6);`
        -12, // on Lbrace, reduce `Expr = Atom => ActionFn(6);`
        0, // on Let, error
        -12, // on Lparen, reduce `Expr = Atom => ActionFn(6);`
        -12, // on Plus, reduce `Expr = Atom => ActionFn(6);`
        -12, // on Quote, reduce `Expr = Atom => ActionFn(6);`
        -12, // on Rbrace, reduce `Expr = Atom => ActionFn(6);`
        0, // on Rparen, error
        0, // on Semi, error
        // State 84
        0, // on Arrow, error
        0, // on Eq, error
        87, // on Fun, goto 86
        88, // on IdT, goto 87
        89, // on Int, goto 88
        90, // on Lbrace, goto 89
        0, // on Let, error
        91, // on Lparen, goto 90
        104, // on Plus, goto 103
        92, // on Quote, goto 91
        105, // on Rbrace, goto 104
        0, // on Rparen, error
        0, // on Semi, error
        // State 85
        0, // on Arrow, error
        0, // on Eq, error
        -2, // on Fun, reduce `Atom = Id => ActionFn(8);`
        -2, // on IdT, reduce `Atom = Id => ActionFn(8);`
        -2, // on Int, reduce `Atom = Id => ActionFn(8);`
        -2, // on Lbrace, reduce `Atom = Id => ActionFn(8);`
        0, // on Let, error
        -2, // on Lparen, reduce `Atom = Id => ActionFn(8);`
        -2, // on Plus, reduce `Atom = Id => ActionFn(8);`
        -2, // on Quote, reduce `Atom = Id => ActionFn(8);`
        -2, // on Rbrace, reduce `Atom = Id => ActionFn(8);`
        0, // on Rparen, error
        0, // on Semi, error
        // State 86
        0, // on Arrow, error
        0, // on Eq, error
        0, // on Fun, error
        17, // on IdT, goto 16
        0, // on Int, error
        0, // on Lbrace, error
        0, // on Let, error
        0, // on Lparen, error
        0, // on Plus, error
        0, // on Quote, error
        0, // on Rbrace, error
        0, // on Rparen, error
        0, // on Semi, error
        // State 87
        0, // on Arrow, error
        0, // on Eq, error
        -13, // on Fun, reduce `Id = IdT => ActionFn(13);`
        -13, // on IdT, reduce `Id = IdT => ActionFn(13);`
        -13, // on Int, reduce `Id = IdT => ActionFn(13);`
        -13, // on Lbrace, reduce `Id = IdT => ActionFn(13);`
        0, // on Let, error
        -13, // on Lparen, reduce `Id = IdT => ActionFn(13);`
        -13, // on Plus, reduce `Id = IdT => ActionFn(13);`
        -13, // on Quote, reduce `Id = IdT => ActionFn(13);`
        -13, // on Rbrace, reduce `Id = IdT => ActionFn(13);`
        0, // on Rparen, error
        0, // on Semi, error
        // State 88
        0, // on Arrow, error
        0, // on Eq, error
        -1, // on Fun, reduce `Atom = Int => ActionFn(7);`
        -1, // on IdT, reduce `Atom = Int => ActionFn(7);`
        -1, // on Int, reduce `Atom = Int => ActionFn(7);`
        -1, // on Lbrace, reduce `Atom = Int => ActionFn(7);`
        0, // on Let, error
        -1, // on Lparen, reduce `Atom = Int => ActionFn(7);`
        -1, // on Plus, reduce `Atom = Int => ActionFn(7);`
        -1, // on Quote, reduce `Atom = Int => ActionFn(7);`
        -1, // on Rbrace, reduce `Atom = Int => ActionFn(7);`
        0, // on Rparen, error
        0, // on Semi, error
        // State 89
        0, // on Arrow, error
        0, // on Eq, error
        22, // on Fun, goto 21
        23, // on IdT, goto 22
        24, // on Int, goto 23
        25, // on Lbrace, goto 24
        26, // on Let, goto 25
        27, // on Lparen, goto 26
        0, // on Plus, error
        28, // on Quote, goto 27
        0, // on Rbrace, error
        0, // on Rparen, error
        0, // on Semi, error
        // State 90
        0, // on Arrow, error
        0, // on Eq, error
        35, // on Fun, goto 34
        36, // on IdT, goto 35
        37, // on Int, goto 36
        38, // on Lbrace, goto 37
        39, // on Let, goto 38
        40, // on Lparen, goto 39
        0, // on Plus, error
        41, // on Quote, goto 40
        0, // on Rbrace, error
        0, // on Rparen, error
        0, // on Semi, error
        // State 91
        0, // on Arrow, error
        0, // on Eq, error
        -6, // on Fun, reduce `Atom = Quote => ActionFn(12);`
        -6, // on IdT, reduce `Atom = Quote => ActionFn(12);`
        -6, // on Int, reduce `Atom = Quote => ActionFn(12);`
        -6, // on Lbrace, reduce `Atom = Quote => ActionFn(12);`
        0, // on Let, error
        -6, // on Lparen, reduce `Atom = Quote => ActionFn(12);`
        -6, // on Plus, reduce `Atom = Quote => ActionFn(12);`
        -6, // on Quote, reduce `Atom = Quote => ActionFn(12);`
        -6, // on Rbrace, reduce `Atom = Quote => ActionFn(12);`
        0, // on Rparen, error
        0, // on Semi, error
        // State 92
        0, // on Arrow, error
        0, // on Eq, error
        87, // on Fun, goto 86
        88, // on IdT, goto 87
        89, // on Int, goto 88
        90, // on Lbrace, goto 89
        0, // on Let, error
        91, // on Lparen, goto 90
        0, // on Plus, error
        92, // on Quote, goto 91
        0, // on Rbrace, error
        0, // on Rparen, error
        0, // on Semi, error
        // State 93
        0, // on Arrow, error
        0, // on Eq, error
        72, // on Fun, goto 71
        73, // on IdT, goto 72
        74, // on Int, goto 73
        75, // on Lbrace, goto 74
        0, // on Let, error
        76, // on Lparen, goto 75
        96, // on Plus, goto 95
        77, // on Quote, goto 76
        0, // on Rbrace, error
        0, // on Rparen, error
        110, // on Semi, goto 109
        // State 94
        0, // on Arrow, error
        0, // on Eq, error
        -11, // on Fun, reduce `Expr = Expr, Atom => ActionFn(5);`
        -11, // on IdT, reduce `Expr = Expr, Atom => ActionFn(5);`
        -11, // on Int, reduce `Expr = Expr, Atom => ActionFn(5);`
        -11, // on Lbrace, reduce `Expr = Expr, Atom => ActionFn(5);`
        0, // on Let, error
        -11, // on Lparen, reduce `Expr = Expr, Atom => ActionFn(5);`
        -11, // on Plus, reduce `Expr = Expr, Atom => ActionFn(5);`
        -11, // on Quote, reduce `Expr = Expr, Atom => ActionFn(5);`
        0, // on Rbrace, error
        0, // on Rparen, error
        -11, // on Semi, reduce `Expr = Expr, Atom => ActionFn(5);`
        // State 95
        0, // on Arrow, error
        0, // on Eq, error
        72, // on Fun, goto 71
        73, // on IdT, goto 72
        74, // on Int, goto 73
        75, // on Lbrace, goto 74
        0, // on Let, error
        76, // on Lparen, goto 75
        0, // on Plus, error
        77, // on Quote, goto 76
        0, // on Rbrace, error
        0, // on Rparen, error
        0, // on Semi, error
        // State 96
        0, // on Arrow, error
        0, // on Eq, error
        6, // on Fun, goto 5
        7, // on IdT, goto 6
        8, // on Int, goto 7
        9, // on Lbrace, goto 8
        0, // on Let, error
        11, // on Lparen, goto 10
        0, // on Plus, error
        12, // on Quote, goto 11
        0, // on Rbrace, error
        0, // on Rparen, error
        0, // on Semi, error
        // State 97
        113, // on Arrow, goto 112
        0, // on Eq, error
        0, // on Fun, error
        0, // on IdT, error
        0, // on Int, error
        0, // on Lbrace, error
        0, // on Let, error
        0, // on Lparen, error
        0, // on Plus, error
        0, // on Quote, error
        0, // on Rbrace, error
        0, // on Rparen, error
        0, // on Semi, error
        // State 98
        0, // on Arrow, error
        0, // on Eq, error
        0, // on Fun, error
        0, // on IdT, error
        0, // on Int, error
        0, // on Lbrace, error
        0, // on Let, error
        0, // on Lparen, error
        0, // on Plus, error
        0, // on Quote, error
        114, // on Rbrace, goto 113
        0, // on Rparen, error
        46, // on Semi, goto 45
        // State 99
        0, // on Arrow, error
        0, // on Eq, error
        0, // on Fun, error
        0, // on IdT, error
        0, // on Int, error
        0, // on Lbrace, error
        0, // on Let, error
        0, // on Lparen, error
        0, // on Plus, error
        0, // on Quote, error
        0, // on Rbrace, error
        115, // on Rparen, goto 114
        55, // on Semi, goto 54
        // State 100
        0, // on Arrow, error
        0, // on Eq, error
        87, // on Fun, goto 86
        88, // on IdT, goto 87
        89, // on Int, goto 88
        90, // on Lbrace, goto 89
        0, // on Let, error
        91, // on Lparen, goto 90
        0, // on Plus, error
        92, // on Quote, goto 91
        0, // on Rbrace, error
        0, // on Rparen, error
        0, // on Semi, error
        // State 101
        0, // on Arrow, error
        0, // on Eq, error
        72, // on Fun, goto 71
        73, // on IdT, goto 72
        74, // on Int, goto 73
        75, // on Lbrace, goto 74
        0, // on Let, error
        76, // on Lparen, goto 75
        96, // on Plus, goto 95
        77, // on Quote, goto 76
        0, // on Rbrace, error
        0, // on Rparen, error
        117, // on Semi, goto 116
        // State 102
        0, // on Arrow, error
        0, // on Eq, error
        -11, // on Fun, reduce `Expr = Expr, Atom => ActionFn(5);`
        -11, // on IdT, reduce `Expr = Expr, Atom => ActionFn(5);`
        -11, // on Int, reduce `Expr = Expr, Atom => ActionFn(5);`
        -11, // on Lbrace, reduce `Expr = Expr, Atom => ActionFn(5);`
        0, // on Let, error
        -11, // on Lparen, reduce `Expr = Expr, Atom => ActionFn(5);`
        -11, // on Plus, reduce `Expr = Expr, Atom => ActionFn(5);`
        -11, // on Quote, reduce `Expr = Expr, Atom => ActionFn(5);`
        -11, // on Rbrace, reduce `Expr = Expr, Atom => ActionFn(5);`
        0, // on Rparen, error
        0, // on Semi, error
        // State 103
        0, // on Arrow, error
        0, // on Eq, error
        87, // on Fun, goto 86
        88, // on IdT, goto 87
        89, // on Int, goto 88
        90, // on Lbrace, goto 89
        0, // on Let, error
        91, // on Lparen, goto 90
        0, // on Plus, error
        92, // on Quote, goto 91
        0, // on Rbrace, error
        0, // on Rparen, error
        0, // on Semi, error
        // State 104
        0, // on Arrow, error
        0, // on Eq, error
        -5, // on Fun, reduce `Atom = Fun, Id, Arrow, Lbrace, Expr, Rbrace => ActionFn(11);`
        -5, // on IdT, reduce `Atom = Fun, Id, Arrow, Lbrace, Expr, Rbrace => ActionFn(11);`
        -5, // on Int, reduce `Atom = Fun, Id, Arrow, Lbrace, Expr, Rbrace => ActionFn(11);`
        -5, // on Lbrace, reduce `Atom = Fun, Id, Arrow, Lbrace, Expr, Rbrace => ActionFn(11);`
        0, // on Let, error
        -5, // on Lparen, reduce `Atom = Fun, Id, Arrow, Lbrace, Expr, Rbrace => ActionFn(11);`
        -5, // on Plus, reduce `Atom = Fun, Id, Arrow, Lbrace, Expr, Rbrace => ActionFn(11);`
        -5, // on Quote, reduce `Atom = Fun, Id, Arrow, Lbrace, Expr, Rbrace => ActionFn(11);`
        0, // on Rbrace, error
        0, // on Rparen, error
        -5, // on Semi, reduce `Atom = Fun, Id, Arrow, Lbrace, Expr, Rbrace => ActionFn(11);`
        // State 105
        119, // on Arrow, goto 118
        0, // on Eq, error
        0, // on Fun, error
        0, // on IdT, error
        0, // on Int, error
        0, // on Lbrace, error
        0, // on Let, error
        0, // on Lparen, error
        0, // on Plus, error
        0, // on Quote, error
        0, // on Rbrace, error
        0, // on Rparen, error
        0, // on Semi, error
        // State 106
        0, // on Arrow, error
        0, // on Eq, error
        0, // on Fun, error
        0, // on IdT, error
        0, // on Int, error
        0, // on Lbrace, error
        0, // on Let, error
        0, // on Lparen, error
        0, // on Plus, error
        0, // on Quote, error
        120, // on Rbrace, goto 119
        0, // on Rparen, error
        46, // on Semi, goto 45
        // State 107
        0, // on Arrow, error
        0, // on Eq, error
        0, // on Fun, error
        0, // on IdT, error
        0, // on Int, error
        0, // on Lbrace, error
        0, // on Let, error
        0, // on Lparen, error
        0, // on Plus, error
        0, // on Quote, error
        0, // on Rbrace, error
        121, // on Rparen, goto 120
        55, // on Semi, goto 54
        // State 108
        0, // on Arrow, error
        0, // on Eq, error
        87, // on Fun, goto 86
        88, // on IdT, goto 87
        89, // on Int, goto 88
        90, // on Lbrace, goto 89
        0, // on Let, error
        91, // on Lparen, goto 90
        104, // on Plus, goto 103
        92, // on Quote, goto 91
        122, // on Rbrace, goto 121
        0, // on Rparen, error
        0, // on Semi, error
        // State 109
        0, // on Arrow, error
        0, // on Eq, error
        22, // on Fun, goto 21
        23, // on IdT, goto 22
        24, // on Int, goto 23
        25, // on Lbrace, goto 24
        0, // on Let, error
        27, // on Lparen, goto 26
        0, // on Plus, error
        28, // on Quote, goto 27
        0, // on Rbrace, error
        0, // on Rparen, error
        0, // on Semi, error
        // State 110
        0, // on Arrow, error
        0, // on Eq, error
        -10, // on Fun, reduce `Expr = Expr, Plus, Atom => ActionFn(4);`
        -10, // on IdT, reduce `Expr = Expr, Plus, Atom => ActionFn(4);`
        -10, // on Int, reduce `Expr = Expr, Plus, Atom => ActionFn(4);`
        -10, // on Lbrace, reduce `Expr = Expr, Plus, Atom => ActionFn(4);`
        0, // on Let, error
        -10, // on Lparen, reduce `Expr = Expr, Plus, Atom => ActionFn(4);`
        -10, // on Plus, reduce `Expr = Expr, Plus, Atom => ActionFn(4);`
        -10, // on Quote, reduce `Expr = Expr, Plus, Atom => ActionFn(4);`
        0, // on Rbrace, error
        0, // on Rparen, error
        -10, // on Semi, reduce `Expr = Expr, Plus, Atom => ActionFn(4);`
        // State 111
        0, // on Arrow, error
        0, // on Eq, error
        6, // on Fun, goto 5
        7, // on IdT, goto 6
        8, // on Int, goto 7
        9, // on Lbrace, goto 8
        0, // on Let, error
        11, // on Lparen, goto 10
        15, // on Plus, goto 14
        12, // on Quote, goto 11
        0, // on Rbrace, error
        0, // on Rparen, error
        -7, // on Semi, reduce `Block = Let, Id, Eq, Expr, Semi, Expr => ActionFn(1);`
        // State 112
        0, // on Arrow, error
        0, // on Eq, error
        0, // on Fun, error
        0, // on IdT, error
        0, // on Int, error
        124, // on Lbrace, goto 123
        0, // on Let, error
        0, // on Lparen, error
        0, // on Plus, error
        0, // on Quote, error
        0, // on Rbrace, error
        0, // on Rparen, error
        0, // on Semi, error
        // State 113
        0, // on Arrow, error
        0, // on Eq, error
        -4, // on Fun, reduce `Atom = Lbrace, Block, Rbrace => ActionFn(10);`
        -4, // on IdT, reduce `Atom = Lbrace, Block, Rbrace => ActionFn(10);`
        -4, // on Int, reduce `Atom = Lbrace, Block, Rbrace => ActionFn(10);`
        -4, // on Lbrace, reduce `Atom = Lbrace, Block, Rbrace => ActionFn(10);`
        0, // on Let, error
        -4, // on Lparen, reduce `Atom = Lbrace, Block, Rbrace => ActionFn(10);`
        -4, // on Plus, reduce `Atom = Lbrace, Block, Rbrace => ActionFn(10);`
        -4, // on Quote, reduce `Atom = Lbrace, Block, Rbrace => ActionFn(10);`
        0, // on Rbrace, error
        0, // on Rparen, error
        -4, // on Semi, reduce `Atom = Lbrace, Block, Rbrace => ActionFn(10);`
        // State 114
        0, // on Arrow, error
        0, // on Eq, error
        -3, // on Fun, reduce `Atom = Lparen, Block, Rparen => ActionFn(9);`
        -3, // on IdT, reduce `Atom = Lparen, Block, Rparen => ActionFn(9);`
        -3, // on Int, reduce `Atom = Lparen, Block, Rparen => ActionFn(9);`
        -3, // on Lbrace, reduce `Atom = Lparen, Block, Rparen => ActionFn(9);`
        0, // on Let, error
        -3, // on Lparen, reduce `Atom = Lparen, Block, Rparen => ActionFn(9);`
        -3, // on Plus, reduce `Atom = Lparen, Block, Rparen => ActionFn(9);`
        -3, // on Quote, reduce `Atom = Lparen, Block, Rparen => ActionFn(9);`
        0, // on Rbrace, error
        0, // on Rparen, error
        -3, // on Semi, reduce `Atom = Lparen, Block, Rparen => ActionFn(9);`
        // State 115
        0, // on Arrow, error
        0, // on Eq, error
        87, // on Fun, goto 86
        88, // on IdT, goto 87
        89, // on Int, goto 88
        90, // on Lbrace, goto 89
        0, // on Let, error
        91, // on Lparen, goto 90
        104, // on Plus, goto 103
        92, // on Quote, goto 91
        125, // on Rbrace, goto 124
        0, // on Rparen, error
        0, // on Semi, error
        // State 116
        0, // on Arrow, error
        0, // on Eq, error
        35, // on Fun, goto 34
        36, // on IdT, goto 35
        37, // on Int, goto 36
        38, // on Lbrace, goto 37
        0, // on Let, error
        40, // on Lparen, goto 39
        0, // on Plus, error
        41, // on Quote, goto 40
        0, // on Rbrace, error
        0, // on Rparen, error
        0, // on Semi, error
        // State 117
        0, // on Arrow, error
        0, // on Eq, error
        -10, // on Fun, reduce `Expr = Expr, Plus, Atom => ActionFn(4);`
        -10, // on IdT, reduce `Expr = Expr, Plus, Atom => ActionFn(4);`
        -10, // on Int, reduce `Expr = Expr, Plus, Atom => ActionFn(4);`
        -10, // on Lbrace, reduce `Expr = Expr, Plus, Atom => ActionFn(4);`
        0, // on Let, error
        -10, // on Lparen, reduce `Expr = Expr, Plus, Atom => ActionFn(4);`
        -10, // on Plus, reduce `Expr = Expr, Plus, Atom => ActionFn(4);`
        -10, // on Quote, reduce `Expr = Expr, Plus, Atom => ActionFn(4);`
        -10, // on Rbrace, reduce `Expr = Expr, Plus, Atom => ActionFn(4);`
        0, // on Rparen, error
        0, // on Semi, error
        // State 118
        0, // on Arrow, error
        0, // on Eq, error
        0, // on Fun, error
        0, // on IdT, error
        0, // on Int, error
        127, // on Lbrace, goto 126
        0, // on Let, error
        0, // on Lparen, error
        0, // on Plus, error
        0, // on Quote, error
        0, // on Rbrace, error
        0, // on Rparen, error
        0, // on Semi, error
        // State 119
        0, // on Arrow, error
        0, // on Eq, error
        -4, // on Fun, reduce `Atom = Lbrace, Block, Rbrace => ActionFn(10);`
        -4, // on IdT, reduce `Atom = Lbrace, Block, Rbrace => ActionFn(10);`
        -4, // on Int, reduce `Atom = Lbrace, Block, Rbrace => ActionFn(10);`
        -4, // on Lbrace, reduce `Atom = Lbrace, Block, Rbrace => ActionFn(10);`
        0, // on Let, error
        -4, // on Lparen, reduce `Atom = Lbrace, Block, Rbrace => ActionFn(10);`
        -4, // on Plus, reduce `Atom = Lbrace, Block, Rbrace => ActionFn(10);`
        -4, // on Quote, reduce `Atom = Lbrace, Block, Rbrace => ActionFn(10);`
        -4, // on Rbrace, reduce `Atom = Lbrace, Block, Rbrace => ActionFn(10);`
        0, // on Rparen, error
        0, // on Semi, error
        // State 120
        0, // on Arrow, error
        0, // on Eq, error
        -3, // on Fun, reduce `Atom = Lparen, Block, Rparen => ActionFn(9);`
        -3, // on IdT, reduce `Atom = Lparen, Block, Rparen => ActionFn(9);`
        -3, // on Int, reduce `Atom = Lparen, Block, Rparen => ActionFn(9);`
        -3, // on Lbrace, reduce `Atom = Lparen, Block, Rparen => ActionFn(9);`
        0, // on Let, error
        -3, // on Lparen, reduce `Atom = Lparen, Block, Rparen => ActionFn(9);`
        -3, // on Plus, reduce `Atom = Lparen, Block, Rparen => ActionFn(9);`
        -3, // on Quote, reduce `Atom = Lparen, Block, Rparen => ActionFn(9);`
        -3, // on Rbrace, reduce `Atom = Lparen, Block, Rparen => ActionFn(9);`
        0, // on Rparen, error
        0, // on Semi, error
        // State 121
        0, // on Arrow, error
        0, // on Eq, error
        -5, // on Fun, reduce `Atom = Fun, Id, Arrow, Lbrace, Expr, Rbrace => ActionFn(11);`
        -5, // on IdT, reduce `Atom = Fun, Id, Arrow, Lbrace, Expr, Rbrace => ActionFn(11);`
        -5, // on Int, reduce `Atom = Fun, Id, Arrow, Lbrace, Expr, Rbrace => ActionFn(11);`
        -5, // on Lbrace, reduce `Atom = Fun, Id, Arrow, Lbrace, Expr, Rbrace => ActionFn(11);`
        0, // on Let, error
        -5, // on Lparen, reduce `Atom = Fun, Id, Arrow, Lbrace, Expr, Rbrace => ActionFn(11);`
        -5, // on Plus, reduce `Atom = Fun, Id, Arrow, Lbrace, Expr, Rbrace => ActionFn(11);`
        -5, // on Quote, reduce `Atom = Fun, Id, Arrow, Lbrace, Expr, Rbrace => ActionFn(11);`
        -5, // on Rbrace, reduce `Atom = Fun, Id, Arrow, Lbrace, Expr, Rbrace => ActionFn(11);`
        0, // on Rparen, error
        -5, // on Semi, reduce `Atom = Fun, Id, Arrow, Lbrace, Expr, Rbrace => ActionFn(11);`
        // State 122
        0, // on Arrow, error
        0, // on Eq, error
        22, // on Fun, goto 21
        23, // on IdT, goto 22
        24, // on Int, goto 23
        25, // on Lbrace, goto 24
        0, // on Let, error
        27, // on Lparen, goto 26
        48, // on Plus, goto 47
        28, // on Quote, goto 27
        -7, // on Rbrace, reduce `Block = Let, Id, Eq, Expr, Semi, Expr => ActionFn(1);`
        0, // on Rparen, error
        -7, // on Semi, reduce `Block = Let, Id, Eq, Expr, Semi, Expr => ActionFn(1);`
        // State 123
        0, // on Arrow, error
        0, // on Eq, error
        87, // on Fun, goto 86
        88, // on IdT, goto 87
        89, // on Int, goto 88
        90, // on Lbrace, goto 89
        0, // on Let, error
        91, // on Lparen, goto 90
        0, // on Plus, error
        92, // on Quote, goto 91
        0, // on Rbrace, error
        0, // on Rparen, error
        0, // on Semi, error
        // State 124
        0, // on Arrow, error
        0, // on Eq, error
        -5, // on Fun, reduce `Atom = Fun, Id, Arrow, Lbrace, Expr, Rbrace => ActionFn(11);`
        -5, // on IdT, reduce `Atom = Fun, Id, Arrow, Lbrace, Expr, Rbrace => ActionFn(11);`
        -5, // on Int, reduce `Atom = Fun, Id, Arrow, Lbrace, Expr, Rbrace => ActionFn(11);`
        -5, // on Lbrace, reduce `Atom = Fun, Id, Arrow, Lbrace, Expr, Rbrace => ActionFn(11);`
        0, // on Let, error
        -5, // on Lparen, reduce `Atom = Fun, Id, Arrow, Lbrace, Expr, Rbrace => ActionFn(11);`
        -5, // on Plus, reduce `Atom = Fun, Id, Arrow, Lbrace, Expr, Rbrace => ActionFn(11);`
        -5, // on Quote, reduce `Atom = Fun, Id, Arrow, Lbrace, Expr, Rbrace => ActionFn(11);`
        0, // on Rbrace, error
        -5, // on Rparen, reduce `Atom = Fun, Id, Arrow, Lbrace, Expr, Rbrace => ActionFn(11);`
        -5, // on Semi, reduce `Atom = Fun, Id, Arrow, Lbrace, Expr, Rbrace => ActionFn(11);`
        // State 125
        0, // on Arrow, error
        0, // on Eq, error
        35, // on Fun, goto 34
        36, // on IdT, goto 35
        37, // on Int, goto 36
        38, // on Lbrace, goto 37
        0, // on Let, error
        40, // on Lparen, goto 39
        57, // on Plus, goto 56
        41, // on Quote, goto 40
        0, // on Rbrace, error
        -7, // on Rparen, reduce `Block = Let, Id, Eq, Expr, Semi, Expr => ActionFn(1);`
        -7, // on Semi, reduce `Block = Let, Id, Eq, Expr, Semi, Expr => ActionFn(1);`
        // State 126
        0, // on Arrow, error
        0, // on Eq, error
        87, // on Fun, goto 86
        88, // on IdT, goto 87
        89, // on Int, goto 88
        90, // on Lbrace, goto 89
        0, // on Let, error
        91, // on Lparen, goto 90
        0, // on Plus, error
        92, // on Quote, goto 91
        0, // on Rbrace, error
        0, // on Rparen, error
        0, // on Semi, error
        // State 127
        0, // on Arrow, error
        0, // on Eq, error
        87, // on Fun, goto 86
        88, // on IdT, goto 87
        89, // on Int, goto 88
        90, // on Lbrace, goto 89
        0, // on Let, error
        91, // on Lparen, goto 90
        104, // on Plus, goto 103
        92, // on Quote, goto 91
        130, // on Rbrace, goto 129
        0, // on Rparen, error
        0, // on Semi, error
        // State 128
        0, // on Arrow, error
        0, // on Eq, error
        87, // on Fun, goto 86
        88, // on IdT, goto 87
        89, // on Int, goto 88
        90, // on Lbrace, goto 89
        0, // on Let, error
        91, // on Lparen, goto 90
        104, // on Plus, goto 103
        92, // on Quote, goto 91
        131, // on Rbrace, goto 130
        0, // on Rparen, error
        0, // on Semi, error
        // State 129
        0, // on Arrow, error
        0, // on Eq, error
        -5, // on Fun, reduce `Atom = Fun, Id, Arrow, Lbrace, Expr, Rbrace => ActionFn(11);`
        -5, // on IdT, reduce `Atom = Fun, Id, Arrow, Lbrace, Expr, Rbrace => ActionFn(11);`
        -5, // on Int, reduce `Atom = Fun, Id, Arrow, Lbrace, Expr, Rbrace => ActionFn(11);`
        -5, // on Lbrace, reduce `Atom = Fun, Id, Arrow, Lbrace, Expr, Rbrace => ActionFn(11);`
        0, // on Let, error
        -5, // on Lparen, reduce `Atom = Fun, Id, Arrow, Lbrace, Expr, Rbrace => ActionFn(11);`
        -5, // on Plus, reduce `Atom = Fun, Id, Arrow, Lbrace, Expr, Rbrace => ActionFn(11);`
        -5, // on Quote, reduce `Atom = Fun, Id, Arrow, Lbrace, Expr, Rbrace => ActionFn(11);`
        0, // on Rbrace, error
        0, // on Rparen, error
        -5, // on Semi, reduce `Atom = Fun, Id, Arrow, Lbrace, Expr, Rbrace => ActionFn(11);`
        // State 130
        0, // on Arrow, error
        0, // on Eq, error
        -5, // on Fun, reduce `Atom = Fun, Id, Arrow, Lbrace, Expr, Rbrace => ActionFn(11);`
        -5, // on IdT, reduce `Atom = Fun, Id, Arrow, Lbrace, Expr, Rbrace => ActionFn(11);`
        -5, // on Int, reduce `Atom = Fun, Id, Arrow, Lbrace, Expr, Rbrace => ActionFn(11);`
        -5, // on Lbrace, reduce `Atom = Fun, Id, Arrow, Lbrace, Expr, Rbrace => ActionFn(11);`
        0, // on Let, error
        -5, // on Lparen, reduce `Atom = Fun, Id, Arrow, Lbrace, Expr, Rbrace => ActionFn(11);`
        -5, // on Plus, reduce `Atom = Fun, Id, Arrow, Lbrace, Expr, Rbrace => ActionFn(11);`
        -5, // on Quote, reduce `Atom = Fun, Id, Arrow, Lbrace, Expr, Rbrace => ActionFn(11);`
        -5, // on Rbrace, reduce `Atom = Fun, Id, Arrow, Lbrace, Expr, Rbrace => ActionFn(11);`
        0, // on Rparen, error
        0, // on Semi, error
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0, // on EOF, error
        -12, // on EOF, reduce `Expr = Atom => ActionFn(6);`
        -14, // on EOF, reduce `__Block = Block => ActionFn(0);`
        -9, // on EOF, reduce `Block = Expr => ActionFn(3);`
        -2, // on EOF, reduce `Atom = Id => ActionFn(8);`
        0, // on EOF, error
        -13, // on EOF, reduce `Id = IdT => ActionFn(13);`
        -1, // on EOF, reduce `Atom = Int => ActionFn(7);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -6, // on EOF, reduce `Atom = Quote => ActionFn(12);`
        0, // on EOF, error
        -11, // on EOF, reduce `Expr = Expr, Atom => ActionFn(5);`
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
        -8, // on EOF, reduce `Block = Block, Semi, Expr => ActionFn(2);`
        -10, // on EOF, reduce `Expr = Expr, Plus, Atom => ActionFn(4);`
        0, // on EOF, error
        -4, // on EOF, reduce `Atom = Lbrace, Block, Rbrace => ActionFn(10);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -3, // on EOF, reduce `Atom = Lparen, Block, Rparen => ActionFn(9);`
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
        -5, // on EOF, reduce `Atom = Fun, Id, Arrow, Lbrace, Expr, Rbrace => ActionFn(11);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -7, // on EOF, reduce `Block = Let, Id, Eq, Expr, Semi, Expr => ActionFn(1);`
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
        14, // on Atom, goto 13
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
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        16, // on Id, goto 15
        0, // on __Block, error
        // State 6
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 7
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 8
        18, // on Atom, goto 17
        19, // on Block, goto 18
        20, // on Expr, goto 19
        21, // on Id, goto 20
        0, // on __Block, error
        // State 9
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        29, // on Id, goto 28
        0, // on __Block, error
        // State 10
        31, // on Atom, goto 30
        32, // on Block, goto 31
        33, // on Expr, goto 32
        34, // on Id, goto 33
        0, // on __Block, error
        // State 11
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 12
        2, // on Atom, goto 1
        0, // on Block, error
        42, // on Expr, goto 41
        5, // on Id, goto 4
        0, // on __Block, error
        // State 13
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 14
        43, // on Atom, goto 42
        0, // on Block, error
        0, // on Expr, error
        5, // on Id, goto 4
        0, // on __Block, error
        // State 15
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 16
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 17
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 18
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 19
        47, // on Atom, goto 46
        0, // on Block, error
        0, // on Expr, error
        21, // on Id, goto 20
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
        49, // on Id, goto 48
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
        18, // on Atom, goto 17
        50, // on Block, goto 49
        20, // on Expr, goto 19
        21, // on Id, goto 20
        0, // on __Block, error
        // State 25
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        51, // on Id, goto 50
        0, // on __Block, error
        // State 26
        31, // on Atom, goto 30
        52, // on Block, goto 51
        33, // on Expr, goto 32
        34, // on Id, goto 33
        0, // on __Block, error
        // State 27
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 28
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
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
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 32
        56, // on Atom, goto 55
        0, // on Block, error
        0, // on Expr, error
        34, // on Id, goto 33
        0, // on __Block, error
        // State 33
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 34
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        58, // on Id, goto 57
        0, // on __Block, error
        // State 35
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 36
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 37
        18, // on Atom, goto 17
        59, // on Block, goto 58
        20, // on Expr, goto 19
        21, // on Id, goto 20
        0, // on __Block, error
        // State 38
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        60, // on Id, goto 59
        0, // on __Block, error
        // State 39
        31, // on Atom, goto 30
        61, // on Block, goto 60
        33, // on Expr, goto 32
        34, // on Id, goto 33
        0, // on __Block, error
        // State 40
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 41
        14, // on Atom, goto 13
        0, // on Block, error
        0, // on Expr, error
        5, // on Id, goto 4
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
        18, // on Atom, goto 17
        0, // on Block, error
        63, // on Expr, goto 62
        21, // on Id, goto 20
        0, // on __Block, error
        // State 46
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 47
        64, // on Atom, goto 63
        0, // on Block, error
        0, // on Expr, error
        21, // on Id, goto 20
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
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 51
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 52
        69, // on Atom, goto 68
        0, // on Block, error
        70, // on Expr, goto 69
        71, // on Id, goto 70
        0, // on __Block, error
        // State 53
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 54
        31, // on Atom, goto 30
        0, // on Block, error
        78, // on Expr, goto 77
        34, // on Id, goto 33
        0, // on __Block, error
        // State 55
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 56
        79, // on Atom, goto 78
        0, // on Block, error
        0, // on Expr, error
        34, // on Id, goto 33
        0, // on __Block, error
        // State 57
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 58
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 59
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 60
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 61
        84, // on Atom, goto 83
        0, // on Block, error
        85, // on Expr, goto 84
        86, // on Id, goto 85
        0, // on __Block, error
        // State 62
        47, // on Atom, goto 46
        0, // on Block, error
        0, // on Expr, error
        21, // on Id, goto 20
        0, // on __Block, error
        // State 63
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 64
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 65
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 66
        69, // on Atom, goto 68
        0, // on Block, error
        94, // on Expr, goto 93
        71, // on Id, goto 70
        0, // on __Block, error
        // State 67
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 68
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 69
        95, // on Atom, goto 94
        0, // on Block, error
        0, // on Expr, error
        71, // on Id, goto 70
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
        98, // on Id, goto 97
        0, // on __Block, error
        // State 72
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 73
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 74
        18, // on Atom, goto 17
        99, // on Block, goto 98
        20, // on Expr, goto 19
        21, // on Id, goto 20
        0, // on __Block, error
        // State 75
        31, // on Atom, goto 30
        100, // on Block, goto 99
        33, // on Expr, goto 32
        34, // on Id, goto 33
        0, // on __Block, error
        // State 76
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 77
        56, // on Atom, goto 55
        0, // on Block, error
        0, // on Expr, error
        34, // on Id, goto 33
        0, // on __Block, error
        // State 78
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
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
        69, // on Atom, goto 68
        0, // on Block, error
        102, // on Expr, goto 101
        71, // on Id, goto 70
        0, // on __Block, error
        // State 82
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 83
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 84
        103, // on Atom, goto 102
        0, // on Block, error
        0, // on Expr, error
        86, // on Id, goto 85
        0, // on __Block, error
        // State 85
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 86
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        106, // on Id, goto 105
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
        18, // on Atom, goto 17
        107, // on Block, goto 106
        20, // on Expr, goto 19
        21, // on Id, goto 20
        0, // on __Block, error
        // State 90
        31, // on Atom, goto 30
        108, // on Block, goto 107
        33, // on Expr, goto 32
        34, // on Id, goto 33
        0, // on __Block, error
        // State 91
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 92
        84, // on Atom, goto 83
        0, // on Block, error
        109, // on Expr, goto 108
        86, // on Id, goto 85
        0, // on __Block, error
        // State 93
        95, // on Atom, goto 94
        0, // on Block, error
        0, // on Expr, error
        71, // on Id, goto 70
        0, // on __Block, error
        // State 94
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 95
        111, // on Atom, goto 110
        0, // on Block, error
        0, // on Expr, error
        71, // on Id, goto 70
        0, // on __Block, error
        // State 96
        2, // on Atom, goto 1
        0, // on Block, error
        112, // on Expr, goto 111
        5, // on Id, goto 4
        0, // on __Block, error
        // State 97
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 98
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 99
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 100
        84, // on Atom, goto 83
        0, // on Block, error
        116, // on Expr, goto 115
        86, // on Id, goto 85
        0, // on __Block, error
        // State 101
        95, // on Atom, goto 94
        0, // on Block, error
        0, // on Expr, error
        71, // on Id, goto 70
        0, // on __Block, error
        // State 102
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 103
        118, // on Atom, goto 117
        0, // on Block, error
        0, // on Expr, error
        86, // on Id, goto 85
        0, // on __Block, error
        // State 104
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 105
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 106
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 107
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 108
        103, // on Atom, goto 102
        0, // on Block, error
        0, // on Expr, error
        86, // on Id, goto 85
        0, // on __Block, error
        // State 109
        18, // on Atom, goto 17
        0, // on Block, error
        123, // on Expr, goto 122
        21, // on Id, goto 20
        0, // on __Block, error
        // State 110
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 111
        14, // on Atom, goto 13
        0, // on Block, error
        0, // on Expr, error
        5, // on Id, goto 4
        0, // on __Block, error
        // State 112
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 113
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 114
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 115
        103, // on Atom, goto 102
        0, // on Block, error
        0, // on Expr, error
        86, // on Id, goto 85
        0, // on __Block, error
        // State 116
        31, // on Atom, goto 30
        0, // on Block, error
        126, // on Expr, goto 125
        34, // on Id, goto 33
        0, // on __Block, error
        // State 117
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 118
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 119
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 120
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 121
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 122
        47, // on Atom, goto 46
        0, // on Block, error
        0, // on Expr, error
        21, // on Id, goto 20
        0, // on __Block, error
        // State 123
        84, // on Atom, goto 83
        0, // on Block, error
        128, // on Expr, goto 127
        86, // on Id, goto 85
        0, // on __Block, error
        // State 124
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 125
        56, // on Atom, goto 55
        0, // on Block, error
        0, // on Expr, error
        34, // on Id, goto 33
        0, // on __Block, error
        // State 126
        84, // on Atom, goto 83
        0, // on Block, error
        129, // on Expr, goto 128
        86, // on Id, goto 85
        0, // on __Block, error
        // State 127
        103, // on Atom, goto 102
        0, // on Block, error
        0, // on Expr, error
        86, // on Id, goto 85
        0, // on __Block, error
        // State 128
        103, // on Atom, goto 102
        0, // on Block, error
        0, // on Expr, error
        86, // on Id, goto 85
        0, // on __Block, error
        // State 129
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
        // State 130
        0, // on Atom, error
        0, // on Block, error
        0, // on Expr, error
        0, // on Id, error
        0, // on __Block, error
    ];
    pub fn parse_Block<
        __TOKEN: __ToTriple<Error=()>,
        __TOKENS: IntoIterator<Item=__TOKEN>,
    >(
        __tokens0: __TOKENS,
    ) -> Result<Term, __lalrpop_util::ParseError<(),Token,()>>
    {
        let __tokens = __tokens0.into_iter();
        let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        '__shift: loop {
            let __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
            };
            let __integer = match __lookahead {
                (_, Token::Arrow, _) if true => 0,
                (_, Token::Eq, _) if true => 1,
                (_, Token::Fun, _) if true => 2,
                (_, Token::Id(_), _) if true => 3,
                (_, Token::Int(_), _) if true => 4,
                (_, Token::Lbrace, _) if true => 5,
                (_, Token::Let, _) if true => 6,
                (_, Token::Lparen, _) if true => 7,
                (_, Token::Plus, _) if true => 8,
                (_, Token::Quote(_), _) if true => 9,
                (_, Token::Rbrace, _) if true => 10,
                (_, Token::Rparen, _) if true => 11,
                (_, Token::Semi, _) if true => 12,
                _ => {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            };
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 13 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            __tok @ Token::Arrow => __Symbol::TermArrow(__tok),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            __tok @ Token::Eq => __Symbol::TermEq(__tok),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            __tok @ Token::Fun => __Symbol::TermFun(__tok),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            Token::Id(__tok0) => __Symbol::TermIdT(__tok0),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            Token::Int(__tok0) => __Symbol::TermInt(__tok0),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            __tok @ Token::Lbrace => __Symbol::TermLbrace(__tok),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            __tok @ Token::Let => __Symbol::TermLet(__tok),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            __tok @ Token::Lparen => __Symbol::TermLparen(__tok),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            __tok @ Token::Plus => __Symbol::TermPlus(__tok),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            Token::Quote(__tok0) => __Symbol::TermQuote(__tok0),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            __tok @ Token::Rbrace => __Symbol::TermRbrace(__tok),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            __tok @ Token::Rparen => __Symbol::TermRparen(__tok),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            __tok @ Token::Semi => __Symbol::TermSemi(__tok),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(__action, Some(&__lookahead.0), &mut __states, &mut __symbols) {
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
                if let Some(r) = __reduce(__action, None, &mut __states, &mut __symbols) {
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
    >(
        __action: i32,
        __lookahead_start: Option<&()>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<((),__Symbol<>,())>,
    ) -> Option<Result<Term,__lalrpop_util::ParseError<(),Token,()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // Atom = Int => ActionFn(7);
                let __sym0 = __pop_TermInt(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                0
            }
            2 => {
                // Atom = Id => ActionFn(8);
                let __sym0 = __pop_NtId(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                0
            }
            3 => {
                // Atom = Lparen, Block, Rparen => ActionFn(9);
                let __sym2 = __pop_TermRparen(__symbols);
                let __sym1 = __pop_NtBlock(__symbols);
                let __sym0 = __pop_TermLparen(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action9(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                0
            }
            4 => {
                // Atom = Lbrace, Block, Rbrace => ActionFn(10);
                let __sym2 = __pop_TermRbrace(__symbols);
                let __sym1 = __pop_NtBlock(__symbols);
                let __sym0 = __pop_TermLbrace(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action10(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                0
            }
            5 => {
                // Atom = Fun, Id, Arrow, Lbrace, Expr, Rbrace => ActionFn(11);
                let __sym5 = __pop_TermRbrace(__symbols);
                let __sym4 = __pop_NtExpr(__symbols);
                let __sym3 = __pop_TermLbrace(__symbols);
                let __sym2 = __pop_TermArrow(__symbols);
                let __sym1 = __pop_NtId(__symbols);
                let __sym0 = __pop_TermFun(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym5.2.clone();
                let __nt = super::__action11(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
                let __states_len = __states.len();
                __states.truncate(__states_len - 6);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                0
            }
            6 => {
                // Atom = Quote => ActionFn(12);
                let __sym0 = __pop_TermQuote(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAtom(__nt), __end));
                0
            }
            7 => {
                // Block = Let, Id, Eq, Expr, Semi, Expr => ActionFn(1);
                let __sym5 = __pop_NtExpr(__symbols);
                let __sym4 = __pop_TermSemi(__symbols);
                let __sym3 = __pop_NtExpr(__symbols);
                let __sym2 = __pop_TermEq(__symbols);
                let __sym1 = __pop_NtId(__symbols);
                let __sym0 = __pop_TermLet(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym5.2.clone();
                let __nt = super::__action1(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
                let __states_len = __states.len();
                __states.truncate(__states_len - 6);
                __symbols.push((__start, __Symbol::NtBlock(__nt), __end));
                1
            }
            8 => {
                // Block = Block, Semi, Expr => ActionFn(2);
                let __sym2 = __pop_NtExpr(__symbols);
                let __sym1 = __pop_TermSemi(__symbols);
                let __sym0 = __pop_NtBlock(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action2(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtBlock(__nt), __end));
                1
            }
            9 => {
                // Block = Expr => ActionFn(3);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBlock(__nt), __end));
                1
            }
            10 => {
                // Expr = Expr, Plus, Atom => ActionFn(4);
                let __sym2 = __pop_NtAtom(__symbols);
                let __sym1 = __pop_TermPlus(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action4(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                2
            }
            11 => {
                // Expr = Expr, Atom => ActionFn(5);
                let __sym1 = __pop_NtAtom(__symbols);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action5(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                2
            }
            12 => {
                // Expr = Atom => ActionFn(6);
                let __sym0 = __pop_NtAtom(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                2
            }
            13 => {
                // Id = IdT => ActionFn(13);
                let __sym0 = __pop_TermIdT(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtId(__nt), __end));
                3
            }
            14 => {
                // __Block = Block => ActionFn(0);
                let __sym0 = __pop_NtBlock(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0(__sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 5 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_TermArrow<
    >(
        __symbols: &mut ::std::vec::Vec<((),__Symbol<>,())>
    ) -> ((), Token, ()) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::TermArrow(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_TermEq<
    >(
        __symbols: &mut ::std::vec::Vec<((),__Symbol<>,())>
    ) -> ((), Token, ()) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::TermEq(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_TermFun<
    >(
        __symbols: &mut ::std::vec::Vec<((),__Symbol<>,())>
    ) -> ((), Token, ()) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::TermFun(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_TermIdT<
    >(
        __symbols: &mut ::std::vec::Vec<((),__Symbol<>,())>
    ) -> ((), String, ()) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::TermIdT(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_TermInt<
    >(
        __symbols: &mut ::std::vec::Vec<((),__Symbol<>,())>
    ) -> ((), i32, ()) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::TermInt(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_TermLbrace<
    >(
        __symbols: &mut ::std::vec::Vec<((),__Symbol<>,())>
    ) -> ((), Token, ()) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::TermLbrace(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_TermLet<
    >(
        __symbols: &mut ::std::vec::Vec<((),__Symbol<>,())>
    ) -> ((), Token, ()) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::TermLet(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_TermLparen<
    >(
        __symbols: &mut ::std::vec::Vec<((),__Symbol<>,())>
    ) -> ((), Token, ()) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::TermLparen(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_TermPlus<
    >(
        __symbols: &mut ::std::vec::Vec<((),__Symbol<>,())>
    ) -> ((), Token, ()) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::TermPlus(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_TermQuote<
    >(
        __symbols: &mut ::std::vec::Vec<((),__Symbol<>,())>
    ) -> ((), String, ()) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::TermQuote(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_TermRbrace<
    >(
        __symbols: &mut ::std::vec::Vec<((),__Symbol<>,())>
    ) -> ((), Token, ()) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::TermRbrace(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_TermRparen<
    >(
        __symbols: &mut ::std::vec::Vec<((),__Symbol<>,())>
    ) -> ((), Token, ()) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::TermRparen(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_TermSemi<
    >(
        __symbols: &mut ::std::vec::Vec<((),__Symbol<>,())>
    ) -> ((), Token, ()) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::TermSemi(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtAtom<
    >(
        __symbols: &mut ::std::vec::Vec<((),__Symbol<>,())>
    ) -> ((), Term, ()) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtAtom(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtBlock<
    >(
        __symbols: &mut ::std::vec::Vec<((),__Symbol<>,())>
    ) -> ((), Term, ()) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtBlock(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExpr<
    >(
        __symbols: &mut ::std::vec::Vec<((),__Symbol<>,())>
    ) -> ((), Term, ()) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExpr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtId<
    >(
        __symbols: &mut ::std::vec::Vec<((),__Symbol<>,())>
    ) -> ((), Var, ()) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtId(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Block<
    >(
        __symbols: &mut ::std::vec::Vec<((),__Symbol<>,())>
    ) -> ((), Term, ()) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Block(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Block::parse_Block;

pub fn __action0<
>(
    (_, __0, _): ((), Term, ()),
) -> Term
{
    (__0)
}

pub fn __action1<
>(
    (_, _, _): ((), Token, ()),
    (_, id, _): ((), Var, ()),
    (_, _, _): ((), Token, ()),
    (_, bind, _): ((), Term, ()),
    (_, _, _): ((), Token, ()),
    (_, body, _): ((), Term, ()),
) -> Term
{
    term::into(TermV::Let((bind, (id, body))))
}

pub fn __action2<
>(
    (_, l, _): ((), Term, ()),
    (_, _, _): ((), Token, ()),
    (_, r, _): ((), Term, ()),
) -> Term
{
    term::into(TermV::Let((l, (Var::new("dummy".to_string()), r))))
}

pub fn __action3<
>(
    (_, __0, _): ((), Term, ()),
) -> Term
{
    (__0)
}

pub fn __action4<
>(
    (_, l, _): ((), Term, ()),
    (_, _, _): ((), Token, ()),
    (_, r, _): ((), Term, ()),
) -> Term
{
    term::into(TermV::Plus((l, r)))
}

pub fn __action5<
>(
    (_, l, _): ((), Term, ()),
    (_, r, _): ((), Term, ()),
) -> Term
{
    term::into(TermV::App((l, r)))
}

pub fn __action6<
>(
    (_, __0, _): ((), Term, ()),
) -> Term
{
    (__0)
}

pub fn __action7<
>(
    (_, __0, _): ((), i32, ()),
) -> Term
{
    term::into(TermV::Number(__0))
}

pub fn __action8<
>(
    (_, __0, _): ((), Var, ()),
) -> Term
{
    term::into(TermV::Var(__0))
}

pub fn __action9<
>(
    (_, _, _): ((), Token, ()),
    (_, e, _): ((), Term, ()),
    (_, _, _): ((), Token, ()),
) -> Term
{
    e
}

pub fn __action10<
>(
    (_, _, _): ((), Token, ()),
    (_, e, _): ((), Term, ()),
    (_, _, _): ((), Token, ()),
) -> Term
{
    e
}

pub fn __action11<
>(
    (_, _, _): ((), Token, ()),
    (_, id, _): ((), Var, ()),
    (_, _, _): ((), Token, ()),
    (_, _, _): ((), Token, ()),
    (_, e, _): ((), Term, ()),
    (_, _, _): ((), Token, ()),
) -> Term
{
    term::into(TermV::Lam(((id, typ::into(TypV::Number)), e)))
}

pub fn __action12<
>(
    (_, __0, _): ((), String, ()),
) -> Term
{
    term::into(TermV::Quote(__0.to_string()))
}

pub fn __action13<
>(
    (_, __0, _): ((), String, ()),
) -> Var
{
    Var::new(__0)
}

pub trait __ToTriple<> {
    type Error;
    fn to_triple(value: Self) -> Result<((),Token,()),Self::Error>;
}

impl<> __ToTriple<> for Token {
    type Error = ();
    fn to_triple(value: Self) -> Result<((),Token,()),()> {
        Ok(((), value, ()))
    }
}
impl<> __ToTriple<> for Result<(Token),()> {
    type Error = ();
    fn to_triple(value: Self) -> Result<((),Token,()),()> {
        value.map(|v| ((), v, ()))
    }
}
