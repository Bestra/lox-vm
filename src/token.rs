#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Error,
    Eof,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub start: usize,
    pub length: usize,
    pub line: usize,
    pub t: TokenType,
    pub source: String,
}

impl Token {
    pub fn to_string(&self) -> String {
        self.source.clone()
    }

    ///
    /// The slice of the source string that represents the token
    pub fn as_slice(&self) -> &str {
        &self.source
    }

    pub fn empty() -> Token {
        Token {
            start: 0,
            length: 0,
            line: 0,
            t: TokenType::Error,
            source: String::from("Empty token"),
        }
    }
}
