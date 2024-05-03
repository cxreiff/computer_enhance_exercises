use instruction::{Immediate, Instruction, Location};

use crate::{instruction::Register, utils::blice};

mod instruction;
pub mod utils;

pub fn decode(bytes: Vec<u8>) -> Vec<Instruction> {
    let mut result: Vec<Instruction> = vec![];
    let mut bytes = bytes.iter();

    let mut next_byte = || {
        let byte = bytes.next();

        #[cfg(debug_assertions)]
        if let Some(byte) = byte {
            println!("{byte:08b}");
        }

        byte
    };

    while let Some(instruction_byte) = next_byte() {
        result.push(match blice(instruction_byte, 0, 4) {
            0b1000 => match blice(instruction_byte, 4, 2) {
                0b10 => {
                    let d = blice(instruction_byte, 6, 1);
                    let w = blice(instruction_byte, 7, 1);

                    let register_byte = next_byte().unwrap();

                    let mod_bits = blice(register_byte, 0, 2);
                    let reg_bits = blice(register_byte, 2, 3);
                    let r_m_bits = blice(register_byte, 5, 3);

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
                        displacement: decode_displacement(&mut next_byte, &mod_bits, &r_m_bits),
                    };

                    let (src, dest) = if d == 0b1 { (r_m, reg) } else { (reg, r_m) };

                    Instruction::Mov { src, dest }
                }
                _ => Instruction::Noop,
            },
            0b1010 => {
                let w = blice(instruction_byte, 7, 1);
                let accumulator = Location {
                    is_mem_addr: false,
                    register: Some(Register::AX),
                    addr_calc: None,
                    displacement: None,
                };
                let mem = Location {
                    is_mem_addr: true,
                    register: None,
                    addr_calc: None,
                    displacement: decode_addr(&mut next_byte, &w),
                };
                match blice(instruction_byte, 4, 3) {
                    0b000 => Instruction::Mov {
                        src: mem,
                        dest: accumulator,
                    },
                    0b001 => Instruction::Mov {
                        src: accumulator,
                        dest: mem,
                    },
                    _ => panic!(),
                }
            }
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

                let data = decode_data(&mut next_byte, &w);

                Instruction::MovImmediate { data, dest: reg }
            }
            0b1100 => match blice(instruction_byte, 4, 3) {
                0b011 => {
                    let w = blice(instruction_byte, 7, 1);

                    let register_byte = next_byte().unwrap();

                    let mod_bits = blice(register_byte, 0, 2);
                    let r_m_bits = blice(register_byte, 5, 3);

                    let (r_m, addr_calc) = decode_register_r_m(&r_m_bits, &w, &mod_bits);

                    let dest = Location {
                        register: r_m,
                        is_mem_addr: mod_bits != 0b11,
                        addr_calc,
                        displacement: decode_displacement(&mut next_byte, &mod_bits, &r_m_bits),
                    };

                    let data = decode_data(&mut next_byte, &w);

                    Instruction::MovImmediate { data, dest }
                }
                _ => Instruction::Noop,
            },
            _ => Instruction::Noop,
        });

        #[cfg(debug_assertions)]
        println!("{}", result.last().unwrap());
    }

    result
}

fn decode_register_r_m(
    r_m_bits: &u8,
    w: &u8,
    mod_bits: &u8,
) -> (Option<Register>, Option<Register>) {
    match mod_bits {
        0b00 if *r_m_bits == 0b110 => (None, None),
        0b11 => (decode_register_reg(r_m_bits, w), None),
        _ => decode_register_mem(r_m_bits),
    }
}

fn decode_register_mem(bits: &u8) -> (Option<Register>, Option<Register>) {
    match bits {
        0b000 => (Some(Register::BX), Some(Register::SI)),
        0b001 => (Some(Register::BX), Some(Register::DI)),
        0b010 => (Some(Register::BP), Some(Register::SI)),
        0b011 => (Some(Register::BP), Some(Register::DI)),
        0b100 => (Some(Register::SI), None),
        0b101 => (Some(Register::DI), None),
        0b110 => (Some(Register::BP), None),
        0b111 => (Some(Register::BX), None),
        _ => unreachable!(),
    }
}

fn decode_register_reg(bits: &u8, w: &u8) -> Option<Register> {
    match (bits, w) {
        (0b000u8, 0b1) => Some(Register::AX),
        (0b001u8, 0b1) => Some(Register::CX),
        (0b010u8, 0b1) => Some(Register::DX),
        (0b011u8, 0b1) => Some(Register::BX),
        (0b100u8, 0b1) => Some(Register::SP),
        (0b101u8, 0b1) => Some(Register::BP),
        (0b110u8, 0b1) => Some(Register::SI),
        (0b111u8, 0b1) => Some(Register::DI),
        (0b000u8, 0b0) => Some(Register::AL),
        (0b001u8, 0b0) => Some(Register::CL),
        (0b010u8, 0b0) => Some(Register::DL),
        (0b011u8, 0b0) => Some(Register::BL),
        (0b100u8, 0b0) => Some(Register::AH),
        (0b101u8, 0b0) => Some(Register::CH),
        (0b110u8, 0b0) => Some(Register::DH),
        (0b111u8, 0b0) => Some(Register::BH),
        _ => unreachable!(),
    }
}

fn decode_data<'a>(next_byte: &mut impl FnMut() -> Option<&'a u8>, w: &u8) -> Immediate {
    if *w == 0b0 {
        let lo = next_byte().unwrap();
        Immediate::Byte(i8::from_be_bytes([*lo]))
    } else {
        let lo = next_byte().unwrap();
        let hi = next_byte().unwrap();
        Immediate::Word(i16::from_be_bytes([*hi, *lo]))
    }
}

fn decode_addr<'a>(next_byte: &mut impl FnMut() -> Option<&'a u8>, w: &u8) -> Option<i16> {
    if *w == 0b0 {
        let lo = next_byte().unwrap();
        Some(u8::from_be_bytes([*lo]) as i16)
    } else {
        let lo = next_byte().unwrap();
        let hi = next_byte().unwrap();
        Some(u16::from_be_bytes([*hi, *lo]) as i16)
    }
}

fn decode_displacement<'a>(
    next_byte: &mut impl FnMut() -> Option<&'a u8>,
    mod_bits: &u8,
    r_m_bits: &u8,
) -> Option<i16> {
    match mod_bits {
        0b00 if *r_m_bits == 0b110 => {
            let lo = next_byte().unwrap();
            let hi = next_byte().unwrap();
            let displacement = i16::from_be_bytes([*hi, *lo]);
            Some(displacement)
        }
        0b01 => {
            let lo = next_byte().unwrap();
            let displacement = i8::from_be_bytes([*lo]) as i16;
            Some(displacement)
        }
        0b10 => {
            let lo = next_byte().unwrap();
            let hi = next_byte().unwrap();
            let displacement = i16::from_be_bytes([*hi, *lo]);
            Some(displacement)
        }
        _ => None,
    }
}
