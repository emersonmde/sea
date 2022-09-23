use regex::{Regex, RegexSet};

#[derive(Debug, Copy, Clone)]
pub enum TokenType {
    INT,
    PLUS,
    MINUS,
    EQUALS,
    VAR,
    SEMICOLON,
    STRING,
}

#[derive(Debug, Clone)]
pub enum TokenValue {
    Integer(i64),
    Float(f64),
    String(String)
}

#[derive(Debug, Clone)]
struct TokenMatcher {
    token_type: TokenType,
    pattern: String,
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    value: String,
}

#[derive(Debug)]
pub struct Lexer<'a> {
    pub src: &'a str,
    pub tokens: Vec<Token>,

    token_matchers: Vec<TokenMatcher>,
    pos: usize,
    size: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        let tokens = Vec::new();
        let token_matchers: Vec<TokenMatcher> = vec![
            TokenMatcher { token_type: TokenType::SEMICOLON, pattern: "^;".to_string() },
            TokenMatcher { token_type: TokenType::INT, pattern: "^\\d+".to_string() },
            TokenMatcher { token_type: TokenType::PLUS, pattern: "^\\+".to_string() },
            TokenMatcher { token_type: TokenType::MINUS, pattern: "^-".to_string() },
            TokenMatcher { token_type: TokenType::EQUALS, pattern: "^=".to_string() },
            TokenMatcher { token_type: TokenType::VAR, pattern: "^[a-zA-Z][a-zA-Z0-9_-]+".to_string() },
            TokenMatcher { token_type: TokenType::STRING, pattern: "^\".*\"".to_string() },
        ];

        Self { src, tokens, token_matchers, pos: 0, size: src.len() }
    }

    pub fn tokenize(&mut self) {
        while let Some(token) = self.get_token() {
            println!("Found token: {:?}", token);
            self.tokens.push(token);
        }
    }


    fn get_token(&mut self) -> Option<Token> {
        // self.tokens.push(Token { token_type: TokenType::INT })

        // https://docs.rs/regex/latest/regex/struct.RegexSet.html#limitations
        // let patterns = ["^\\d+", "^\".*\""];

        if self.pos >= self.size {
            return None;
        }

        let patterns: Vec<String> = self.token_matchers.clone().into_iter().map(|token| token.pattern).collect();
        let set = RegexSet::new(&patterns).unwrap();
        let regexes: Vec<_> = set.patterns().iter()
            .map(|pattern| Regex::new(pattern).unwrap())
            .collect();

        // Matches arrive in the order the constituent patterns were declared, not the order
        // they appear in the input
        let src = &self.src[self.pos..];
        let matches: Vec<usize> = set.matches(src).into_iter().collect();
        if matches.len() == 0 {
            println!("No matches");
            return None;
        }

        // Get the first match
        let match_idx = matches[0];
        let regex = &regexes[match_idx];
        let token_type = self.token_matchers[match_idx].token_type;
        let regex_match = regex.find(src).unwrap();
        let value = regex_match.as_str().to_string();

        self.pos += value.len();

        Some(Token { token_type, value })
    }
}
