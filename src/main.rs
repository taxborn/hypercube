#![allow(non_snake_case)]
use std::fmt;
use std::fs::File;
use std::io::Read;

// Libraries used
use clap::Parser;
use ndarray::{Array, Ix4};

/// Argument struct for the CLI
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The file name
    #[clap(short, long)]
    file: String,
    // The side length of the hypercube. This shouldn't change to adhere to
    // the spec, but might be interesting to play around with.
    //
    // TODO: The barrier here is that this is a runtime found value, and that
    // causes issues if I want to use the value at compile time.
    #[clap(short, long, default_value_t = 8)]
    count: usize,
}

// The location struct to hold the current position of our program
#[derive(Clone, Copy)]
struct Loc {
    x: usize,
    y: usize,
    z: usize,
    w: usize,
    count: usize,
}

#[derive(Debug)]
enum Direction {
    XPos,
    XNeg,
    YPos,
    YNeg,
    ZPos,
    ZNeg,
    WPos,
    WNeg,
}

// Use a unit struct to declare custom 'Falling off the hypercube' error.
struct MovError {
    direction: Direction,
}

// Implementing std::fmt::Display for MovError
impl fmt::Display for MovError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ERROR: Fell off the hypercube, in the {:?} direction",
            self.direction
        )
    }
}

impl Loc {
    // Create a new location struct
    fn new(count: usize) -> Self {
        Loc {
            x: 0,
            y: 0,
            z: 0,
            w: 0,
            count
        }
    }

    fn mov(
        &mut self,
        direction: Direction,
        steps: isize,
    ) -> Result<(), MovError> {
        match direction {
            Direction::XPos => {
                if self.x + steps as usize >= self.count {
                    return Err(MovError {
                        direction: Direction::XPos,
                    });
                }

                self.x += steps as usize;
            }
            Direction::XNeg => {
                if self.x as isize - steps < 0 {
                    return Err(MovError {
                        direction: Direction::XNeg,
                    });
                }

                self.x -= steps as usize;
            }
            Direction::YPos => {
                if self.y + steps as usize >= self.count {
                    return Err(MovError {
                        direction: Direction::YPos,
                    });
                }

                self.y += steps as usize;
            }
            Direction::YNeg => {
                if self.y as isize - steps < 0 {
                    return Err(MovError {
                        direction: Direction::YNeg,
                    });
                }

                self.y -= steps as usize;
            }
            Direction::ZPos => {
                if self.z + steps as usize >= self.count {
                    return Err(MovError {
                        direction: Direction::ZPos,
                    });
                }

                self.z += steps as usize;
            }
            Direction::ZNeg => {
                if self.z as isize - steps < 0 {
                    return Err(MovError {
                        direction: Direction::ZNeg,
                    });
                }

                self.z -= steps as usize;
            }
            Direction::WPos => {
                if self.w + steps as usize >= self.count {
                    return Err(MovError {
                        direction: Direction::WPos,
                    });
                }

                self.w += steps as usize;
            }
            Direction::WNeg => {
                if self.w as isize - steps < 0 {
                    return Err(MovError {
                        direction: Direction::WNeg,
                    });
                }

                self.w -= steps as usize;
            }
        }

        Ok(())
    }
}

// All the tokens in our program
#[derive(Debug, Clone)]
enum Token {
    IncrementX,
    DecrementX,
    IncrementY,
    DecrementY,
    IncrementZ,
    DecrementZ,
    IncrementW,
    DecrementW,
    Increment,
    Decrement,
    Write,
    Read,
    LoopBegin,
    LoopEnd,
}

#[derive(Debug, Clone)]
enum Instruction {
    IncrementX,
    DecrementX,
    IncrementY,
    DecrementY,
    IncrementZ,
    DecrementZ,
    IncrementW,
    DecrementW,
    Increment,
    Decrement,
    Write,
    Read,
    Loop(Vec<Instruction>),
}

fn main() {
    let args = Args::parse();
    let count = args.count;
    let mut file = File::open(args.file).expect("Open failed.");
    let mut source = String::new();

    file.read_to_string(&mut source)
        .expect("Read to string failed.");

    let mut mem = Array::<u8, Ix4>::zeros((
        count, count, count, count,
    ));

    let tokens = lexer(source);
    let instructions = parser(tokens);

    let mut locator = Loc::new(count);

    match run(instructions, &mut mem, &mut locator, count) {
        Ok(()) => (),
        Err(e) => eprintln!("{}", e),
    }
}

fn lexer(source: String) -> Vec<Token> {
    let mut tokens = Vec::new();

    for symbol in source.chars() {
        let token = match symbol {
            '+' => Some(Token::Increment),
            '-' => Some(Token::Decrement),
            '>' => Some(Token::IncrementX),
            '<' => Some(Token::DecrementX),
            '^' => Some(Token::IncrementY),
            'V' | 'v' => Some(Token::DecrementY),
            '*' => Some(Token::IncrementZ),
            'O' | 'o' => Some(Token::DecrementZ),
            '@' => Some(Token::IncrementW),
            '?' => Some(Token::DecrementW),
            '.' => Some(Token::Write),
            ',' => Some(Token::Read),
            '[' => Some(Token::LoopBegin),
            ']' => Some(Token::LoopEnd),
            _ => None,
        };
        match token {
            Some(token) => tokens.push(token),
            None => (),
        };
    }

    tokens
}

fn parser(tokens: Vec<Token>) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();
    let mut loop_stack = 0;
    let mut loop_start = 0;

    for (idx, token) in tokens.iter().enumerate() {
        if loop_stack == 0 {
            let instruction = match token {
                Token::IncrementX => Some(Instruction::IncrementX),
                Token::DecrementX => Some(Instruction::DecrementX),
                Token::IncrementY => Some(Instruction::IncrementY),
                Token::DecrementY => Some(Instruction::DecrementY),
                Token::IncrementZ => Some(Instruction::IncrementZ),
                Token::DecrementZ => Some(Instruction::DecrementZ),
                Token::IncrementW => Some(Instruction::IncrementW),
                Token::DecrementW => Some(Instruction::DecrementW),
                Token::Increment => Some(Instruction::Increment),
                Token::Decrement => Some(Instruction::Decrement),
                Token::Read => Some(Instruction::Read),
                Token::Write => Some(Instruction::Write),
                Token::LoopBegin => {
                    loop_start = idx;
                    loop_stack += 1;

                    None
                }
                Token::LoopEnd => {
                    panic!("Loop ending at #{} has no beginning.", idx)
                }
            };

            match instruction {
                Some(instruction) => instructions.push(instruction),
                None => (),
            };
        } else {
            match token {
                Token::LoopBegin => loop_stack += 1,
                Token::LoopEnd => {
                    loop_stack -= 1;

                    if loop_stack == 0 {
                        instructions.push(Instruction::Loop(parser(
                            tokens[loop_start + 1..idx].to_vec(),
                        )));
                    }
                }

                _ => (),
            };
        }
    }

    if loop_stack != 0 {
        panic!(
            "Loop that starts at #{} has no matching ending!",
            loop_start
        );
    }

    instructions
}

fn run(
    instructions: Vec<Instruction>,
    mem: &mut Array<u8, Ix4>,
    locator: &mut Loc,
    count: usize
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
                let zeroArray = Array::<u8, Ix4>::zeros((count, count, count, count));

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
