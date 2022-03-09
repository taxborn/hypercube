#![allow(non_snake_case)]
use std::fs::File;
use std::io::Read;

// Libraries used
use clap::Parser;
use ndarray::{Array, Ix4};

// Crate functions and structures
mod errors;
mod lexer;
mod locator;
mod parser;

use errors::MovError;
use locator::Direction;
use locator::Loc;
use parser::Instruction;

/// Argument struct for the CLI
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The file name
    #[clap(short, long)]
    file: String,
    // The side length of the hypercube. This shouldn't change to adhere to
    // the spec, but might be interesting to play around with.
    #[clap(short, long, default_value_t = 8)]
    count: usize,
}

fn main() {
    let args = Args::parse();
    let count = args.count;
    let mut file = File::open(args.file).expect("Open failed.");
    let mut source = String::new();

    file.read_to_string(&mut source)
        .expect("Read to string failed.");

    let mut mem = Array::<u8, Ix4>::zeros((count, count, count, count));

    let tokens = lexer::lex(source);
    let instructions = parser::parse(tokens);

    let mut locator = Loc::new(count);

    match run(instructions, &mut mem, &mut locator, count) {
        Ok(()) => (),
        Err(e) => eprintln!("{}", e),
    }
}

fn run(
    instructions: Vec<Instruction>,
    mem: &mut Array<u8, Ix4>,
    locator: &mut Loc,
    count: usize,
) -> Result<(), MovError> {
    for instruction in instructions {
        match instruction {
            Instruction::IncrementX => {
                if let Err(e) = locator.mov(Direction::XPos, 1) {
                    return Err(e);
                }
            }
            Instruction::DecrementX => {
                if let Err(e) = locator.mov(Direction::XNeg, 1) {
                    return Err(e);
                }
            }
            Instruction::IncrementY => {
                if let Err(e) = locator.mov(Direction::YPos, 1) {
                    return Err(e);
                }
            }
            Instruction::DecrementY => {
                if let Err(e) = locator.mov(Direction::YNeg, 1) {
                    return Err(e);
                }
            }
            Instruction::IncrementZ => {
                if let Err(e) = locator.mov(Direction::ZPos, 1) {
                    return Err(e);
                }
            }
            Instruction::DecrementZ => {
                if let Err(e) = locator.mov(Direction::ZNeg, 1) {
                    return Err(e);
                }
            }
            Instruction::IncrementW => {
                if let Err(e) = locator.mov(Direction::WPos, 1) {
                    return Err(e);
                }
            }
            Instruction::DecrementW => {
                if let Err(e) = locator.mov(Direction::WNeg, 1) {
                    return Err(e);
                }
            }
            Instruction::Increment => {
                mem[[locator.x, locator.y, locator.z, locator.w]] += 1
            }
            Instruction::Decrement => {
                mem[[locator.x, locator.y, locator.z, locator.w]] -= 1
            }
            Instruction::Write => print!(
                "{}",
                mem[[locator.x, locator.y, locator.z, locator.w]] as char
            ),
            Instruction::Read => {
                let mut input: [u8; 1] = [0; 1];

                std::io::stdin()
                    .read_exact(&mut input)
                    .expect("Failed to read data.");

                mem[[locator.x, locator.y, locator.z, locator.w]] = input[0];
            }
            Instruction::Loop(instructions) => {
                let zeroArray =
                    Array::<u8, Ix4>::zeros((count, count, count, count));

                while mem.to_owned() != zeroArray {
                    match run(instructions.to_owned(), mem, locator, count) {
                        Ok(()) => (),
                        Err(e) => eprintln!("{}", e),
                    }
                }
            }
        }
    }

    Ok(())
}
