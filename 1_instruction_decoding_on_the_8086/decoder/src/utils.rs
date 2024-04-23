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
