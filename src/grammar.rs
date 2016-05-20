#![allow(unused_imports)]
use std::str::FromStr;
use syntax::ast::{Name, Ident};
use syntax::parse::token::{Token as RsToken, BinOpToken, Lit, DelimToken};
use super::ast::{LiaExpr, LiaStmt, LiaFn};
use super::token::LiaToken;
extern crate lalrpop_util as __lalrpop_util;
use self::__lalrpop_util::ParseError as __ParseError;

mod __parse__fun {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use syntax::ast::{Name, Ident};
    use syntax::parse::token::{Token as RsToken, BinOpToken, Lit, DelimToken};
    use super::super::ast::{LiaExpr, LiaStmt, LiaFn};
    use super::super::token::LiaToken;
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    use super::__ToTriple;
    pub fn parse_fun<
        __TOKEN: __ToTriple<Error=()>,
        __TOKENS: IntoIterator<Item=__TOKEN>,
    >(
        __tokens: __TOKENS,
    ) -> Result<LiaFn, __ParseError<(),LiaToken,()>>
    {
        let __tokens = __tokens.into_iter();
        let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match try!(__state0(&mut __tokens, __lookahead)) {
            (Some(__lookahead), _) => {
                Err(__ParseError::ExtraToken { token: __lookahead })
            }
            (None, __Nonterminal::____fun((_, __nt, _))) => {
                Ok(__nt)
            }
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub enum __Nonterminal<> {
        ____fun(((), LiaFn, ())),
        atom(((), LiaExpr, ())),
        expr(((), LiaExpr, ())),
        fun(((), LiaFn, ())),
        stmt(((), LiaStmt, ())),
    }

    pub fn __state0<
        __TOKENS: Iterator<Item=Result<((), LiaToken, ()),()>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), LiaToken, ())>,
    ) -> Result<(Option<((), LiaToken, ())>, __Nonterminal<>), __ParseError<(),LiaToken,()>>
    {
        let mut __result: (Option<((), LiaToken, ())>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, __tok @ LiaToken::Function, __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state2(__tokens, __sym0));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        loop {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::fun(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state1(__tokens, __lookahead, __sym0));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
    }

    pub fn __state1<
        __TOKENS: Iterator<Item=Result<((), LiaToken, ()),()>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), LiaToken, ())>,
        __sym0: &mut Option<((), LiaFn, ())>,
    ) -> Result<(Option<((), LiaToken, ())>, __Nonterminal<>), __ParseError<(),LiaToken,()>>
    {
        let mut __result: (Option<((), LiaToken, ())>, __Nonterminal<>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0(__sym0);
                let __nt = __Nonterminal::____fun((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state2<
        __TOKENS: Iterator<Item=Result<((), LiaToken, ()),()>>,
    >(
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<((), LiaToken, ())>,
    ) -> Result<(Option<((), LiaToken, ())>, __Nonterminal<>), __ParseError<(),LiaToken,()>>
    {
        let mut __result: (Option<((), LiaToken, ())>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((__loc1, LiaToken::RustToken(RsToken::Ident(__tok0)), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state3(__tokens, __sym0, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state3<
        __TOKENS: Iterator<Item=Result<((), LiaToken, ()),()>>,
    >(
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<((), LiaToken, ())>,
        __sym1: &mut Option<((), Ident, ())>,
    ) -> Result<(Option<((), LiaToken, ())>, __Nonterminal<>), __ParseError<(),LiaToken,()>>
    {
        let mut __result: (Option<((), LiaToken, ())>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((__loc1, __tok @ LiaToken::RustToken(RsToken::OpenDelim(DelimToken::Paren)), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state4(__tokens, __sym0, __sym1, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state4<
        __TOKENS: Iterator<Item=Result<((), LiaToken, ()),()>>,
    >(
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<((), LiaToken, ())>,
        __sym1: &mut Option<((), Ident, ())>,
        __sym2: &mut Option<((), LiaToken, ())>,
    ) -> Result<(Option<((), LiaToken, ())>, __Nonterminal<>), __ParseError<(),LiaToken,()>>
    {
        let mut __result: (Option<((), LiaToken, ())>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((__loc1, __tok @ LiaToken::RustToken(RsToken::CloseDelim(DelimToken::Paren)), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state5(__tokens, __sym0, __sym1, __sym2, __sym3));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state5<
        __TOKENS: Iterator<Item=Result<((), LiaToken, ()),()>>,
    >(
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<((), LiaToken, ())>,
        __sym1: &mut Option<((), Ident, ())>,
        __sym2: &mut Option<((), LiaToken, ())>,
        __sym3: &mut Option<((), LiaToken, ())>,
    ) -> Result<(Option<((), LiaToken, ())>, __Nonterminal<>), __ParseError<(),LiaToken,()>>
    {
        let mut __result: (Option<((), LiaToken, ())>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((__loc1, __tok @ LiaToken::RustToken(RsToken::OpenDelim(DelimToken::Brace)), __loc2)) => {
                let mut __sym4 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state6(__tokens, __sym0, __sym1, __sym2, __sym3, __sym4));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state6<
        __TOKENS: Iterator<Item=Result<((), LiaToken, ()),()>>,
    >(
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<((), LiaToken, ())>,
        __sym1: &mut Option<((), Ident, ())>,
        __sym2: &mut Option<((), LiaToken, ())>,
        __sym3: &mut Option<((), LiaToken, ())>,
        __sym4: &mut Option<((), LiaToken, ())>,
    ) -> Result<(Option<((), LiaToken, ())>, __Nonterminal<>), __ParseError<(),LiaToken,()>>
    {
        let mut __result: (Option<((), LiaToken, ())>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((__loc1, __tok @ LiaToken::Var, __loc2)) => {
                let mut __sym5 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state8(__tokens, __sym5));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym4.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::stmt(__nt) => {
                    let __sym5 = &mut Some(__nt);
                    __result = try!(__state7(__tokens, __lookahead, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state7<
        __TOKENS: Iterator<Item=Result<((), LiaToken, ()),()>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), LiaToken, ())>,
        __sym0: &mut Option<((), LiaToken, ())>,
        __sym1: &mut Option<((), Ident, ())>,
        __sym2: &mut Option<((), LiaToken, ())>,
        __sym3: &mut Option<((), LiaToken, ())>,
        __sym4: &mut Option<((), LiaToken, ())>,
        __sym5: &mut Option<((), LiaStmt, ())>,
    ) -> Result<(Option<((), LiaToken, ())>, __Nonterminal<>), __ParseError<(),LiaToken,()>>
    {
        let mut __result: (Option<((), LiaToken, ())>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, __tok @ LiaToken::RustToken(RsToken::CloseDelim(DelimToken::Brace)), __loc2)) => {
                let mut __sym6 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state9(__tokens, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state8<
        __TOKENS: Iterator<Item=Result<((), LiaToken, ()),()>>,
    >(
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<((), LiaToken, ())>,
    ) -> Result<(Option<((), LiaToken, ())>, __Nonterminal<>), __ParseError<(),LiaToken,()>>
    {
        let mut __result: (Option<((), LiaToken, ())>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((__loc1, LiaToken::RustToken(RsToken::Ident(__tok0)), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state10(__tokens, __sym0, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state9<
        __TOKENS: Iterator<Item=Result<((), LiaToken, ()),()>>,
    >(
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<((), LiaToken, ())>,
        __sym1: &mut Option<((), Ident, ())>,
        __sym2: &mut Option<((), LiaToken, ())>,
        __sym3: &mut Option<((), LiaToken, ())>,
        __sym4: &mut Option<((), LiaToken, ())>,
        __sym5: &mut Option<((), LiaStmt, ())>,
        __sym6: &mut Option<((), LiaToken, ())>,
    ) -> Result<(Option<((), LiaToken, ())>, __Nonterminal<>), __ParseError<(),LiaToken,()>>
    {
        let mut __result: (Option<((), LiaToken, ())>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __sym4 = __sym4.take().unwrap();
                let __sym5 = __sym5.take().unwrap();
                let __sym6 = __sym6.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym6.2.clone();
                let __nt = super::__action1(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
                let __nt = __Nonterminal::fun((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state10<
        __TOKENS: Iterator<Item=Result<((), LiaToken, ()),()>>,
    >(
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<((), LiaToken, ())>,
        __sym1: &mut Option<((), Ident, ())>,
    ) -> Result<(Option<((), LiaToken, ())>, __Nonterminal<>), __ParseError<(),LiaToken,()>>
    {
        let mut __result: (Option<((), LiaToken, ())>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((__loc1, __tok @ LiaToken::RustToken(RsToken::Eq), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state11(__tokens, __sym0, __sym1, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state11<
        __TOKENS: Iterator<Item=Result<((), LiaToken, ()),()>>,
    >(
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<((), LiaToken, ())>,
        __sym1: &mut Option<((), Ident, ())>,
        __sym2: &mut Option<((), LiaToken, ())>,
    ) -> Result<(Option<((), LiaToken, ())>, __Nonterminal<>), __ParseError<(),LiaToken,()>>
    {
        let mut __result: (Option<((), LiaToken, ())>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((__loc1, LiaToken::RustToken(RsToken::Literal(Lit::Integer(__tok0), _)), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state14(__tokens, __sym3));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym2.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::atom(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state12(__tokens, __lookahead, __sym3));
                }
                __Nonterminal::expr(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state13(__tokens, __lookahead, __sym0, __sym1, __sym2, __sym3));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state12<
        __TOKENS: Iterator<Item=Result<((), LiaToken, ()),()>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), LiaToken, ())>,
        __sym0: &mut Option<((), LiaExpr, ())>,
    ) -> Result<(Option<((), LiaToken, ())>, __Nonterminal<>), __ParseError<(),LiaToken,()>>
    {
        let mut __result: (Option<((), LiaToken, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, LiaToken::RustToken(RsToken::BinOp(_)), _)) |
            Some((_, LiaToken::RustToken(RsToken::Semi), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4(__sym0);
                let __nt = __Nonterminal::expr((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state13<
        __TOKENS: Iterator<Item=Result<((), LiaToken, ()),()>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), LiaToken, ())>,
        __sym0: &mut Option<((), LiaToken, ())>,
        __sym1: &mut Option<((), Ident, ())>,
        __sym2: &mut Option<((), LiaToken, ())>,
        __sym3: &mut Option<((), LiaExpr, ())>,
    ) -> Result<(Option<((), LiaToken, ())>, __Nonterminal<>), __ParseError<(),LiaToken,()>>
    {
        let mut __result: (Option<((), LiaToken, ())>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, LiaToken::RustToken(RsToken::BinOp(__tok0)), __loc2)) => {
                let mut __sym4 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state15(__tokens, __sym3, __sym4));
            }
            Some((__loc1, __tok @ LiaToken::RustToken(RsToken::Semi), __loc2)) => {
                let mut __sym4 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state16(__tokens, __sym0, __sym1, __sym2, __sym3, __sym4));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state14<
        __TOKENS: Iterator<Item=Result<((), LiaToken, ()),()>>,
    >(
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<((), Name, ())>,
    ) -> Result<(Option<((), LiaToken, ())>, __Nonterminal<>), __ParseError<(),LiaToken,()>>
    {
        let mut __result: (Option<((), LiaToken, ())>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((_, LiaToken::RustToken(RsToken::BinOp(_)), _)) |
            Some((_, LiaToken::RustToken(RsToken::Semi), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5(__sym0);
                let __nt = __Nonterminal::atom((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state15<
        __TOKENS: Iterator<Item=Result<((), LiaToken, ()),()>>,
    >(
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<((), LiaExpr, ())>,
        __sym1: &mut Option<((), BinOpToken, ())>,
    ) -> Result<(Option<((), LiaToken, ())>, __Nonterminal<>), __ParseError<(),LiaToken,()>>
    {
        let mut __result: (Option<((), LiaToken, ())>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((__loc1, LiaToken::RustToken(RsToken::Literal(Lit::Integer(__tok0), _)), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state14(__tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::atom(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state17(__tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state16<
        __TOKENS: Iterator<Item=Result<((), LiaToken, ()),()>>,
    >(
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<((), LiaToken, ())>,
        __sym1: &mut Option<((), Ident, ())>,
        __sym2: &mut Option<((), LiaToken, ())>,
        __sym3: &mut Option<((), LiaExpr, ())>,
        __sym4: &mut Option<((), LiaToken, ())>,
    ) -> Result<(Option<((), LiaToken, ())>, __Nonterminal<>), __ParseError<(),LiaToken,()>>
    {
        let mut __result: (Option<((), LiaToken, ())>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((_, LiaToken::RustToken(RsToken::CloseDelim(DelimToken::Brace)), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __sym4 = __sym4.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym4.2.clone();
                let __nt = super::__action2(__sym0, __sym1, __sym2, __sym3, __sym4);
                let __nt = __Nonterminal::stmt((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state17<
        __TOKENS: Iterator<Item=Result<((), LiaToken, ()),()>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), LiaToken, ())>,
        __sym0: &mut Option<((), LiaExpr, ())>,
        __sym1: &mut Option<((), BinOpToken, ())>,
        __sym2: &mut Option<((), LiaExpr, ())>,
    ) -> Result<(Option<((), LiaToken, ())>, __Nonterminal<>), __ParseError<(),LiaToken,()>>
    {
        let mut __result: (Option<((), LiaToken, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, LiaToken::RustToken(RsToken::BinOp(_)), _)) |
            Some((_, LiaToken::RustToken(RsToken::Semi), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action3(__sym0, __sym1, __sym2);
                let __nt = __Nonterminal::expr((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }
}
pub use self::__parse__fun::parse_fun;

pub fn __action0<
>(
    (_, __0, _): ((), LiaFn, ()),
) -> LiaFn
{
    (__0)
}

pub fn __action1<
>(
    (_, _, _): ((), LiaToken, ()),
    (_, id, _): ((), Ident, ()),
    (_, _, _): ((), LiaToken, ()),
    (_, _, _): ((), LiaToken, ()),
    (_, _, _): ((), LiaToken, ()),
    (_, s, _): ((), LiaStmt, ()),
    (_, _, _): ((), LiaToken, ()),
) -> LiaFn
{
    LiaFn(id, s)
}

pub fn __action2<
>(
    (_, _, _): ((), LiaToken, ()),
    (_, id, _): ((), Ident, ()),
    (_, _, _): ((), LiaToken, ()),
    (_, e, _): ((), LiaExpr, ()),
    (_, _, _): ((), LiaToken, ()),
) -> LiaStmt
{
    LiaStmt::Assign(id, e)
}

pub fn __action3<
>(
    (_, e1, _): ((), LiaExpr, ()),
    (_, op, _): ((), BinOpToken, ()),
    (_, e2, _): ((), LiaExpr, ()),
) -> LiaExpr
{
    LiaExpr::Binop(op, Box::new(e1), Box::new(e2))
}

pub fn __action4<
>(
    (_, __0, _): ((), LiaExpr, ()),
) -> LiaExpr
{
    (__0)
}

pub fn __action5<
>(
    (_, n, _): ((), Name, ()),
) -> LiaExpr
{
    LiaExpr::Integer(i32::from_str(&n.as_str()).unwrap())
}

pub trait __ToTriple<> {
    type Error;
    fn to_triple(value: Self) -> Result<((),LiaToken,()),Self::Error>;
}

impl<> __ToTriple<> for LiaToken {
    type Error = ();
    fn to_triple(value: Self) -> Result<((),LiaToken,()),()> {
        Ok(((), value, ()))
    }
}
impl<> __ToTriple<> for Result<(LiaToken),()> {
    type Error = ();
    fn to_triple(value: Self) -> Result<((),LiaToken,()),()> {
        value.map(|v| ((), v, ()))
    }
}
