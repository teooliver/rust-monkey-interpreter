pub mod lexer;
pub mod token;
use std::io;

use crate::lexer::Lexer;

fn main() -> io::Result<()> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;

    println!("You typed: {}", input.trim());

    let mut l = Lexer::new(&input);

    loop {
        let tok = l.next_token();
        if tok == token::Token::Eof {
            break;
        }
        println!("CURRENT TOKEN: {:?}", tok);
    }

    Ok(())
}
