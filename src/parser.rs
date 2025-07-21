use std::{
    error::Error,
    fmt::{self},
};

use crate::lexer::Token;

#[derive(Debug, PartialEq)]
pub struct ParseError {
    err: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parse error: {}", self.err)
    }
}

impl Error for ParseError {}

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Void,
    Integer(i64),
    Bool(bool),
    Symbol(String),
    Lambda(Vec<String>, Vec<Object>),
    List(Vec<Object>),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Void => write!(f, "Void"),
            Object::Integer(n) => write!(f, "{}", n),
            Object::Bool(b) => write!(f, "{}", b),
            Object::Symbol(s) => write!(f, "{}", s),
            Object::Lambda(params, body) => {
                write!(f, "Lambda(")?;
                for param in params {
                    write!(f, "{} ", param)?;
                }
                write!(f, ")")?;
                for expr in body {
                    write!(f, " {}", expr)?;
                }
                Ok(())
            }
            Object::List(list) => {
                write!(f, "(")?;
                for (i, obj) in list.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", obj)?;
                }
                write!(f, ")")
            }
        }
    }
}

pub fn parse(tokens: &mut [Token]) -> Result<Object, ParseError> {
    let mut rev_tokens = tokens.iter().cloned().rev().collect();
    parse_list(&mut rev_tokens)
}

fn parse_list(tokens: &mut Vec<Token>) -> Result<Object, ParseError> {
    let token = tokens.pop();
    if token != Some(Token::LParen) {
        return Err(ParseError {
            err: format!("Expected LParen, found {:?}", token),
        });
    }

    let mut list: Vec<Object> = Vec::new();
    while !tokens.is_empty() {
        let token = tokens.pop();
        if token.is_none() {
            return Err(ParseError {
                err: "Insufficient tokens".to_string(),
            });
        }

        let t = token.unwrap();
        match t {
            Token::Integer(n) => list.push(Object::Integer(n)),
            Token::Symbol(s) => list.push(Object::Symbol(s)),
            Token::LParen => {
                tokens.push(Token::LParen);
                let sub_list = parse_list(tokens)?;
                list.push(sub_list);
            }
            Token::RParen => {
                return Ok(Object::List(list));
            }
        }
    }

    Ok(Object::List(list))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let mut input = vec![
            Token::LParen,
            Token::Symbol("+".to_string()),
            Token::Integer(1),
            Token::Integer(2),
            Token::RParen,
        ];

        let expected = Ok(Object::List(vec![
            Object::Symbol("+".to_string()),
            Object::Integer(1),
            Object::Integer(2),
        ]));

        assert_eq!(expected, parse(&mut input));
    }
}
