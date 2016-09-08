#[derive(Debug)]
pub enum Token {
    Let,
    Eq,
    Semi,
    Plus,
    Int(i32),
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Fun,
    Arrow,
    Id(String),
    Quote(String)
}
