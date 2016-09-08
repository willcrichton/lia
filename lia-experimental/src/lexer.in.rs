use token::Token;
use std::str::FromStr;

macro_rules! some { ($x:expr) => { |_: &mut Lexer<R>| -> Option<Token> { Some($x) } } }
macro_rules! none { () => { |_: &mut Lexer<R>| -> Option<Token> { None } } }

rustlex! Lexer {
    property brace_depth:u32 = 0;

    let WHITESPACE = [' ' '\n' '\t'];
    let ID = ['a'-'z''A'-'Z''_']['a'-'z''A'-'Z''_''0'-'9']*;
    let NUM = '0' | ['1'-'9']['0'-'9']*;

    INITIAL {
        . => |lexer: &mut Lexer<R>| -> Option<Token> { panic!("unexpected token {:?}", lexer.yystr()) },

        WHITESPACE => none!(),

        ID => |lexer: &mut Lexer<R>| Some(Token::Id(lexer.yystr().clone())),
        NUM => |lexer: &mut Lexer<R>| Some(Token::Int(i32::from_str(&lexer.yystr()).unwrap())),

        '(' => some!(Token::Lparen),
        ')' => some!(Token::Rparen),
        '{' => some!(Token::Lbrace),
        '}' => some!(Token::Rbrace),

        ';' => some!(Token::Semi),
        '=' => some!(Token::Eq),
        '+' => some!(Token::Plus),
        "=>" => some!(Token::Arrow),

        "fn" => some!(Token::Fun),
        "let" => some!(Token::Let),

        "$rs {" => |lexer: &mut Lexer<R>| -> Option<Token>{
            lexer.brace_depth += 1;
            lexer.QUOTE();
            Some(Token::Lbrace)
        },
    }

    QUOTE {
        [^'{''}']+ => |lexer: &mut Lexer<R>| Some(Token::QuoteChar(lexer.yystr())),

        '}' => |lexer: &mut Lexer<R>| -> Option<Token> {
            lexer.brace_depth -= 1;
            if lexer.brace_depth == 0 {
                lexer.INITIAL();
                Some(Token::Rbrace)
            } else {
                Some(Token::QuoteChar("}".to_string()))
            }
        },

        '{' => |lexer: &mut Lexer<R>| -> Option<Token> {
            lexer.brace_depth += 1;
            Some(Token::QuoteChar("{".to_string()))
        }
    }
}
