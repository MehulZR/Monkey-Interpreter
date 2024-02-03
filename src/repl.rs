use crate::{
    lexer::*,
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
            let mut lexer = Lexer::new(line);
            let mut token = lexer.next_token();
            while token != *EOF_TOKEN {
                println!("{:?}", token);
                token = lexer.next_token();
            }
        }
        print!(">> ");
        io::stdout().flush().unwrap();
    }
}
