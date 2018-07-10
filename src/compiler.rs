enum TokenType {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    ERROR,
    EOF,
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

impl<'a> Scanner<'a> {
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

    pub fn is_at_end(&mut self) -> bool {
      self.current >= self.source.len()
    }

    pub fn scan_token(&mut self) -> Token {
        self.start = self.current;
        if self.is_at_end() {
            self.make_token(TokenType::EOF)
        } else {
            self.error_token("Unexpected character.")
        }
    }

    fn make_token(&mut self, t: TokenType) -> Token {
        Token {
            t: t,
            start: self.start,
            length: self.current - self.start,
            line: self.line,
            source: self.source
        }
    }

    fn error_token(&mut self, message: &'static str) -> Token {
        Token {
            t: TokenType::ERROR,
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
