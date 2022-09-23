use crate::lexer::Token;



pub struct Parser {
    tokens: Vec<Token>,
    ast: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, ast: Vec::new() }
    }

    pub fn print_tokens(&self) {
        println!("Parser Tokens: {:?}", self.tokens);
    }
}


struct AddNode {
    left: Option<usize>,
    right: Option<usize>
}

struct SubtractNode {
    left: usize,
    right: usize
}
