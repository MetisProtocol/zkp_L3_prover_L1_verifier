use regex::Regex;

use crate::constants::INSTRUCTION_TABLE;
use crate::datum::{find_datum, Datum};
use crate::instruction::{convert_opcode_to_format, Instruction, InstructionFormat};
use crate::label::{find_label, Label};
use crate::utils::{convert_int_to_binary, convert_string_to_int, get_address_difference};

#[derive(Clone)]
enum ArgumentType {
    NUMBER,
    REGISTER,
    LABEL,
    STACK,
}

pub struct Text {
    rs: i32,
    rt: i32,
    rd: i32,
    shamt: i32,
    funct: i32,
    opcode: i32,
    immediate: i32,
    address: i32,
}

impl Text {
    pub fn new(
        rs: i32,
        rt: i32,
        rd: i32,
        shamt: i32,
        funct: i32,
        opcode: i32,
        immediate: i32,
        address: i32,
    ) -> Self {
        Self {
            rs,
            rt,
            rd,
            shamt,
            funct,
            opcode,
            immediate,
            address,
        }
    }

    pub fn to_binary(&self) -> String {
        match convert_opcode_to_format(self.opcode) {
            InstructionFormat::REGISTER => format!(
                "{}{}{}{}{}{}",
                convert_int_to_binary(self.opcode, 6),
                convert_int_to_binary(self.rs, 5),
                convert_int_to_binary(self.rt, 5),
                convert_int_to_binary(self.rd, 5),
                convert_int_to_binary(self.shamt, 5),
                convert_int_to_binary(self.funct, 6),
            ),
            InstructionFormat::IMMEDIATE => format!(
                "{}{}{}{}",
                convert_int_to_binary(self.opcode, 6),
                convert_int_to_binary(self.rs, 5),
                convert_int_to_binary(self.rt, 5),
                convert_int_to_binary(self.immediate, 16),
            ),
            InstructionFormat::JUMP => format!(
                "{}{}",
                convert_int_to_binary(self.opcode, 6),
                convert_int_to_binary(self.address, 26),
            ),
            InstructionFormat::PSEUDO => panic!("A pseudo instruction found."),
        }
    }
}

pub fn get_text_from_code(
    text: &str,
    current_address: i32,
    data: &[Datum],
    labels: &[Label],
) -> Text {
    if let [name, arguments] = text.trim_start().split('\t').collect::<Vec<&str>>()[..] {
        let instruction = INSTRUCTION_TABLE.get(name).expect("Unknown instruction.");

        let argument_texts = arguments
            .split(',')
            .map(|arg| arg.trim())
            .collect::<Vec<&str>>();

        let arguments = resolve_arguments(&argument_texts, &data, &labels);

        get_text_by_format(&instruction, &arguments, current_address)
    } else {
        panic!("Invalid instruction.");
    }
}

fn get_text_by_format(instruction: &Instruction, arguments: &[i32], current_address: i32) -> Text {
    let first_arg = *arguments.get(0).unwrap_or(&0);
    let second_arg = *arguments.get(1).unwrap_or(&0);
    let third_arg = *arguments.get(2).unwrap_or(&0);

    match convert_opcode_to_format(instruction.opcode) {
        InstructionFormat::REGISTER => {
            if instruction.is_shift() {
                instruction.to_register_format_text(0, second_arg, first_arg, third_arg)
            } else if instruction.is_register_jump() {
                instruction.to_register_format_text(first_arg, 0, 0, 0)
            } else {
                instruction.to_register_format_text(second_arg, third_arg, first_arg, 0)
            }
        }
        InstructionFormat::JUMP => instruction.to_jump_format_text(first_arg >> 2),
        InstructionFormat::IMMEDIATE => {
            if arguments.len() < 3 {
                instruction.to_immediate_format_text(0, first_arg, second_arg)
            } else if instruction.is_branch() {
                let difference = get_address_difference(current_address, third_arg);
                instruction.to_immediate_format_text(first_arg, second_arg, difference)
            } else {
                instruction.to_immediate_format_text(second_arg, first_arg, third_arg)
            }
        }
        InstructionFormat::PSEUDO => panic!("A pseudo instruction found."),
    }
}

fn resolve_arguments(argument_codes: &[&str], data: &[Datum], labels: &[Label]) -> Vec<i32> {
    argument_codes
        .iter()
        .flat_map(|argument_text| match resolve_argument_type(argument_text) {
            ArgumentType::NUMBER => vec![convert_string_to_int(argument_text)],
            ArgumentType::REGISTER => vec![convert_string_to_int(
                &argument_text[1..argument_text.len()],
            )],
            ArgumentType::LABEL => {
                if let Some(datum) = find_datum(argument_text, &data) {
                    vec![datum.address]
                } else if let Some(label) = find_label(argument_text, &labels) {
                    vec![label.address]
                } else {
                    panic!("Failed to resolve argument value.");
                }
            }
            ArgumentType::STACK => {
                if let [offset, base] = argument_text.split('(').collect::<Vec<&str>>()[..] {
                    let base = convert_string_to_int(&base[1..(base.len() - 1)]);
                    let offset = convert_string_to_int(offset);

                    vec![base, offset]
                } else {
                    panic!("Failed to resolve argument value.");
                }
            }
        })
        .collect()
}

fn resolve_argument_type(text: &str) -> ArgumentType {
    let arguments = [
        (Regex::new(r"^\$\d*").unwrap(), ArgumentType::REGISTER),
        (Regex::new(r"^[a-z]\w*").unwrap(), ArgumentType::LABEL),
        (Regex::new(r"^-?\d+\(\$\d*\)").unwrap(), ArgumentType::STACK),
        (Regex::new(r"^(0x)?\d*").unwrap(), ArgumentType::NUMBER),
    ];

    arguments
        .iter()
        .find(|arg| arg.0.is_match(text))
        .expect("Failed to resolve argument.")
        .clone()
        .1
}
