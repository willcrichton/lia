use token::{Token as Tok};
use std::str::FromStr;
use mark::{Mark, Marked};

type Token = Marked<Tok>;

macro_rules! mark {
    ($lexer:ident, $e:expr) => {
        Some(Marked {
            node: $e,
            mark: Mark {
                lo: $lexer._input.tok.off as u32,
                hi: $lexer._input.pos.off as u32
            }
        })
    }
}

macro_rules! some { ($x:expr) => { |lexer: &mut Lexer<R>| -> Option<Token> {
    mark!(lexer, $x)
} } }
macro_rules! none { () => { |_: &mut Lexer<R>| -> Option<Token> { None } } }

rustlex! Lexer {
    property depth: u32 = 0;

    let WHITESPACE = [' ' '\n' '\t'];
    let ID = ['a'-'z''A'-'Z''_']['a'-'z''A'-'Z''_''0'-'9']*;
    let NUM = '0' | ['1'-'9']['0'-'9']*;
    let STRING = '"' .* '"';

    INITIAL {
        . => |lexer: &mut Lexer<R>| -> Option<Token> {
            panic!("unexpected token {:?}", lexer.yystr())
        },

        WHITESPACE => none!(),

        ID => |lexer: &mut Lexer<R>| {
            mark!(lexer, Tok::Id(lexer.yystr().clone()))
        }
        NUM => |lexer: &mut Lexer<R>| {
            mark!(lexer, Tok::Int(i32::from_str(&lexer.yystr()).unwrap()))
        }
        STRING => |lexer: &mut Lexer<R>| {
            let s = lexer.yystr();
            mark!(lexer, Tok::String((&s[1..(s.len()-1)]).to_string()))
        }

        '(' => some!(Tok::Lparen),
        ')' => some!(Tok::Rparen),

        '{' => |lexer: &mut Lexer<R>| -> Option<Token> {
            mark!(lexer, Tok::Lbrace)
        },

        '}' => |lexer: &mut Lexer<R>| -> Option<Token> {
            mark!(lexer, Tok::Rbrace)
        },

        '.' => some!(Tok::Dot),
        ',' => some!(Tok::Comma),
        ':' => some!(Tok::Colon),
        ';' => some!(Tok::Semi),
        '=' => some!(Tok::Eq),
        '+' => some!(Tok::Plus),
        "=>" => some!(Tok::FatArrow),
        "->" => some!(Tok::ThinArrow),

        "fn" => some!(Tok::Fun),
        "let" => some!(Tok::Let),
        "type" => some!(Tok::Type),

        "$rs" WHITESPACE* => |lexer: &mut Lexer<R>| -> Option<Token> {
            lexer.QUOTE();
            mark!(lexer, Tok::QuoteMarker)
        },
    }

    QUOTE {
        [^'{''}''$']+ => |lexer: &mut Lexer<R>| {
            mark!(lexer, Tok::QuoteChar(lexer.yystr()))
        }

        '}' => |lexer: &mut Lexer<R>| -> Option<Token> {
            lexer.depth -= 1;
            mark!(lexer, if lexer.depth == 0 {
                lexer.INITIAL();
                Tok::Rbrace
            } else {
                Tok::QuoteChar("}".to_string())
            })
        },

        '{' => |lexer: &mut Lexer<R>| -> Option<Token> {
            let tok = if lexer.depth == 0 {
                Tok::Lbrace
            } else {
                Tok::QuoteChar("{".to_string())
            };
            lexer.depth += 1;
            mark!(lexer, tok)
        },

        '$' . => |_: &mut Lexer<R>| -> Option<Token> { panic!("bad splice") }

        '$' ID => |lexer: &mut Lexer<R>| -> Option<Token> {
            let s = lexer.yystr();
            mark!(lexer, Tok::Splice((&s[1..s.len()]).to_string()))
        },

        // "${" => |lexer: &mut Lexer<R>| -> Option<Token> {
        //     {
        //         let mut depth = &mut lexer.depth[lexer.depth.len()-1];
        //         *depth += 1;
        //     }
        //     lexer.INITIAL();
        //     Some(Tok::SpliceStart)
        // }
    }
}
