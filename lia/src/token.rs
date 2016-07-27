use syntax::parse::token::{Token as RsToken};
use syntax::tokenstream::TokenTree;

#[derive(Debug, Clone)]
pub enum LiaToken {
    RustToken(RsToken),
    Var,
    Function,
    Return,
    If,
    Else,
    While,
    For,
    True,
    False,
    Quote(Vec<TokenTree>),
    Rust,
}

impl LiaToken {
    pub fn from_rust_token(t: RsToken) -> LiaToken {
        if let RsToken::Ident(ident) = t {
            let s = ident.name.as_str();

            // No better way to go from InternedString -> &str?
            match unsafe { s.slice_unchecked(0, s.len()) } {
                "function" => LiaToken::Function,
                "var" => LiaToken::Var,
                "return" => LiaToken::Return,
                "if" => LiaToken::If,
                "else" => LiaToken::Else,
                "while" => LiaToken::While,
                "for" => LiaToken::For,
                "true" => LiaToken::True,
                "false" => LiaToken::False,
                "rust" => LiaToken::Rust,
                _ => LiaToken::RustToken(t)
            }
        } else {
            LiaToken::RustToken(t)
        }
    }
}
