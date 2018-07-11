use crate::chunk::{Chunk, OpCode};
use crate::scanner::Scanner;
use crate::token::{Token, TokenType};
use crate::value::Value;
use crate::vm::InterpretError;
use std::str::FromStr;

pub struct Compiler {
    scanner: Scanner,
    current: Token,
    previous: Token,
    had_error: bool,
    panic_mode: bool,
    chunk: Chunk,
    options: Options,
}


pub struct Options {
  debug_print_code: bool,
}

impl Options {
    pub fn debug() -> Self {
        Options {
            debug_print_code: true,
        }
    }
}

impl Default for Options {
    fn default() -> Self {
        Options {
            debug_print_code: false,
        }
    }
}



pub fn compile(source: &str, options: Options) -> Result<Chunk, InterpretError> {
    Compiler::new(source, options).compile()
}

impl Compiler {
    pub fn new(source: &str, options: Options) -> Compiler {
        Compiler {
            scanner: Scanner::new(source),
            chunk: Chunk::new(),
            had_error: false,
            panic_mode: false,
            current: Token::empty(),
            previous: Token::empty(),
            options,
        }
    }
    pub fn compile(&mut self) -> Result<Chunk, InterpretError> {
        self.advance();
        self.expression();
        self.consume(TokenType::Eof, "Expected end of file.");
        self.end_compilation();
        match self.had_error {
            false => Ok(self.chunk.clone()),
            true => Err(InterpretError::RuntimeError),
        }
    }

    pub fn emit_byte<T: Into<u8>>(&mut self, byte: T) {
        self.chunk.write(byte, self.previous.line as u32);
    }

    pub fn emit_bytes<T: Into<u8>, U: Into<u8>>(&mut self, byte_1: T, byte_2: U) {
        self.emit_byte(byte_1);
        self.emit_byte(byte_2);
    }

    pub fn end_compilation(&mut self) {
        self.emit_return();
        if self.options.debug_print_code {
          if !self.had_error {
              self.chunk.disassemble_with_iterator("Main chunk");
          }
        }
    }

    pub fn emit_return(&mut self) {
        self.emit_byte(OpCode::Return)
    }

    pub fn emit_constant(&mut self, v: Value) {
        let c = self.make_constant(v);
        self.emit_bytes(OpCode::Constant, c);
    }

    pub fn make_constant(&mut self, v: Value) -> u8 {
        let constant = self.chunk.add_constant(v);
        if constant > std::u8::MAX {
            self.error("Too many constants in one chunk");
            0
        } else {
            constant
        }
    }

    pub fn number(&mut self) {
        match f64::from_str(self.previous.as_slice()) {
            Ok(v) => self.emit_constant(v),
            Err(_) => self.error("Invalid number"),
        }
    }

    pub fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment);
    }

    pub fn grouping(&mut self) {
        self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after expression.");
    }

    pub fn unary(&mut self) {
        let operator_type = self.previous.t.clone();

        self.parse_precedence(Precedence::Unary);

        match operator_type {
            TokenType::Minus => self.emit_byte(OpCode::Negate),
            _ => panic!("Unrecognized operator"),
        }
    }

    pub fn binary(&mut self) {
        let operator_type = self.previous.t.clone();

        let rule = get_rule(&operator_type);
        let p = Precedence::from_int(rule.precedence as u8 + 1).unwrap();

        self.parse_precedence(p);

        match operator_type {
            TokenType::Plus => self.emit_byte(OpCode::Add),
            TokenType::Minus => self.emit_byte(OpCode::Subtract),
            TokenType::Star => self.emit_byte(OpCode::Multiply),
            TokenType::Slash => self.emit_byte(OpCode::Divide),
            _ => panic!("Unrecognized operator"),
        }
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        self.advance();

        let rule = get_rule(&self.previous.t);
        match rule.prefix {
            Some(prefix_rule) => self.parse_grammar_rule(prefix_rule),
            None => self.error("Expected expression.")
        }

        while precedence < get_rule(&self.current.t).precedence {
            self.advance();
            let rule = get_rule(&self.previous.t);
            match rule.infix {
                Some(infix_rule) => self.parse_grammar_rule(infix_rule),
                None => self.error("Expected expression.")
            }
        }
    }

    fn parse_grammar_rule(&mut self, rule: Grammar) {
        match rule {
            Grammar::Number => self.number(),
            Grammar::Unary => self.unary(),
            Grammar::Binary => self.binary(),
            Grammar::Grouping => self.grouping()
        }
    }

    pub fn advance(&mut self) {
        self.previous = self.current.clone();

        loop {
            // loop past any error tokens,
            // if there's an error do something with it
            self.current = self.scanner.scan_token();
            if self.current.t != TokenType::Error {
                break;
            }
            self.error_at_current(&self.current.clone().to_string())
        }
    }

    pub fn consume(&mut self, token_type: TokenType, message: &str) {
        if self.current.t == token_type {
            self.advance();
        } else {
            self.error_at_current(message);
        }
    }

    fn error_at_current(&mut self, message: &str) {
        self.error_at(&self.current.clone(), message)
    }

    fn error(&mut self, message: &str) {
        self.error_at(&self.previous.clone(), message)
    }

    fn error_at(&mut self, token: &Token, message: &str) {
        if self.panic_mode {
            return;
        }
        self.panic_mode = true;
        print!("[line {}] Error", token.line);

        match token.t {
            TokenType::Eof => print!("at end"),
            TokenType::Error => {
                () // do nothing
            }
            _ => print!("at '{}'", token.to_string()),
        }

        print!(": {}\n", message);

        self.had_error = true;
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
enum Precedence {
    None,
    Assignment, // =
    Or,         // or
    And,        // and
    Equality,   // == !=
    Comparison, // < > <= >=
    Term,       // + -
    Factor,     // * /
    Unary,      // ! - +
    Call,       // . () []
    Primary,
}

impl Precedence {
    pub fn from_int(i: u8) -> Option<Precedence> {
        match i {
            0 => Some(Precedence::None),
            2 => Some(Precedence::Assignment), // =
            3 => Some(Precedence::Or),         // or
            4 => Some(Precedence::And),        // and
            5 => Some(Precedence::Equality),   // == !=
            6 => Some(Precedence::Comparison), // < > <= >=
            7 => Some(Precedence::Term),       // + -
            8 => Some(Precedence::Factor),     // * /
            9 => Some(Precedence::Unary),      // ! - +
            10 => Some(Precedence::Call),      // . () []
            11 => Some(Precedence::Primary),
            _ => None,
        }
    }
    pub fn from_token(t: TokenType) -> Option<Precedence> {
        match t {
            _ => None,
        }
    }
}

enum Grammar {
    Grouping,
    Unary,
    Binary,
    Number,
}

struct ParseRule {
    prefix: Option<Grammar>,
    infix: Option<Grammar>,
    precedence: Precedence,
}

impl ParseRule {
   fn new(
        prefix: Option<Grammar>,
        infix: Option<Grammar>,
        precedence: Precedence,
    ) -> ParseRule {
        ParseRule {
            prefix,
            infix,
            precedence: precedence,
        }
    }
}

fn get_rule(t: &TokenType) -> ParseRule {
    use crate::token::TokenType::*;
    match t {
        LeftParen => ParseRule::new(Some(Grammar::Grouping), None, Precedence::Call), // TOKEN_LEFT_PAREN
        RightParen => ParseRule::new(None, None, Precedence::None), // TOKEN_RIGHT_PAREN
        LeftBrace => ParseRule::new(None, None, Precedence::None),  // TOKEN_LEFT_BRACE
        RightBrace => ParseRule::new(None, None, Precedence::None), // TOKEN_RIGHT_BRACE
        Comma => ParseRule::new(None, None, Precedence::None),      // TOKEN_COMMA
        Dot => ParseRule::new(None, None, Precedence::Call),        // TOKEN_DOT
        Minus => ParseRule::new(
            Some(Grammar::Unary),
            Some(Grammar::Binary),
            Precedence::Term,
        ), // TOKEN_MINUS
        Plus => ParseRule::new(None, Some(Grammar::Binary), Precedence::Term), // TOKEN_PLUS
        Semicolon => ParseRule::new(None, None, Precedence::None),  // TOKEN_SEMICOLON
        Slash => ParseRule::new(None, Some(Grammar::Binary), Precedence::Factor), // TOKEN_SLASH
        Star => ParseRule::new(None, Some(Grammar::Binary), Precedence::Factor), // TOKEN_STAR
        Bang => ParseRule::new(None, None, Precedence::None),       // TOKEN_BANG
        BangEqual => ParseRule::new(None, None, Precedence::Equality), // TOKEN_BANG_EQUAL
        Equal => ParseRule::new(None, None, Precedence::None),      // TOKEN_EQUAL
        EqualEqual => ParseRule::new(None, None, Precedence::Equality), // TOKEN_EQUAL_EQUAL
        Greater => ParseRule::new(None, None, Precedence::Comparison), // TOKEN_GREATER
        GreaterEqual => ParseRule::new(None, None, Precedence::Comparison), // TOKEN_GREATER_EQUAL
        Less => ParseRule::new(None, None, Precedence::Comparison), // TOKEN_LESS
        LessEqual => ParseRule::new(None, None, Precedence::Comparison), // TOKEN_LESS_EQUAL
        Identifier => ParseRule::new(None, None, Precedence::None), // TOKEN_IDENTIFIER
        String => ParseRule::new(None, None, Precedence::None),     // TOKEN_STRING
        Number => ParseRule::new(Some(Grammar::Number), None, Precedence::None), // TOKEN_NUMBER
        And => ParseRule::new(None, None, Precedence::And),         // TOKEN_AND
        Class => ParseRule::new(None, None, Precedence::None),      // TOKEN_CLASS
        Else => ParseRule::new(None, None, Precedence::None),       // TOKEN_ELSE
        False => ParseRule::new(None, None, Precedence::None),      // TOKEN_FALSE
        Fun => ParseRule::new(None, None, Precedence::None),        // TOKEN_FUN
        For => ParseRule::new(None, None, Precedence::None),        // TOKEN_FOR
        If => ParseRule::new(None, None, Precedence::None),         // TOKEN_IF
        Nil => ParseRule::new(None, None, Precedence::None),        // TOKEN_NIL
        Or => ParseRule::new(None, None, Precedence::Or),           // TOKEN_OR
        Print => ParseRule::new(None, None, Precedence::None),      // TOKEN_PRINT
        Return => ParseRule::new(None, None, Precedence::None),     // TOKEN_RETURN
        Super => ParseRule::new(None, None, Precedence::None),      // TOKEN_SUPER
        This => ParseRule::new(None, None, Precedence::None),       // TOKEN_THIS
        True => ParseRule::new(None, None, Precedence::None),       // TOKEN_TRUE
        Var => ParseRule::new(None, None, Precedence::None),        // TOKEN_VAR
        While => ParseRule::new(None, None, Precedence::None),      // TOKEN_WHILE
        Error => ParseRule::new(None, None, Precedence::None),      // TOKEN_ERROR
        Eof => ParseRule::new(None, None, Precedence::None),        // TOKEN_EOF
    }
}
