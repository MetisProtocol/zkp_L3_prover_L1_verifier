use std::collections::HashMap;

use crate::{instruction::{
    Instruction, 
    RegisterInstruction, 
    ImmediateInstruction, 
    RegisterOpcode, 
    ImmediateOpcode,
}, inputreader::read_input};

const STDOUT_MEMORY_ADDRESS: usize = 0xFFFF000C;
const STDIN_MEMORY_ADDRESS: usize = 0xFFFF0004;

#[derive(Debug)]
pub struct VM {
    registers: [i32; 32],
    hi: i32,
    lo: i32,
    pc: usize,
    program: Vec<u32>,
    memory: HashMap<usize, u32>,
}

impl VM {
    pub fn new(program: Vec<u32>) -> VM {
        VM {
            registers: [0; 32],
            hi: 0,
            lo: 0,
            program,
            pc: 0,
            memory: HashMap::new(),
        }
    }

    /// Loops as long as instructions can be executed.
    pub fn run(&mut self) {
        self.registers[31] = self.program.len() as i32;
        let mut is_done = false;
        while !is_done {
            is_done = self.execute_instruction();
        }
    }


    fn execute_instruction(&mut self) -> bool {
        if self.pc >= self.program.len() {
            return true;
        }

        match self.decode_instruction() {
            Instruction::R(inst) => self.execute_register_instruction(inst),
            Instruction::I(inst) => self.execute_immediate_instruction(inst),
        }
        false
    }

    fn execute_register_instruction(&mut self, instruction: RegisterInstruction) {
        match instruction.opcode {
            RegisterOpcode::ADD => {
                let register1 = self.registers[instruction.s_register];
                let register2 = self.registers[instruction.t_register];
                self.registers[instruction.d_register] = register1 + register2;
            },
            RegisterOpcode::SUB => {
                let register1 = self.registers[instruction.s_register];
                let register2 = self.registers[instruction.t_register];
                self.registers[instruction.d_register] = register1 - register2;
            },
            RegisterOpcode::MULT => {
                let register1 = self.registers[instruction.s_register] as i64;
                let register2 = self.registers[instruction.t_register] as i64;
                let product = register1 * register2;
                self.lo = product as i32;
                self.hi = (product >> 32) as i32;
            },
            RegisterOpcode::MULTU => {
                let register1 = self.registers[instruction.s_register] as u64;
                let register2 = self.registers[instruction.t_register] as u64;
                let product = register1 * register2;
                self.lo = product as i32;
                self.hi = (product >> 32) as i32;
            },
            RegisterOpcode::DIV => {
                let register1 = self.registers[instruction.s_register];
                let register2 = self.registers[instruction.t_register];
                self.lo = register1 / register2;
                self.hi = register1 % register2;
            },
            RegisterOpcode::DIVU => {
                let register1 = self.registers[instruction.s_register] as u32;
                let register2 = self.registers[instruction.t_register] as u32;
                self.lo = (register1 / register2) as i32;
                self.hi = (register1 % register2) as i32;
            },
            RegisterOpcode::MFHI => {
                self.registers[instruction.d_register] = self.hi;
            },
            RegisterOpcode::MFLO => {
                self.registers[instruction.d_register] = self.lo;
            },
            RegisterOpcode::LIS => {
                self.registers[instruction.d_register] = self.program[self.pc] as i32;
                self.pc += 1;
            },
            RegisterOpcode::SLT => {
                let register1 = self.registers[instruction.s_register];
                let register2 = self.registers[instruction.t_register];
                self.registers[instruction.d_register] = if register1 < register2 {
                    1
                } else {
                    0
                }
            },
            RegisterOpcode::SLTU => {
                let register1 = self.registers[instruction.s_register] as u32;
                let register2 = self.registers[instruction.t_register] as u32;
                self.registers[instruction.d_register] = if register1 < register2 {
                    1
                } else {
                    0
                }
            },
            RegisterOpcode::JR => {
                self.pc = self.registers[instruction.s_register] as usize;
            },
            RegisterOpcode::JALR => {
                self.registers[31] = self.pc as i32;
                self.pc = self.registers[instruction.s_register] as usize;
            },
        }
    }

    fn execute_immediate_instruction(&mut self, instruction: ImmediateInstruction) {
        match instruction.opcode {
            ImmediateOpcode::LW => {
                let s_register = self.registers[instruction.s_register] as usize;
                let immediate_value = instruction.immediate_value as usize;
                let memory_address = s_register + immediate_value;
                if memory_address == STDIN_MEMORY_ADDRESS {
                    self.registers[instruction.t_register] = read_input();
                } else {
                    self.registers[instruction.t_register] = *self.memory.entry(memory_address).or_insert(0) as i32;
                }
            },
            ImmediateOpcode::SW => {
                let s_register = self.registers[instruction.s_register] as usize;
                let immediate_value = instruction.immediate_value as usize;
                let memory_address = s_register + immediate_value;
                let t_register = self.registers[instruction.t_register] as u32;
                if memory_address == STDOUT_MEMORY_ADDRESS {
                    match std::char::from_u32(t_register) {
                        Some(ch) => print!("{}", ch),
                        None => println!("Unknown character: {}", t_register),
                    }
                } else {
                    self.memory.insert(memory_address, t_register);
                }
            },
            ImmediateOpcode::BEQ => {
                let register1 = self.registers[instruction.s_register];
                let register2 = self.registers[instruction.t_register];
                if register1 == register2 {
                    self.pc += (instruction.immediate_value as usize) * 4;
                }
            },
            ImmediateOpcode::BNE => {
                let register1 = self.registers[instruction.s_register];
                let register2 = self.registers[instruction.t_register];
                if register1 != register2 {
                    self.pc += (instruction.immediate_value as usize) * 4;
                }
            },
        }
    }

    fn decode_instruction(&mut self) -> Instruction {
        let instruction = self.program[self.pc];
        self.pc += 1;
        return Instruction::from(instruction);
    }

    pub fn print_registers(&self) {
        for (index, register) in self.registers.iter().enumerate() {
            println!("${:02}: {}",index, register);
        }
    }
}
