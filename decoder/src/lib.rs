use instruction::{Instruction, Location};

use crate::{instruction::Register, utils::blice};

mod instruction;
pub mod utils;

pub fn decode(bytes: Vec<u8>) -> Vec<Instruction> {
    let mut result: Vec<Instruction> = vec![];
    let mut bytes = bytes.iter();

    while let Some(instruction_byte) = bytes.next() {
        println!("{instruction_byte:08b}");
        result.push(match blice(instruction_byte, 0, 4) {
            0b1000 => match blice(instruction_byte, 4, 2) {
                0b10 => {
                    let d = blice(instruction_byte, 6, 1);
                    let w = blice(instruction_byte, 7, 1);

                    let register_byte = bytes.next().unwrap();
                    println!("{register_byte:08b}");

                    let reg_bits = blice(register_byte, 2, 3);
                    let r_m_bits = blice(register_byte, 5, 3);
                    let mod_bits = blice(register_byte, 0, 2);

                    let reg = decode_register_reg(&reg_bits, &w);
                    let (r_m, addr_calc) = decode_register_r_m(&r_m_bits, &w, &mod_bits);

                    let reg = Location {
                        register: reg,
                        is_mem_addr: false,
                        addr_calc: None,
                        displacement: None,
                    };
                    let r_m = Location {
                        register: r_m,
                        is_mem_addr: mod_bits != 0b11,
                        addr_calc,
                        displacement: None,
                    };

                    let (mut src, dest) = if d == 0b1 { (r_m, reg) } else { (reg, r_m) };

                    match mod_bits {
                        0b01 => {
                            let lo = bytes.next().unwrap();
                            println!("{lo:08b}");
                            let displacement = i8::from_be_bytes([*lo]) as i16;
                            src.displacement = Some(displacement);
                        }
                        0b10 => {
                            let lo = bytes.next().unwrap();
                            println!("{lo:08b}");
                            let hi = bytes.next().unwrap();
                            println!("{hi:08b}");
                            let displacement = i16::from_be_bytes([*hi, *lo]);
                            src.displacement = Some(displacement);
                        }
                        _ => {}
                    };

                    Instruction::Mov { src, dest }
                }
                _ => Instruction::Noop,
            },
            0b1011 => {
                let w = blice(instruction_byte, 4, 1);
                let reg_bits = blice(instruction_byte, 5, 3);

                let reg = decode_register_reg(&reg_bits, &w);
                let reg = Location {
                    register: reg,
                    is_mem_addr: false,
                    addr_calc: None,
                    displacement: None,
                };

                let data_word = if w == 0b0 {
                    i8::from_be_bytes([*bytes.next().unwrap()]) as i16
                } else {
                    let left = bytes.next().unwrap();
                    let right = bytes.next().unwrap();
                    i16::from_be_bytes([*right, *left])
                };

                Instruction::MovImmediate {
                    data: data_word,
                    dest: reg,
                }
            }
            _ => Instruction::Noop,
        });
        println!("{}", result.last().unwrap());
        println!();
    }

    result
}

fn decode_register_r_m(bits: &u8, w: &u8, mod_bits: &u8) -> (Register, Option<Register>) {
    match mod_bits {
        0b11 => (decode_register_reg(bits, w), None),
        _ => decode_register_mem(bits),
    }
}

fn decode_register_mem(bits: &u8) -> (Register, Option<Register>) {
    match bits {
        0b000 => (Register::BX, Some(Register::SI)),
        0b001 => (Register::BX, Some(Register::DI)),
        0b010 => (Register::BP, Some(Register::SI)),
        0b011 => (Register::BP, Some(Register::DI)),
        0b100 => (Register::SI, None),
        0b101 => (Register::DI, None),
        0b110 => (Register::BP, None),
        0b111 => (Register::BX, None),
        _ => unreachable!(),
    }
}

fn decode_register_reg(bits: &u8, w: &u8) -> Register {
    match (bits, w) {
        (0b000u8, 0b1) => Register::AX,
        (0b001u8, 0b1) => Register::CX,
        (0b010u8, 0b1) => Register::DX,
        (0b011u8, 0b1) => Register::BX,
        (0b100u8, 0b1) => Register::SP,
        (0b101u8, 0b1) => Register::BP,
        (0b110u8, 0b1) => Register::SI,
        (0b111u8, 0b1) => Register::DI,
        (0b000u8, 0b0) => Register::AL,
        (0b001u8, 0b0) => Register::CL,
        (0b010u8, 0b0) => Register::DL,
        (0b011u8, 0b0) => Register::BL,
        (0b100u8, 0b0) => Register::AH,
        (0b101u8, 0b0) => Register::CH,
        (0b110u8, 0b0) => Register::DH,
        (0b111u8, 0b0) => Register::BH,
        _ => unreachable!(),
    }
}
