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
            0b0000 | 0b0010 | 0b0011 => match blice(instruction_byte, 5, 1) {
                0b0 => {
                    let d = blice(instruction_byte, 6, 1);
                    let w = blice(instruction_byte, 7, 1);

                    let (src, dest) = decode_mod_reg_rm(&mut next_byte, &d, &w);

                    match blice(instruction_byte, 2, 3) {
                        0b000 => Instruction::Add { src, dest },
                        0b101 => Instruction::Sub { src, dest },
                        0b111 => Instruction::Cmp { src, dest },
                        _ => Instruction::Noop,
                    }
                }
                0b1 => match blice(instruction_byte, 6, 1) {
                    0b0 => {
                        let w = blice(instruction_byte, 7, 1);

                        let (data, dest) = decode_accum_immediate(&mut next_byte, &w);

                        match blice(instruction_byte, 2, 3) {
                            0b000 => Instruction::AddImmediate { data, dest },
                            0b101 => Instruction::SubImmediate { data, dest },
                            0b111 => Instruction::CmpImmediate { data, dest },
                            _ => Instruction::Noop,
                        }
                    }
                    _ => Instruction::Noop,
                },
                _ => Instruction::Noop,
            },
            0b1000 => match blice(instruction_byte, 4, 2) {
                0b00 => {
                    let s = blice(instruction_byte, 6, 1);
                    let w = blice(instruction_byte, 7, 1);

                    let (data, dest, ident) = decode_mod_rm(&mut next_byte, &s, &w);

                    match ident {
                        0b000 => Instruction::AddImmediate { data, dest },
                        0b101 => Instruction::SubImmediate { data, dest },
                        0b111 => Instruction::CmpImmediate { data, dest },
                        _ => panic!(),
                    }
                }
                0b10 => {
                    let d = blice(instruction_byte, 6, 1);
                    let w = blice(instruction_byte, 7, 1);

                    let (src, dest) = decode_mod_reg_rm(&mut next_byte, &d, &w);

                    Instruction::Mov { src, dest }
                }
                _ => Instruction::Noop,
            },
            0b1010 => {
                let ident = blice(instruction_byte, 4, 3);
                let w = blice(instruction_byte, 7, 1);

                let (src, dest) = decode_accum_mem(&mut next_byte, &ident, &w);

                Instruction::Mov { src, dest }
            }
            0b1011 => {
                let w = blice(instruction_byte, 4, 1);
                let reg_bits = blice(instruction_byte, 5, 3);

                let reg = decode_register_reg(&reg_bits, &w);
                let dest = Location {
                    register: reg,
                    is_mem_addr: false,
                    addr_calc: None,
                    displacement: None,
                };

                let data = decode_data(&mut next_byte, &0, &w);

                Instruction::MovImmediate { data, dest }
            }
            0b1100 => match blice(instruction_byte, 4, 3) {
                0b011 => {
                    let w = blice(instruction_byte, 7, 1);

                    let (data, dest, _) = decode_mod_rm(&mut next_byte, &0, &w);

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

fn decode_mod_reg_rm<'a>(
    next_byte: &mut impl FnMut() -> Option<&'a u8>,
    d: &u8,
    w: &u8,
) -> (Location, Location) {
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
        displacement: decode_displacement(next_byte, &mod_bits, &r_m_bits),
    };

    if *d == 0b1 {
        (r_m, reg)
    } else {
        (reg, r_m)
    }
}

fn decode_mod_rm<'a>(
    next_byte: &mut impl FnMut() -> Option<&'a u8>,
    s: &u8,
    w: &u8,
) -> (Immediate, Location, u8) {
    let register_byte = next_byte().unwrap();

    let mod_bits = blice(register_byte, 0, 2);
    let ident = blice(register_byte, 2, 3);
    let r_m_bits = blice(register_byte, 5, 3);

    let (r_m, addr_calc) = decode_register_r_m(&r_m_bits, &w, &mod_bits);

    let dest = Location {
        register: r_m,
        is_mem_addr: mod_bits != 0b11,
        addr_calc,
        displacement: decode_displacement(next_byte, &mod_bits, &r_m_bits),
    };

    let data = decode_data(next_byte, &s, &w);

    (data, dest, ident)
}

fn decode_accum_mem<'a>(
    next_byte: &mut impl FnMut() -> Option<&'a u8>,
    ident: &u8,
    w: &u8,
) -> (Location, Location) {
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
        displacement: decode_addr(next_byte, w),
    };

    match *ident {
        0b000 => (mem, accumulator),
        0b001 => (accumulator, mem),
        _ => panic!(),
    }
}

fn decode_accum_immediate<'a>(
    next_byte: &mut impl FnMut() -> Option<&'a u8>,
    w: &u8,
) -> (Immediate, Location) {
    let accumulator = Location {
        is_mem_addr: false,
        register: Some(if *w == 0b0 {
            Register::AL
        } else {
            Register::AX
        }),
        addr_calc: None,
        displacement: None,
    };
    let data = decode_data(next_byte, &0, &w);

    (data, accumulator)
}

fn decode_data<'a>(next_byte: &mut impl FnMut() -> Option<&'a u8>, s: &u8, w: &u8) -> Immediate {
    if *w == 0b0 {
        let lo = next_byte().unwrap();
        Immediate::Byte(i8::from_be_bytes([*lo]))
    } else if *s == 0b1 {
        let lo = next_byte().unwrap();
        Immediate::Word(i16::from_be_bytes([0, *lo]))
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
