use token::Token;
use std::str::FromStr;

macro_rules! some { ($x:expr) => { |_: &mut Lexer<R>| -> Option<Token> { Some($x) } } }
macro_rules! none { () => { |_: &mut Lexer<R>| -> Option<Token> { None } } }


rustlex! Lexer {
    property depth:u32 = 0;

    let WHITESPACE = [' ' '\n' '\t'];
    let ID = ['a'-'z''A'-'Z''_']['a'-'z''A'-'Z''_''0'-'9']*;
    let NUM = '0' | ['1'-'9']['0'-'9']*;
    let STRING = '"' .* '"';

    INITIAL {
        . => |lexer: &mut Lexer<R>| -> Option<Token> { panic!("unexpected token {:?}", lexer.yystr()) },

        WHITESPACE => none!(),

        ID => |lexer: &mut Lexer<R>| Some(Token::Id(lexer.yystr().clone())),
        NUM => |lexer: &mut Lexer<R>| Some(Token::Int(i32::from_str(&lexer.yystr()).unwrap())),
        STRING => |lexer: &mut Lexer<R>| Some(Token::String(lexer.yystr())),

        '(' => some!(Token::Lparen),
        ')' => some!(Token::Rparen),

        '{' => |lexer: &mut Lexer<R>| -> Option<Token> {
            Some(Token::Lbrace)
        },

        '}' => |lexer: &mut Lexer<R>| -> Option<Token> {
            Some(Token::Rbrace)
        },

        ';' => some!(Token::Semi),
        '=' => some!(Token::Eq),
        '+' => some!(Token::Plus),
        "=>" => some!(Token::Arrow),

        "fn" => some!(Token::Fun),
        "let" => some!(Token::Let),

        "$rs" WHITESPACE* => |lexer: &mut Lexer<R>| -> Option<Token> {
            lexer.QUOTE();
            Some(Token::QuoteMarker)
        },
    }

    QUOTE {
        [^'{''}''$']+ => |lexer: &mut Lexer<R>| Some(Token::QuoteChar(lexer.yystr())),

        '}' => |lexer: &mut Lexer<R>| -> Option<Token> {
            lexer.depth -= 1;
            if lexer.depth == 0 {
                lexer.INITIAL();
                Some(Token::Rbrace)
            } else {
                Some(Token::QuoteChar("}".to_string()))
            }
        },

        '{' => |lexer: &mut Lexer<R>| -> Option<Token> {
            let tok = if lexer.depth == 0 {
                Some(Token::Lbrace)
            } else {
                Some(Token::QuoteChar("{".to_string()))
            };
            lexer.depth += 1;
            tok
        },

        '$' . => |lexer: &mut Lexer<R>| -> Option<Token> { panic!("bad splice") }

        '$' ID => |lexer: &mut Lexer<R>| -> Option<Token> {
            let s = lexer.yystr();
            Some(Token::Splice((&s[1..s.len()]).to_string()))
        },

        // "${" => |lexer: &mut Lexer<R>| -> Option<Token> {
        //     {
        //         let mut depth = &mut lexer.depth[lexer.depth.len()-1];
        //         *depth += 1;
        //     }
        //     lexer.INITIAL();
        //     Some(Token::SpliceStart)
        // }
    }
}
