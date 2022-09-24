use crate::lexer::{Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    token_idx: usize,
    ast: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, token_idx: 0, ast: Vec::new() }
    }

    pub fn print_tokens(&self) {
        println!("Parser Tokens: {:?}", self.tokens);
    }


    pub fn accept_token(&mut self, token_type: TokenType) -> Result<Option<&Token>, &str> {
        if let Some(next_token) = self.next_token() {
            if next_token.token_type == token_type {
                return Ok(Some(next_token));
            } else {
                // TODO: Provide helpful error message
                return Err("Error: Unexpected token");
            }
        }
        Ok(None)
    }

    pub fn next_token(&mut self) -> Option<&Token> {
        self.token_idx += 1;
        if self.token_idx >= self.tokens.len() {
            return None;
        }
        Some(&self.tokens[self.token_idx])
    }

    pub fn parse_factor(&mut self) {
        // TODO: refactor, abandon accept_token, iterate tokens
        if let Ok(Some(token)) = self.accept_token(TokenType::INT) {

        }
    }

    pub fn parse_term(&self) {
        let token = &self.tokens[self.token_idx];
        let value = &token.value;
    }

    pub fn parse_expression(&self) {

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
