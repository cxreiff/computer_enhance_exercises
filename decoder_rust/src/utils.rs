use std::fmt::{self, Display};

pub struct PrintVec<T>(pub Vec<T>)
where
    T: Display;

impl<T> Display for PrintVec<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for elem in self.0.iter() {
            writeln!(f, "{}", elem)?;
        }
        Ok(())
    }
}

pub fn blice(byte: &u8, start: u8, width: u8) -> u8 {
    let byte = byte >> (8 - width - start);
    byte & match width {
        1 => 0b1,
        2 => 0b11,
        3 => 0b111,
        4 => 0b1111,
        5 => 0b11111,
        6 => 0b111111,
        7 => 0b1111111,
        8 => 0b11111111,
        _ => panic!(),
    }
}
