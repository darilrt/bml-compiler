use super::{lexer::lex, token::Token};

pub struct Scanner {
    pub tokens: Vec<Token>,
    index: usize,
}

impl Scanner {
    pub fn new(source: String, file_name: String) -> Scanner {
        let tokens = lex(source, file_name);

        Scanner { tokens, index: 0 }
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.index)
    }

    pub fn advance(&mut self) {
        self.index += 1;
    }

    pub fn is_at_end(&self) -> bool {
        self.index >= self.tokens.len()
    }
}
