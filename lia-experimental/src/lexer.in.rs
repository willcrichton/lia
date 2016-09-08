use token::Token;
use std::str::FromStr;

macro_rules! some { ($x:expr) => { |_: &mut Lexer<R>| -> Option<Token> { Some($x) } } }
macro_rules! none { () => { |_: &mut Lexer<R>| -> Option<Token> { None } } }

rustlex! Lexer {
    let WHITESPACE = [' ' '\n' '\t'];
    let ID = ['a'-'z''A'-'Z''_']['a'-'z''A'-'Z''_''0'-'9']*;
    let NUM = '0' | ['1'-'9']['0'-'9']*;

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

    '%' .* '%' => |lexer: &mut Lexer<R>| {
        let s = lexer.yystr().clone();

        Some(Token::Quote(s[1..s.len()-1].to_string()))
    },
}
