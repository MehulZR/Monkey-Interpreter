use crate::{
    ast::Node,
    lexer::Lexer,
    parser::Parser,
    token::{Token, TokenType},
};
use lazy_static::lazy_static;
use std::io::{self, BufRead, Write};
use users::{get_current_uid, get_user_by_uid};
lazy_static! {
    static ref EOF_TOKEN: Token = Token {
        r#type: TokenType::EOF,
        literal: "".to_string(),
    };
}
pub fn start() {
    let user = get_user_by_uid(get_current_uid()).unwrap();

    println!(
        "Hello {}! This is the Monkey programming language!",
        user.name().to_string_lossy()
    );
    println!("Feel free to type commands");
    print!(">> ");

    io::stdout().flush().unwrap();

    for line in io::stdin().lock().lines() {
        if let Ok(line) = line {
            let mut l = Lexer::new(line);
            let mut p = Parser::new(&mut l);
            let program = p.parse_program();

            let program = match program {
                Some(p) => p,
                None => panic!("Can't parse program"),
            };

            let errors = p.errors();
            if errors.len() > 0 {
                print_parser_errors(&errors);
            } else {
                println!("{}", program.string());
            }
        }

        print!(">> ");
        io::stdout().flush().unwrap();
    }
}

fn print_parser_errors(errors: &[String]) {
    println!("Woops! We ran into some monkey business here!");
    println!("  parser errors:");
    for err in errors {
        println!("\t{}", err);
    }
}
