extern crate lox;

use lox::chunk::{Chunk, OpCode};
fn main() {
    println!("Hello, world!");

    let mut c = Chunk::new();
    c.write_code(OpCode::Return);
    c.write_code(OpCode::Return);
    c.write_code(OpCode::Return);
    c.write_code(OpCode::Return);
    c.disassemble("test chunk");

}
