use crate::token::{Token, TokenType};
pub struct Scanner<'a> {
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
            source,
        }
    }

    pub fn current_line(&self) -> usize {
        self.current
    }

    pub fn current_str(&self) -> &str {
        &self.source[self.start..self.current]
    }

    pub fn peek(&self) -> char {
        self.source.chars().nth(self.current).unwrap_or('\0')
    }

    pub fn peek_next(&self) -> Option<char> {
        self.source.chars().nth(self.current + 1)
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn scan_token(&mut self) -> Token<'a> {
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
            '"' => self.string(),
            c if is_digit(c) => self.number(),
            c if is_alpha(c) => self.identifier(),
            _ => self.error_token("Unexpected character."),
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            println!("skip whitespace loop");
            match self.peek() {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                '/' => {
                    if let Some('/') = self.peek_next() {
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        return
                    }
                }
                _ => return,
            };
        }
    }

    fn advance(&mut self) -> char {
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

    // pub fn peek_mut(&mut self) -> char {
    //     self.source.chars().nth(self.current).unwrap()
    // }

    fn make_token(&mut self, t: TokenType) -> Token<'a> {
        Token {
            t: t,
            start: self.start,
            length: self.current - self.start,
            line: self.line,
            source: &self.source[self.start..self.current],
        }
    }

    fn error_token(&mut self, message: &'static str) -> Token<'static> {
        Token {
            t: TokenType::Error,
            start: 0,
            length: message.len(),
            line: self.line,
            source: message,
        }
    }

    // Methods dealing with grammer start here

    pub fn string(&mut self) -> Token<'a> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return self.error_token("Unterminated string.");
        }

        // consume the closing '"'
        assert_eq!(self.advance(), '"');
        self.make_token(TokenType::String)
    }

    pub fn number(&mut self) -> Token<'a> {
        while is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && is_digit(self.peek_next().unwrap()) {
            // consume the '.'
            self.advance();
        }

        while is_digit(self.peek()) {
            self.advance();
        }

        self.make_token(TokenType::Number)
    }

    pub fn identifier(&mut self) -> Token<'a> {
        while is_alpha(self.peek()) || is_digit(self.peek()) {
            self.advance();
        }

        let t = self.identifier_type();

        self.make_token(t)
    }

    pub fn identifier_type(&self) -> TokenType {
        match self.current_str() {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "if" => TokenType::If,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier,
        }
    }
}

pub fn is_digit(c: char) -> bool {
    match c {
        '0'...'9' => true,
        _ => false,
    }
}

fn is_alpha(c: char) -> bool {
    match c {
        's'...'z' | 'S'...'Z' | '_' => true,
        _ => false,
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

#[test]
fn scanner_scan_string() {
    let mut s = Scanner::new("\"Hey\"");
    let t = s.scan_token();
    assert_eq!(t.start, 0);
    assert_eq!(t.length, 5);
    assert_eq!(t.to_string(), "\"Hey\"");
}

#[test]
fn scanner_current_str() {
    let mut s = Scanner::new("{}");
    s.advance();
    s.advance();
    assert_eq!(s.current_str(), "{}")
}
