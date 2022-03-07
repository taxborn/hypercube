#![allow(non_snake_case)]
use std::fs::File;
use std::io::Read;

use clap::Parser;
use ndarray::{Array, Ix4};

/// Argument struct for the CLI
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The file name
    #[clap(short, long)]
    file: String,

    /// The side length of the hypercube. This shouldn't change to adhere to
    /// the spec, but might be interesting to play around with.
    #[clap(short, long, default_value_t = 8)]
    count: usize,
}

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
    let mut file = File::open(args.file).expect("Open failed.");
    let mut source = String::new();

    file.read_to_string(&mut source).expect("Read to string failed.");

    let mut mem = Array::<u32, Ix4>
        ::zeros((args.count, args.count, args.count, args.count));

    let tokens = lexer(source);
    let instructions = parser(tokens);

    println!("INSTRUCTIONS:");
    println!("{:?}", instructions);
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
            _   => None
        }; match token {
            Some(token) => tokens.push(token),
            None        => (),
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
                Token::LoopEnd => 
                    panic!("Loop ending at #{} has no beginning.", idx),
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
