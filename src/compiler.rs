#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
enum TokenType {
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

struct Token<'t> {
    start: usize,
    length: usize,
    line: usize,
    t: TokenType,
    source: &'t str,
}
struct Scanner<'a> {
    start: usize,
    current: usize,
    line: usize,
    source: &'a str,
}

impl Scanner<'a> {
    pub fn new(source: &'a str) -> Scanner {
        Scanner {
            start: 0,
            current: 0,
            line: 1,
            source: source,
        }
    }

    pub fn current_line(&self) -> usize {
        self.current
    }

    // pub fn is_at_end_mut(&mut self) -> bool {
    //     self.current >= self.source.len()
    // }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();
        self.start = self.current;
        if self.is_at_end() {
            return self.make_token(TokenType::Eof);
        }

        match self.advance() {
            '(' => self.make_token(TokenType::LeftParen),
            ')' => self.make_token(TokenType::RightParen),
            '{' => self.make_token(TokenType::LeftBrace),
            '}' => self.make_token(TokenType::RightBrace),
            ';' => self.make_token(TokenType::Semicolon),
            ',' => self.make_token(TokenType::Comma),
            '.' => self.make_token(TokenType::Dot),
            '-' => self.make_token(TokenType::Minus),
            '+' => self.make_token(TokenType::Plus),
            '/' => self.make_token(TokenType::Slash),
            '*' => self.make_token(TokenType::Star),
            '!' => {
                if self.consume_match('=') {
                    self.make_token(TokenType::BangEqual)
                } else {
                    self.make_token(TokenType::Bang)
                }
            }
            '=' => {
                if self.consume_match('=') {
                    self.make_token(TokenType::EqualEqual)
                } else {
                    self.make_token(TokenType::Equal)
                }
            }
            '<' => {
                if self.consume_match('=') {
                    self.make_token(TokenType::LessEqual)
                } else {
                    self.make_token(TokenType::Less)
                }
            }
            '>' => {
                if self.consume_match('=') {
                    self.make_token(TokenType::GreaterEqual)
                } else {
                    self.make_token(TokenType::Greater)
                }
            }
            _ => self.error_token("Unexpected character."),
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.peek() {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                },
                _ => return,
            };
        }
    }

    pub fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }

    pub fn consume_match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.peek() != expected {
            return false;
        }

        self.advance();
        true
    }

    pub fn peek(&self) -> char {
        self.source.chars().nth(self.current).unwrap()
    }

    // pub fn peek_mut(&mut self) -> char {
    //     self.source.chars().nth(self.current).unwrap()
    // }

    fn make_token(&mut self, t: TokenType) -> Token {
        Token {
            t: t,
            start: self.start,
            length: self.current - self.start,
            line: self.line,
            source: self.source,
        }
    }

    fn error_token(&mut self, message: &'static str) -> Token {
        Token {
            t: TokenType::Error,
            start: 0,
            length: message.len(),
            line: self.line,
            source: message,
        }
    }
}

pub fn compile(source: &str) {
    let mut scanner = Scanner::new(source);
    loop {
        let current_line = scanner.current_line();
        let token = scanner.scan_token();

        if token.line != current_line {
            print!("{}", token.line);
        } else {
            print!("|");
        }
    }
}

#[test]
fn scanner_peek_and_advance() {
    let mut s = Scanner::new("{}");
    assert_eq!(s.peek(), '{');
    assert_eq!(s.advance(), '{');
    assert_eq!(s.peek(), '}');
    assert_eq!(s.advance(), '}');
}

#[test]
fn scanner_scan_token() {
    let mut s = Scanner::new("{}");
    assert_eq!(s.scan_token().t, TokenType::LeftBrace);
    assert_eq!(s.scan_token().t, TokenType::RightBrace);
}

#[test]
fn scanner_skip_whitespace() {
    let mut s = Scanner::new(" { } ");
    assert_eq!(s.scan_token().t, TokenType::LeftBrace);
    assert_eq!(s.scan_token().t, TokenType::RightBrace);
}

#[test]
fn scanner_skip_whitespace_newline() {
    let mut s = Scanner::new(" { \n } ");
    assert_eq!(s.scan_token().line, 1);
    assert_eq!(s.scan_token().line, 2);
}

#[test]
fn scanner_scan_token_with_equals() {
    let mut s = Scanner::new("!=<");
    assert_eq!(s.scan_token().t, TokenType::BangEqual);
    assert_eq!(s.scan_token().t, TokenType::Less);
}
