// All the tokens in our program
#[derive(Debug, Clone)]
pub enum Token {
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

pub fn lex(source: String) -> Vec<Token> {
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
