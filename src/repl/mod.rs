use std::io;

use crate::{lexer::Lexer, token};

pub fn start() -> io::Result<()> {
    println!("Hello! This is the Monkey programming language!");
    println!("Feel free to type in commands");

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let mut l = Lexer::new(&input);

    loop {
        let tok = l.next_token();
        if tok == token::Token::Eof {
            break;
        }
        println!("{:?}", tok);
    }

    Ok(())
}
