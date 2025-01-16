use std::{
    cmp::Ordering,
    fmt::{self, Display},
};

pub enum Instruction {
    Mov { src: Location, dest: Location },
    MovImmediate { data: Immediate, dest: Location },

    Add { src: Location, dest: Location },
    AddImmediate { data: Immediate, dest: Location },

    Sub { src: Location, dest: Location },
    SubImmediate { data: Immediate, dest: Location },

    Cmp { src: Location, dest: Location },
    CmpImmediate { data: Immediate, dest: Location },

    Je { increment: Immediate },
    Jl { increment: Immediate },
    Jle { increment: Immediate },
    Jb { increment: Immediate },
    Jbe { increment: Immediate },
    Jp { increment: Immediate },
    Jo { increment: Immediate },
    Js { increment: Immediate },
    Jne { increment: Immediate },
    Jnl { increment: Immediate },
    Jnle { increment: Immediate },
    Jnb { increment: Immediate },
    Jnbe { increment: Immediate },
    Jnp { increment: Immediate },
    Jno { increment: Immediate },
    Jns { increment: Immediate },
    Loop { increment: Immediate },
    Loopz { increment: Immediate },
    Loopnz { increment: Immediate },
    Jcxz { increment: Immediate },

    Noop,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Mov { src, dest } => {
                write!(f, "mov {}, {}", dest, src)
            }
            Instruction::MovImmediate { data, dest } => {
                write!(
                    f,
                    "mov {}, {}",
                    dest,
                    string_for_immediate(data, Some(dest))
                )
            }
            Instruction::Add { src, dest } => {
                write!(f, "add {}, {}", dest, src)
            }
            Instruction::AddImmediate { data, dest } => {
                write!(
                    f,
                    "add {}, {}",
                    dest,
                    string_for_immediate(data, Some(dest))
                )
            }
            Instruction::Sub { src, dest } => {
                write!(f, "sub {}, {}", dest, src)
            }
            Instruction::SubImmediate { data, dest } => {
                write!(
                    f,
                    "sub {}, {}",
                    dest,
                    string_for_immediate(data, Some(dest))
                )
            }
            Instruction::Cmp { src, dest } => {
                write!(f, "cmp {}, {}", dest, src)
            }
            Instruction::CmpImmediate { data, dest } => {
                write!(
                    f,
                    "cmp {}, {}",
                    dest,
                    string_for_immediate(data, Some(dest))
                )
            }
            Instruction::Je { increment } => {
                write!(f, "je {}", string_for_immediate(increment, None))
            }
            Instruction::Jl { increment } => {
                write!(f, "jl {}", string_for_immediate(increment, None))
            }
            Instruction::Jle { increment } => {
                write!(f, "jle {}", string_for_immediate(increment, None))
            }
            Instruction::Jb { increment } => {
                write!(f, "jb {}", string_for_immediate(increment, None))
            }
            Instruction::Jbe { increment } => {
                write!(f, "jbe {}", string_for_immediate(increment, None))
            }
            Instruction::Jp { increment } => {
                write!(f, "jp {}", string_for_immediate(increment, None))
            }
            Instruction::Jo { increment } => {
                write!(f, "jo {}", string_for_immediate(increment, None))
            }
            Instruction::Js { increment } => {
                write!(f, "js {}", string_for_immediate(increment, None))
            }
            Instruction::Jne { increment } => {
                write!(f, "jne {}", string_for_immediate(increment, None))
            }
            Instruction::Jnl { increment } => {
                write!(f, "jnl {}", string_for_immediate(increment, None))
            }
            Instruction::Jnle { increment } => {
                write!(f, "jnle {}", string_for_immediate(increment, None))
            }
            Instruction::Jnb { increment } => {
                write!(f, "jnb {}", string_for_immediate(increment, None))
            }
            Instruction::Jnbe { increment } => {
                write!(f, "jnbe {}", string_for_immediate(increment, None))
            }
            Instruction::Jnp { increment } => {
                write!(f, "jnp {}", string_for_immediate(increment, None))
            }
            Instruction::Jno { increment } => {
                write!(f, "jno {}", string_for_immediate(increment, None))
            }

            Instruction::Jns { increment } => {
                write!(f, "jns {}", string_for_immediate(increment, None))
            }

            Instruction::Loop { increment } => {
                write!(f, "loop {}", string_for_immediate(increment, None))
            }

            Instruction::Loopz { increment } => {
                write!(f, "loopz {}", string_for_immediate(increment, None))
            }

            Instruction::Loopnz { increment } => {
                write!(f, "loopnz {}", string_for_immediate(increment, None))
            }

            Instruction::Jcxz { increment } => {
                write!(f, "jcxz {}", string_for_immediate(increment, None))
            }
            Instruction::Noop => write!(f, "noop"),
        }
    }
}

#[derive(Debug)]
pub struct Location {
    pub is_mem_addr: bool,
    pub register: Option<Register>,
    pub addr_calc: Option<Register>,
    pub displacement: Option<i16>,
}

impl Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Location {
            ref register,
            is_mem_addr,
            ref addr_calc,
            ref displacement,
        } = *self;

        if let Some(register) = register {
            if is_mem_addr {
                let mut msg = register.to_string();
                if let Some(addr_calc) = addr_calc {
                    msg.push_str(&format!(" + {}", addr_calc));
                }
                if let Some(displacement) = displacement {
                    match displacement.cmp(&0b0) {
                        Ordering::Less => {
                            msg.push_str(&format!(" - {}", -displacement));
                        }
                        Ordering::Greater => {
                            msg.push_str(&format!(" + {}", displacement));
                        }
                        Ordering::Equal => {}
                    };
                }
                write!(f, "[{}]", msg)
            } else {
                write!(f, "{}", register)
            }
        } else {
            let direct_address = displacement.expect("displacement required for direct address");
            write!(f, "[{}]", direct_address)
        }
    }
}

#[derive(Debug)]
pub enum Register {
    AL,
    CL,
    DL,
    BL,
    AH,
    CH,
    DH,
    BH,
    AX,
    CX,
    BX,
    DX,
    SP,
    BP,
    SI,
    DI,
}

impl Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

pub enum Immediate {
    Byte(i8),
    Word(i16),
}

fn string_for_immediate(data: &Immediate, dest: Option<&Location>) -> String {
    if let Some(Location { is_mem_addr, .. }) = dest {
        if *is_mem_addr {
            return match data {
                Immediate::Byte(data) => {
                    format!("byte {}", data)
                }
                Immediate::Word(data) => {
                    format!("word {}", data)
                }
            };
        }
    }

    match data {
        Immediate::Byte(data) => {
            format!("{}", data)
        }
        Immediate::Word(data) => {
            format!("{}", data)
        }
    }
}
