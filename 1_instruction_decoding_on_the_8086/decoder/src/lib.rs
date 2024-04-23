use instruction::Instruction;
use itertools::Itertools;

use crate::instruction::Register;

mod instruction;

pub fn decode(bytes: Vec<u8>) -> Vec<Instruction> {
    bytes.iter().tuples().map(decode_op).collect()
}

fn decode_op((first, second): (&u8, &u8)) -> Instruction {
    match first {
        0b10001001u8 => {
            let (src, dest) = decode_registers(second);
            Instruction::Mov(src, false, dest, false)
        }
        _ => Instruction::Noop,
    }
}

fn decode_registers(byte: &u8) -> (Register, Register) {
    const SRC_MASK: u8 = 0b00111000u8;
    const DEST_MASK: u8 = 0b00000111u8;
    let src = match byte & SRC_MASK {
        0b00000000u8 => Register::AX,
        0b00011000u8 => Register::BX,
        0b00111000u8 => Register::DI,
        _ => unreachable!(),
    };
    let dest = match byte & DEST_MASK {
        0b00000000u8 => Register::AX,
        0b00000001u8 => Register::CX,
        0b00000111u8 => Register::DI,
        _ => unreachable!(),
    };
    (src, dest)
}
