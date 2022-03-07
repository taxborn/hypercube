#![allow(non_snake_case)]
use std::fs::File;
use std::io::Read;

use clap::Parser;

use ndarray::Array;
use ndarray::Ix4;

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
    LoopEnd
}

fn main() {
    let args = Args::parse();
    let mut file = File::open(args.file).expect("Open failed.");
    let mut source = String::new();

    file.read_to_string(&mut source).expect("Read to string failed.");

    let mut mem = Array::<u32, Ix4>
        ::zeros((args.count, args.count, args.count, args.count));

    let tokens = lexer(source);

    println!("{:?}", tokens);
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
        };

        match token {
            Some(token) => tokens.push(token),
            None      => (),
        };
    }

    tokens
}
