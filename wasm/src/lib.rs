mod ast;
mod eval;
mod lexer;
mod object;
mod parser;
mod repl;
mod token;
mod utils;

use eval::eval;
use lexer::Lexer;
use object::ObjectTrait;
use parser::Parser;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn interpret(input: String) -> String {
    let mut l = Lexer::new(input);
    let mut p = Parser::new(&mut l);
    let program = p.parse_program();

    let program = match program {
        Some(p) => p,
        None => return "Can't parse program".to_string(),
    };

    let errors = p.errors();

    if errors.len() > 0 {
        return errors.join("\n");
    }

    eval(program).inspect()
}
