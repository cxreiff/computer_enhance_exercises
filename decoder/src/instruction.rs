use std::fmt::{self, Display};

pub enum Instruction {
    Mov { src: Location, dest: Location },
    MovImmediate { data: i16, dest: Location },
    Noop,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Mov { src, dest } => {
                write!(f, "mov {}, {}", dest, src)
            }
            Instruction::MovImmediate { data, dest } => {
                write!(f, "mov {}, {}", dest, data)
            }
            Instruction::Noop => write!(f, "noop"),
        }
    }
}

pub struct Location {
    pub register: Register,
    pub is_mem_addr: bool,
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

        if is_mem_addr {
            let mut msg = register.to_string();
            if let Some(addr_calc) = addr_calc {
                msg.push_str(&format!(" + {}", addr_calc));
            }
            if let Some(displacement) = displacement {
                if *displacement != 0b0 {
                    msg.push_str(&format!(" + {}", displacement));
                }
            }
            write!(f, "[{}]", msg)
        } else {
            write!(f, "{}", register)
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
