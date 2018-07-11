use crate::compiler::{compile, Options};
use crate::vm::{InterpretError, VM};
pub fn interpret(source: &str) -> Result<(), InterpretError> {
    println!("interpreting source {}", source);

    let chunk = compile(source, Options::debug())?;
    let mut vm = VM::new(&chunk);
    vm.run()
}
