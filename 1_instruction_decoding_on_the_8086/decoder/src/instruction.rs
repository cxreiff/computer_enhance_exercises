use std::fmt::{self, Display};

#[derive(Debug)]
pub enum Instruction {
    Mov(Register, bool, Register, bool),
    Noop,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Mov(src, _, dest, _) => write!(f, "mov {}, {}", dest, src),
            Instruction::Noop => write!(f, "noop"),
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
