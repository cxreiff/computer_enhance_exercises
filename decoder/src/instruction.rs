use std::{
    cmp::Ordering,
    fmt::{self, Display},
};

pub enum Instruction {
    Mov { src: Location, dest: Location },
    MovImmediate { data: Immediate, dest: Location },
    Noop,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Mov { src, dest } => {
                write!(f, "mov {}, {}", dest, src)
            }
            Instruction::MovImmediate { data, dest } => {
                let data_string = if dest.is_mem_addr {
                    match data {
                        Immediate::Byte(data) => {
                            format!("byte {}", data)
                        }
                        Immediate::Word(data) => {
                            format!("word {}", data)
                        }
                    }
                } else {
                    match data {
                        Immediate::Byte(data) => {
                            format!("{}", data)
                        }
                        Immediate::Word(data) => {
                            format!("{}", data)
                        }
                    }
                };

                write!(f, "mov {}, {}", dest, data_string)
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
            return write!(f, "[{}]", direct_address);
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
