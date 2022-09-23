use crate::lexer::Lexer;
use crate::parser::Parser;

mod lexer;
mod parser;

fn main() {
    let src = "12+4;";
    let mut lexer = Lexer::new(src);
    println!("Source: {}", lexer.src);
    lexer.tokenize();
    let mut parser = Parser::new(lexer.tokens);
    parser.print_tokens();
}
