use crate::lexer::Token;

#[derive(Debug, Clone)]
pub enum Instruction {
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

pub fn parse(tokens: Vec<Token>) -> Vec<Instruction> {
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
                        instructions.push(Instruction::Loop(parse(
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
