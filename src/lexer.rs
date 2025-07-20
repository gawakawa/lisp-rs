use std::{error::Error, fmt};

#[derive(Debug, PartialEq)]
pub enum Token {
    Integer(i64),
    Symbol(String),
    LParen,
    RParen,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Integer(n) => write!(f, "{}", n),
            Token::Symbol(s) => write!(f, "{}", s),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
        }
    }
}

#[derive(Debug)]
pub struct TokenError {
    ch: char,
}

impl Error for TokenError {}

impl fmt::Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unexpected character: {}", self.ch)
    }
}

pub fn tokenize(program: &str) -> Result<Vec<Token>, TokenError> {
    let program = program.replace("(", " ( ").replace(")", " ) ");
    let words = program.split_whitespace();

    let tokens: Result<Vec<Token>, TokenError> = words
        .into_iter()
        .map(|word| match word {
            "(" => Ok(Token::LParen),
            ")" => Ok(Token::RParen),
            _ => match word.parse::<i64>() {
                Ok(n) => Ok(Token::Integer(n)),
                Err(_) => Ok(Token::Symbol(word.to_string())),
            },
        })
        .collect();
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let tokens = tokenize("(+ 1 2)").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::LParen,
                Token::Symbol("+".to_string()),
                Token::Integer(1),
                Token::Integer(2),
                Token::RParen,
            ]
        );
    }
}
