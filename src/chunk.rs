use crate::value::{print_value, Value, ValueArray};

#[repr(u8)]
pub enum OpCode {
    Unknown = 0,
    Return = 1,
    Constant = 2,
    Negate = 3,
    Add = 4,
    Subtract = 5,
    Multiply = 6,
    Divide = 7,
    Nil = 8,
    True = 9,
    False = 10,
}

impl Into<u8> for OpCode {
    fn into(self) -> u8 {
        self as u8
    }
}

impl OpCode {
    pub fn from_int(i: u8) -> OpCode {
        match i {
            1 => OpCode::Return,
            2 => OpCode::Constant,
            3 => OpCode::Negate,
            4 => OpCode::Add,
            5 => OpCode::Subtract,
            6 => OpCode::Multiply,
            7 => OpCode::Divide,
            8 => OpCode::Nil,
            9 => OpCode::True,
            10 => OpCode::False,
            _ => OpCode::Unknown,
        }
    }

    pub fn code_length(&self) -> usize {
        match *self {
            OpCode::Return => 1,
            OpCode::Constant => 2,
            OpCode::Negate => 1,
            OpCode::Unknown => 1,
            OpCode::Add => 1,
            OpCode::Subtract => 1,
            OpCode::Multiply => 1,
            OpCode::Divide => 1,
            OpCode::Nil => 1,
            OpCode::True => 1,
            OpCode::False => 1,
        }
    }
}

type Offset = usize;

#[derive(Clone)]
pub struct Chunk {
    pub code: Vec<u8>,
    pub lines: Vec<u32>,
    pub constants: ValueArray,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            // TODO: use with_capacity(8) here?
            code: Vec::new(),
            constants: ValueArray::new(),
            lines: Vec::new(),
        }
    }

    pub fn code_iter(&self) -> ChunkCodeIterator {
        ChunkCodeIterator {
            offset: 0,
            chunk: &self,
        }
    }

    pub fn write<T: Into<u8>>(&mut self, byte: T, line: u32) {
        self.code.push(byte.into());
        self.lines.push(line);
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        let new_index = self.constants.len();
        self.constants.push(value);
        new_index
    }

    pub fn disassemble_with_iterator(&self, name: &str) {
        println!("== {} == ", name);
        for (offset, instr) in self.code_iter() {
            self.disassemble_instruction(offset, &instr);
        }
    }

    fn disassemble_constant(&self, name: &str, instruction: &[u8]) {
        let constant_idx = instruction[1] as usize;
        print!("{} {:04} '", name, constant_idx);
        print_value(self.constants[constant_idx]);
        println!();
    }

    fn disassemble_instruction(&self, offset: usize, instruction: &[u8]) {
        print!("{:04} ", offset);

        if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
            print!("   | ");
        } else {
            print!("{:04} ", self.lines[offset]);
        }

        let op_byte = instruction[0];
        match OpCode::from_int(op_byte) {
            OpCode::Return => println!("OP_RETURN"),
            OpCode::Negate => println!("OP_NEGATE"),
            OpCode::Add => println!("OP_ADD"),
            OpCode::Subtract => println!("OP_SUBTRACT"),
            OpCode::Multiply => println!("OP_MULTIPLY"),
            OpCode::Divide => println!("OP_DIVIDE"),
            OpCode::Constant => {
                self.disassemble_constant("OP_CONSTANT", instruction);
            }
            OpCode::Nil => println!("OP_NIL"),
            OpCode::True => println!("OP_TRUE"),
            OpCode::False => println!("OP_FALSE"),
            OpCode::Unknown => println!("Unknown opcode {:?}", instruction),
        }
    }
}

pub struct ChunkCodeIterator<'a> {
    chunk: &'a Chunk,
    offset: Offset,
}

impl<'a> Iterator for ChunkCodeIterator<'a> {
    type Item = (Offset, &'a [u8]);

    fn next(&mut self) -> Option<(Offset, &'a [u8])> {
        if self.offset >= self.chunk.code.len() {
            return None;
        }
        let current_offset = self.offset;

        let current_code = self.chunk.code[self.offset];
        let op_code = OpCode::from_int(current_code);
        let code_length = op_code.code_length();

        let arr = &self.chunk.code[self.offset..self.offset + code_length];
        self.offset += code_length;
        Some((current_offset, arr))
    }
}
