use std::collections::HashMap;

use crate::opcode::Opcode;

const STACK_SIZE: usize = 1024;

pub struct Vm {
    sp: u64,
    ip: u64,
    mp: u64,
    stack: [u64; STACK_SIZE],
    labels: HashMap<u64, u64>,
    code: Vec<u8>,
    memory: Vec<u8>,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            sp: 0,
            ip: 0,
            mp: 0,
            stack: [0; STACK_SIZE],
            labels: HashMap::new(),
            code: Vec::new(),
            memory: Vec::new(),
        }
    }

    fn consume_u8(&mut self) -> u8 {
        let value = self.code[self.ip as usize];
        self.ip += 1;
        value
    }

    fn imm_u64(&mut self) -> u64 {
        let bytes = [
            self.consume_u8(),
            self.consume_u8(),
            self.consume_u8(),
            self.consume_u8(),
            self.consume_u8(),
            self.consume_u8(),
            self.consume_u8(),
            self.consume_u8(),
        ];
        u64::from_ne_bytes(bytes)
    }

    fn consume_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.consume_u8());
        opcode
    }

    fn goto(&mut self) {
        let label = self.imm_u64();
        self.ip = self.labels[&label];
    }

    fn push(&mut self, value: u64) {
        self.stack[self.sp as usize] = value;
        self.sp += 1;
    }

    fn pop(&mut self) -> u64 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }

    // push immediate value to stack
    fn imm(&mut self) {
        let value = self.imm_u64();
        self.push(value);
    }

    pub fn run(&mut self) {
        while self.ip < self.code.len() as u64 {
            let opcode = self.consume_opcode();
            match opcode {
                Opcode::Goto => self.goto(),
                Opcode::UIntImm => self.imm(),
                Opcode::IntImm => self.imm(),
                Opcode::FloatImm => self.imm(),
                Opcode::True => self.push(1),
                Opcode::False => self.push(0),
                _ => unimplemented!(),
            }
        }
    }
}
