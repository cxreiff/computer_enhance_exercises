use std::{io, process::Command};

use clap::Parser;
use decoder::{decode, utils::PrintVec};

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

    #[cfg(debug_assertions)]
    println!();

    println!("{}", PrintVec(instructions));

    Command::new("rm").arg(output).output()?;

    Ok(())
}
