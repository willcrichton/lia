#![allow(unused_imports)]
use std::str::FromStr;
use syntax::parse::token::{Token, BinOpToken, Lit};
use super::ast::Expr;
extern crate lalrpop_util as __lalrpop_util;
use self::__lalrpop_util::ParseError as __ParseError;

mod __parse__expr {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use syntax::parse::token::{Token, BinOpToken, Lit};
    use super::super::ast::Expr;
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    use super::__ToTriple;
    pub fn parse_expr<
        __TOKEN: __ToTriple<Error=()>,
        __TOKENS: IntoIterator<Item=__TOKEN>,
    >(
        __tokens: __TOKENS,
    ) -> Result<Expr, __ParseError<(),Token,()>>
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
            (None, __Nonterminal::____expr((_, __nt, _))) => {
                Ok(__nt)
            }
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub enum __Nonterminal<> {
        ____expr(((), Expr, ())),
        atom(((), Expr, ())),
        expr(((), Expr, ())),
    }

    pub fn __state0<
        __TOKENS: Iterator<Item=Result<((), Token, ()),()>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), Token, ())>,
    ) -> Result<(Option<((), Token, ())>, __Nonterminal<>), __ParseError<(),Token,()>>
    {
        let mut __result: (Option<((), Token, ())>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, Token::Literal(__tok0, _), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state3(__tokens, __sym0));
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
                __Nonterminal::atom(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state1(__tokens, __lookahead, __sym0));
                }
                __Nonterminal::expr(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state2(__tokens, __lookahead, __sym0));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
    }

    pub fn __state1<
        __TOKENS: Iterator<Item=Result<((), Token, ()),()>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), Token, ())>,
        __sym0: &mut Option<((), Expr, ())>,
    ) -> Result<(Option<((), Token, ())>, __Nonterminal<>), __ParseError<(),Token,()>>
    {
        let mut __result: (Option<((), Token, ())>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, Token::BinOp(_), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2(__sym0);
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

    pub fn __state2<
        __TOKENS: Iterator<Item=Result<((), Token, ()),()>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), Token, ())>,
        __sym0: &mut Option<((), Expr, ())>,
    ) -> Result<(Option<((), Token, ())>, __Nonterminal<>), __ParseError<(),Token,()>>
    {
        let mut __result: (Option<((), Token, ())>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, Token::BinOp(__tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state4(__tokens, __sym0, __sym1));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0(__sym0);
                let __nt = __Nonterminal::____expr((
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
        return Ok(__result);
    }

    pub fn __state3<
        __TOKENS: Iterator<Item=Result<((), Token, ()),()>>,
    >(
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<((), Lit, ())>,
    ) -> Result<(Option<((), Token, ())>, __Nonterminal<>), __ParseError<(),Token,()>>
    {
        let mut __result: (Option<((), Token, ())>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            None |
            Some((_, Token::BinOp(_), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3(__sym0);
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

    pub fn __state4<
        __TOKENS: Iterator<Item=Result<((), Token, ()),()>>,
    >(
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<((), Expr, ())>,
        __sym1: &mut Option<((), BinOpToken, ())>,
    ) -> Result<(Option<((), Token, ())>, __Nonterminal<>), __ParseError<(),Token,()>>
    {
        let mut __result: (Option<((), Token, ())>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((__loc1, Token::Literal(__tok0, _), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state3(__tokens, __sym2));
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
                    __result = try!(__state5(__tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state5<
        __TOKENS: Iterator<Item=Result<((), Token, ()),()>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), Token, ())>,
        __sym0: &mut Option<((), Expr, ())>,
        __sym1: &mut Option<((), BinOpToken, ())>,
        __sym2: &mut Option<((), Expr, ())>,
    ) -> Result<(Option<((), Token, ())>, __Nonterminal<>), __ParseError<(),Token,()>>
    {
        let mut __result: (Option<((), Token, ())>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, Token::BinOp(_), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action1(__sym0, __sym1, __sym2);
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
pub use self::__parse__expr::parse_expr;

pub fn __action0<
>(
    (_, __0, _): ((), Expr, ()),
) -> Expr
{
    (__0)
}

pub fn __action1<
>(
    (_, e1, _): ((), Expr, ()),
    (_, op, _): ((), BinOpToken, ()),
    (_, e2, _): ((), Expr, ()),
) -> Expr
{
    Expr::Binop(op, Box::new(e1), Box::new(e2))
}

pub fn __action2<
>(
    (_, __0, _): ((), Expr, ()),
) -> Expr
{
    (__0)
}

pub fn __action3<
>(
    (_, n, _): ((), Lit, ()),
) -> Expr
{
    Expr::Literal(n)
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
