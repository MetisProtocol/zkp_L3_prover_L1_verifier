use crate::bitparser::{
    get_register_opcode, 
    get_immediate_opcode, 
    get_s_register, 
    get_t_register, 
    get_d_register, 
    get_immediate_value,
};

#[derive(Debug)]
pub enum Instruction {
    R(RegisterInstruction),
    I(ImmediateInstruction),
}

impl From<u32> for Instruction {
    fn from(v: u32) -> Self {
        let opcode = get_immediate_opcode(v);

        match opcode {
            0 => Instruction::R(RegisterInstruction::from(v)),
            _ => Instruction::I(ImmediateInstruction::from(v)),
        }
    }
}

#[derive(Debug)]
pub struct RegisterInstruction {
    pub opcode: RegisterOpcode,
    pub s_register: usize,
    pub t_register: usize,
    pub d_register: usize,
}

#[derive(Debug)]
pub enum RegisterOpcode {
  ADD,
  SUB,
  MULT,
  MULTU,
  DIV,
  DIVU,
  MFHI,
  MFLO,
  LIS,
  SLT,
  SLTU,
  JR,
  JALR,
}

impl From<u32> for RegisterInstruction {
    fn from(v: u32) -> Self {
        let opcode = get_register_opcode(v);

        Self {
            opcode: RegisterOpcode::from(opcode),
            s_register: get_s_register(v),
            t_register: get_t_register(v),
            d_register: get_d_register(v),
        }
    }
}

impl From<u8> for RegisterOpcode {
    fn from(v: u8) -> Self {
        match v {
            0x20 => RegisterOpcode::ADD,
            0x22 => RegisterOpcode::SUB,
            0x18 => RegisterOpcode::MULT,
            0x19 => RegisterOpcode::MULTU,
            0x1A => RegisterOpcode::DIV,
            0x1B => RegisterOpcode::DIVU,
            0x10 => RegisterOpcode::MFHI,
            0x12 => RegisterOpcode::MFLO,
            0x14 => RegisterOpcode::LIS,
            0x29 => RegisterOpcode::SLT,
            0x2A => RegisterOpcode::SLTU,
            0x08 => RegisterOpcode::JR,
            0x09 => RegisterOpcode::JALR,
            _ => panic!("Unrecognized opcode."),
        }
    }
}

#[derive(Debug)]
pub struct ImmediateInstruction {
    pub opcode: ImmediateOpcode,
    pub s_register: usize,
    pub t_register: usize,
    pub immediate_value: u16,
}

#[derive(Debug)]
pub enum ImmediateOpcode {
  LW,
  SW,
  BEQ,
  BNE,
}

impl From<u32> for ImmediateInstruction {
    fn from(v: u32) -> Self {
        let opcode = get_immediate_opcode(v);

        Self {
            opcode: ImmediateOpcode::from(opcode),
            s_register: get_s_register(v),
            t_register: get_t_register(v),
            immediate_value: get_immediate_value(v),
        }
    }
}

impl From<u8> for ImmediateOpcode {
    fn from(v: u8) -> Self {
        match v {
            0x23 => ImmediateOpcode::LW,
            0x2B => ImmediateOpcode::SW,
            0x04 => ImmediateOpcode::BEQ,
            0x05 => ImmediateOpcode::BNE,
            _ => panic!("Unrecognized opcode."),
        }
    }
}
