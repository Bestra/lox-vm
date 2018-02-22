extern crate lox;

use lox::chunk::{Chunk, OpCode};
fn main() {
    let mut c = Chunk::new();
    c.write_code(OpCode::Constant, 123);
    let a = c.add_constant(2.5);
    c.write(a, 123);
    c.write_code(OpCode::Constant, 123);
    let b = c.add_constant(512373.2);
    c.write(b, 123);
    c.write_code(OpCode::Return, 123);
    c.write_code(OpCode::Return, 7);
    c.disassemble("test chunk");

}
