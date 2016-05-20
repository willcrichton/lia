use syntax::parse::token::Token;

#[derive(Debug)]
pub enum LiaToken {
    RustToken(Token),
    Var,
    Function,
    Return,
}

impl LiaToken {
    pub fn from_rust_token(t: Token) -> LiaToken {
        if let Token::Ident(ident) = t {
            let s = ident.name.as_str();

            // No better way to go from InternedString -> &str?
            match unsafe { s.slice_unchecked(0, s.len()) } {
                "function" => LiaToken::Function,
                "var" => LiaToken::Var,
                "return" => LiaToken::Return,
                _ => LiaToken::RustToken(t)
            }
        } else {
            LiaToken::RustToken(t)
        }
    }
}
