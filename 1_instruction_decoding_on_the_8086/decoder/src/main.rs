use std::{io, process::Command};

use clap::Parser;
use decoder::decode;

use crate::utils::PrintVec;

mod utils;

#[derive(Parser)]
struct Args {
    asm: String,
}

fn main() -> Result<(), io::Error> {
    let Args { asm } = Args::parse();

    Command::new("nasm").arg(&asm).output()?;

    let output = match asm.strip_suffix(".asm") {
        Some(s) => s,
        None => &asm,
    };

    let bytes = std::fs::read(output).unwrap();

    let instructions = decode(bytes);

    print!("{}", PrintVec(instructions));

    Command::new("rm").arg(output).output()?;

    Ok(())
}
