use value::{Value, ValueArray, print_value};

pub enum OpCode {
    Unknown = 0,
    Return = 1,
    Constant = 2,
}

impl OpCode {
    pub fn from_int(i: u8) -> OpCode {
        match i {
            1 => OpCode::Return,
            2 => OpCode::Constant,
            _ => OpCode::Unknown,
        }
    }
}

type Offset = usize;

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

    pub fn write(&mut self, byte: u8, line: u32) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn write_code(&mut self, code: OpCode, line: u32) {
        self.write(code as u8, line);
    }

    pub fn add_constant(&mut self, value: Value) -> u8 {
        let new_index = self.constants.len();
        self.constants.push(value);
        new_index as u8
    }

    pub fn disassemble(&self, name: &str) {
        println!("== {} == ", name);
        for (offset, instr) in self.code_iter() {
            self.disassemble_instruction(offset, &instr);
        }
    }

    fn disassemble_constant(&self, name: &str, instruction: &[u8]) {
        let constant_idx = instruction[1] as usize;
        print!("{} {:04} '", name, constant_idx);
        print_value(self.constants[constant_idx]);
        print!("'\n");
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
            OpCode::Return => print!("OP_RETURN\n"),
            OpCode::Constant => {
                self.disassemble_constant("OP_CONSTANT", instruction);
            }
            OpCode::Unknown => print!("Unknown opcode {:?}\n", instruction),
        }
    }

}

pub struct ChunkCodeIterator<'a> {
    chunk: &'a Chunk,
    offset: Offset,
}

impl<'a> Iterator for ChunkCodeIterator<'a> {
    type Item = (Offset, &'a[u8]);

    fn next(&mut self) -> Option<(Offset, &'a[u8])> {
        if self.offset >= self.chunk.code.len() {
            return None;
        }
        let current_offset = self.offset;

        let current_code = self.chunk.code[self.offset];
        let op_code = OpCode::from_int(current_code);
        let code_length = match op_code {
            OpCode::Return => 1,
            OpCode::Constant => 2,
            OpCode::Unknown => 1,
        };

        let arr = &self.chunk.code[self.offset..self.offset + code_length];
        self.offset += code_length;
        Some((current_offset, arr))
    }
}
