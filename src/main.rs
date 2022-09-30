pub mod lexer;
mod repl;
pub mod token;
use std::io;

fn main() {
    repl::start();
}
