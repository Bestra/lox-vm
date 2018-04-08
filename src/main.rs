extern crate lox;

use lox::chunk::{Chunk, OpCode};
use lox::value::{Value};
fn main() {
    let mut c = Chunk::new();
    c.write_code(OpCode::Constant, 123);
    let a = c.add_constant(2.5);
    c.write(a, 123);
    c.write_code(OpCode::Constant, 123);
    let b = c.add_constant(512373.2);
    c.write(b, 123);
    c.write_code(OpCode::Return, 123);
    c.disassemble("test chunk");

    let mut vm = VM::new(&c);
    println!("Running");
    vm.run();
}

enum InterpretError {
    CompileError,
    RuntimeError,
}

struct VM<'v> {
    chunk: &'v Chunk,
    ip: usize,
}

impl<'v> VM<'v> {
    fn new(chunk: &'v Chunk) -> VM {
        VM {
            chunk: chunk,
            ip: 0,
        }
    }

    fn read_byte(&mut self) -> u8 {
        let instruction = self.chunk.code[self.ip];
        self.ip += 1;
        instruction
    }

    fn read_constant(&mut self) -> Value {
        let offset = self.read_byte() as usize;
        self.chunk.constants[offset]
    }

    fn run(&mut self) -> Result<(), InterpretError> {
        loop {
            let instruction = self.chunk.code[self.ip];
            self.ip += 1;

            match OpCode::from_int(instruction) {
                OpCode::Constant => {
                    let c = self.read_constant();
                    println!("{}", c)

                }
                OpCode::Return => return Ok(()),
                _ => ()
            }
        }
    }
}
