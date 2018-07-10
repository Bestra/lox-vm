use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use lox::interpreter::interpret;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(file_path) => interpret_file(file_path),
        None => repl(),
    }
}

fn repl() {
    println!("Lox Repl");
    loop {
        let input = rprompt::prompt_reply_stdout(">").unwrap();
        interpret(&input);
    }
}

fn interpret_file(path: &str) {
    let file = File::open(path).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    interpret(&contents);
}
