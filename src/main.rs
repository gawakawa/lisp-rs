use std::{cell::RefCell, rc::Rc};

use linefeed::{Interface, ReadResult};

use crate::{
    eval::{Env, eval},
    lexer::tokenize,
    parser::parse,
};

mod eval;
mod lexer;
mod parser;

const PROMPT: &str = "lisp-rs> ";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = Interface::new(PROMPT).unwrap();
    reader.set_prompt(PROMPT.to_string().as_ref()).unwrap();

    let mut env = Rc::new(RefCell::new(Env::new()));

    while let ReadResult::Input(input) = reader.read_line().unwrap() {
        if input.eq("exit") {
            break;
        }

        let mut tokens = tokenize(input.as_ref())?;
        let ast = parse(&mut tokens)?;
        let val = eval(&ast, &mut env)?;
        println!("{}", val);
    }

    println!("Good bye");
    Ok(())
}
