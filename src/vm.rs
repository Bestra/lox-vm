use std::fmt;
use chunk::{Chunk, OpCode};
use value::{Value, print_value};

const STACK_MAX: usize = 256;

#[derive(Debug)]
pub enum InterpretError {
    CompileError,
    RuntimeError,
}

struct OperandStack {
    stack: [Value; STACK_MAX],
    stack_top: usize,
}

impl OperandStack {
    pub fn new() -> OperandStack {
        OperandStack {
            stack: [0.0; 256],
            stack_top: 0,
        }
    }

    pub fn push(&mut self, value: Value) {
        self.stack[self.stack_top] = value;
        self.stack_top += 1;
    }

    pub fn pop(&mut self) -> Value {
        self.stack_top -= 1;
        self.stack[self.stack_top]
    }

    pub fn len(&self) -> usize {
        self.stack_top
    }
}

impl fmt::Debug for OperandStack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        for i in 0..self.stack_top {
            write!(f, "{}, ", self.stack[i])?;
        }
        write!(f, "]")
    }
}

pub struct VM<'v> {
    chunk: &'v Chunk,
    ip: usize,
    stack: OperandStack,
}

impl<'v> VM<'v> {
    pub fn new(chunk: &'v Chunk) -> VM {
        VM {
            chunk: chunk,
            ip: 0,
            stack: OperandStack::new(),
        }
    }

    fn read_byte(&mut self) -> u8 {
        let instruction = self.chunk.code[self.ip];
        self.ip += 1;
        instruction
    }

    fn binary_op<F>(&mut self, perform: F)
    where
        F: Fn(Value, Value) -> Value,
    {
        assert!(
            self.stack.len() >= 2,
            "stack needs at least 2 values to perform binary operation"
        );

        let b = self.stack.pop();
        let a = self.stack.pop();
        self.stack.push(perform(a, b));
    }

    fn read_constant(&mut self) -> Value {
        let offset = self.read_byte() as usize;
        self.chunk.constants[offset]
    }

    fn print_debug_info(&self) {
      println!("stack: {:?}", self.stack)
    }

    pub fn run(&mut self) -> Result<(), InterpretError> {
        loop {
            let instruction = self.chunk.code[self.ip];
            self.ip += 1;

            self.print_debug_info();

            match OpCode::from_int(instruction) {
                OpCode::Constant => {
                    let c = self.read_constant();
                    self.stack.push(c);
                }
                OpCode::Return => {
                    print_value(self.stack.pop());
                    return Ok(());
                }
                OpCode::Negate => {
                    let c = self.stack.pop();
                    self.stack.push(-c);
                }

                OpCode::Add => {
                    self.binary_op(|a, b| a + b);
                },
                OpCode::Subtract => {
                    self.binary_op(|a, b| a - b);
                },
                OpCode::Multiply => {
                    self.binary_op(|a, b| a * b);
                },
                OpCode::Divide => {
                    self.binary_op(|a, b| a / b);
                },
                OpCode::Unknown => return Err(InterpretError::RuntimeError)
            }
        }
    }
}
