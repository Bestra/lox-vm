extern crate lox;

use lox::chunk::{Chunk, OpCode};
use lox::vm::{VM};
fn main() {
    let mut c = Chunk::new();
    c.write(OpCode::Constant, 123);
    let b = c.add_constant(1.2);
    c.write(b, 123);

    let constant = c.add_constant(3.4);
    c.write(OpCode::Constant, 123);
    c.write(constant, 123);

    c.write(OpCode::Add, 123);

    let constant = c.add_constant(5.6);
    c.write(OpCode::Constant, 123);
    c.write(constant, 123);

    c.write(OpCode::Divide, 123);
    c.write(OpCode::Negate, 123);

    c.write(OpCode::Return, 123);
    c.disassemble_with_iterator("test chunk");

    let mut vm = VM::new(&c);
    println!("Running");
    vm.run().unwrap();
}
