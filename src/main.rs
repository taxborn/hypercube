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

use errors::LoopError;
use errors::MovError;
use locator::Direction;
use locator::Loc;
use parser::Instruction;

/// Argument struct for the CLI
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The file name.
    #[clap(short = 'f', long)]
    file: String,

    // The side length of the hypercube. This shouldn't change to adhere to
    // the spec, but might be interesting to play around with. This value
    // gives us n^4 memory cells, so definitely be careful when altering this
    // value.
    /// The side length of the 'hypercube' of memory.
    #[clap(short = 'n', long, default_value_t = 8)]
    count: usize,

    /// Show the tokens the lexer outputs.
    #[clap(short = 't', long)]
    show_tokens: bool,

    /// Show the instructions the parser outputs.
    #[clap(short = 'i', long)]
    show_instructions: bool,
}

fn main() -> Result<(), LoopError> {
    let args = Args::parse();
    let count = args.count;

    let mut file = File::open(args.file).expect("Open failed.");
    let mut source = String::new();

    // Read the file to a string
    file.read_to_string(&mut source)
        .expect("Read to string failed.");

    // Create the working memory of the interpreter
    let mut mem = Array::<u8, Ix4>::zeros((count, count, count, count));

    // Use the lexer to store a vector of Tokens
    let tokens = lexer::lex(source);

    // Debug information for the tokens
    if args.show_tokens {
        println!("Tokens:");
        println!("{:?}", &tokens);
    }

    // Use the parser to store a vector of Instructions
    let instructions = parser::parse(tokens)?;

    // Debug information for the instructions
    if args.show_instructions {
        println!("Instructions:");
        println!("{:?}", &instructions);
    }

    // Create a locator for the interpreter.
    //
    // TODO: Maybe the interpreter should create this itself?
    let mut locator = Loc::new(count);

    // If we wanted to output the tokens or instructions, maybe we didn't want
    // to run the program.
    if !args.show_tokens && !args.show_instructions {
        // Interpret the program!
        match run(instructions, &mut mem, &mut locator, count) {
            Ok(()) => (),
            Err(e) => eprintln!("{}", e),
        }
    }

    Ok(())
}

// The interpreter. Takes in a vector of instructions, working memory for the
// interpreter, a locator to keep track of where we are, and the length of a
// side of the hypercube.
fn run(
    instructions: Vec<Instruction>,
    mem: &mut Array<u8, Ix4>,
    locator: &mut Loc,
    count: usize,
) -> Result<(), MovError> {
    // Loop over all of the instructions
    for instruction in instructions {
        match instruction {
            // Increment the X pointer.
            Instruction::IncrementX => locator.mov(Direction::XPos, 1)?,
            // Decrement the X pointer.
            Instruction::DecrementX => locator.mov(Direction::XNeg, 1)?,
            // Increment the Y pointer.
            Instruction::IncrementY => locator.mov(Direction::YPos, 1)?,
            // Decrement the Y pointer.
            Instruction::DecrementY => locator.mov(Direction::YNeg, 1)?,
            // Increment the Z pointer.
            Instruction::IncrementZ => locator.mov(Direction::ZPos, 1)?,
            // Decrement the Z pointer.
            Instruction::DecrementZ => locator.mov(Direction::ZNeg, 1)?,
            // Increment the W pointer.
            Instruction::IncrementW => locator.mov(Direction::WPos, 1)?,
            // Decrement the W pointer.
            Instruction::DecrementW => locator.mov(Direction::WNeg, 1)?,
            // Increment the value of the current cell under the locator.
            Instruction::Increment => {
                mem[[locator.x, locator.y, locator.z, locator.w]] += 1
            }
            // Decrement the value of the current cell under the locator.
            Instruction::Decrement => {
                mem[[locator.x, locator.y, locator.z, locator.w]] -= 1
            }
            // Write to stdout the current character represented by the memory 
            // cell.
            Instruction::Write => print!(
                "{}",
                mem[[locator.x, locator.y, locator.z, locator.w]] as char
            ),
            // Read from stdin a character, and put its value into the current cell.
            Instruction::Read => {
                let mut input: [u8; 1] = [0; 1];

                std::io::stdin()
                    .read_exact(&mut input)
                    .expect("Failed to read data.");

                mem[[locator.x, locator.y, locator.z, locator.w]] = input[0];
            }
            // A loop instruction.
            Instruction::Loop(instructions) => {
                // A zeroed out 4d memory hypercube for comparison purposes.
                let zeroArray =
                    Array::<u8, Ix4>::zeros((count, count, count, count));

                // Loop through the 'inner' instructions of the loop while
                // there are still instructions.
                while mem.to_owned() != zeroArray {
                    match run(instructions.to_owned(), mem, locator, count) {
                        // All is well, nothing to return.
                        Ok(()) => (),
                        // 'Falling off the hypercube' happened.
                        // 
                        // TODO: Should this be a panic instead?
                        Err(e) => eprintln!("{}", e),
                    }
                }
            }
        }
    }

    // All is well, nothing to return.
    Ok(())
}
