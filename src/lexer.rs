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

// Our lexer, which takes in a source string, and converts it into a vector of Tokens
pub fn lex(source: String) -> Vec<Token> {
    let mut tokens = Vec::new();

    for symbol in source.chars() {
        let token = match symbol {
            // Increment the memory cell under the pointer.
            '+' => Some(Token::Increment),
            // Decrement the memory cell under the pointer.
            '-' => Some(Token::Decrement),
            // Increase the pointer's position along the X axis.
            '>' => Some(Token::IncrementX),
            // Decrease the pointer's position along the X axis.
            '<' => Some(Token::DecrementX),
            // Increase the pointer's position along the Y axis.
            '^' => Some(Token::IncrementY),
            // Decrease the pointer's position along the Y axis.
            'V' | 'v' => Some(Token::DecrementY),
            // Increase the pointer's position along the Z axis.
            '*' => Some(Token::IncrementZ),
            // Decrease the pointer's position along the Z axis.
            'O' | 'o' => Some(Token::DecrementZ),
            // Increase the pointer's position along the W axis.
            '@' => Some(Token::IncrementW),
            // Decrease the pointer's position along the W axis.
            '?' => Some(Token::DecrementW),
            // Output the character signified by the cell at the pointer's
            // position on the X, Y, Z, and W axis.
            '.' => Some(Token::Write),
            // Input a character and store it in the cell at the pointer's
            // position on the X, Y, Z, and W axis.
            ',' => Some(Token::Read),
            // Jump past the matching ] if the cell under the pointer is 0.
            '[' => Some(Token::LoopBegin),
            // Jump back to the matching [ if the cell under the pointer is
            // nonzero.
            ']' => Some(Token::LoopEnd),
            // Don't read the token.
            _ => None,
        };

        // Push to the token vector if it is a valid token
        match token {
            Some(token) => tokens.push(token),
            None => (),
        };
    }

    // Return all the tokens
    tokens
}
