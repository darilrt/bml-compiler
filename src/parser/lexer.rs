use super::token::{Location, Token, TokenKind};

#[derive(Debug, Clone, PartialEq)]
pub struct Lexer {
    pub tokens: Vec<Token>,
    pub source: String,
    pub file_name: String,
    pub index: usize,
    pub line: usize,
    pub column: usize,
}

impl Lexer {
    pub fn new(source: String, file_name: String) -> Lexer {
        Lexer {
            tokens: Vec::new(),
            source,
            index: 0,
            line: 1,
            column: 1,
            file_name,
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        if self.index >= self.source.len() {
            return Token {
                kind: TokenKind::Eof,
                lexeme: "Eof".to_owned(),
                location: Location {
                    line: self.line,
                    column: self.column,
                    file: self.file_name.clone(),
                },
            };
        }

        let c = self.peek();
        let token = match c {
            '0'..='9' | '.' => self.lex_number(),
            'a'..='z' | 'A'..='Z' | '_' => self.lex_identifier(),
            '+' | '-' | '*' | '/' | '%' | '=' | '!' | '<' | '>' => self.lex_operator(),
            '(' | ')' | '{' | '}' | '[' | ']' | ',' | ';' => self.lex_punctuation(),
            '"' | '\'' => self.lex_string(),
            '@' | '#' | '$' | '?' | ':' => self.lex_symbol(),
            _ => self.lex_unknown(),
        };

        token
    }

    pub fn advance(&mut self) {
        if self.peek() == '\n' {
            self.line += 1;
            self.column = 1;
        }
        self.index += 1;
        self.column += 1;
    }

    pub fn skip_whitespace(&mut self) {
        while self.peek().is_whitespace() {
            self.advance();
        }
    }

    pub fn peek(&self) -> char {
        if self.index < self.source.len() {
            self.source.chars().nth(self.index).unwrap()
        } else {
            '\0'
        }
    }

    pub fn lex_symbol(&mut self) -> Token {
        let c = self.peek();
        self.advance();

        Token {
            kind: TokenKind::Symbol,
            lexeme: c.to_string(),
            location: Location {
                line: self.line,
                column: self.column - 1,
                file: self.file_name.clone(),
            },
        }
    }

    pub fn lex_unknown(&mut self) -> Token {
        let c = self.peek();
        self.advance();

        Token {
            kind: TokenKind::Unknown,
            lexeme: c.to_string(),
            location: Location {
                line: self.line,
                column: self.column - 1,
                file: self.file_name.clone(),
            },
        }
    }

    pub fn lex_number(&mut self) -> Token {
        let mut lexeme = String::new();

        while self.peek().is_digit(10) {
            lexeme.push(self.peek());
            self.advance();
        }

        if self.peek() == '.' {
            lexeme.push(self.peek());
            self.advance();

            while self.peek().is_digit(10) {
                lexeme.push(self.peek());
                self.advance();
            }
        }

        let len = lexeme.len();

        Token {
            kind: TokenKind::Number,
            lexeme,
            location: Location {
                line: self.line,
                column: self.column - len,
                file: self.file_name.clone(),
            },
        }
    }

    pub fn lex_identifier(&mut self) -> Token {
        let mut lexeme = String::new();

        while self.peek().is_alphanumeric() || self.peek() == '_' {
            lexeme.push(self.peek());
            self.advance();
        }

        let len = lexeme.len();

        if self.is_keyword(&lexeme) {
            Token {
                kind: TokenKind::Keyword,
                lexeme,
                location: Location {
                    line: self.line,
                    column: self.column - len,
                    file: self.file_name.clone(),
                },
            }
        } else {
            Token {
                kind: TokenKind::Identifier,
                lexeme,
                location: Location {
                    line: self.line,
                    column: self.column - len,
                    file: self.file_name.clone(),
                },
            }
        }
    }

    pub fn lex_punctuation(&mut self) -> Token {
        let c = self.peek();
        self.advance();

        Token {
            kind: TokenKind::Punctuation,
            lexeme: c.to_string(),
            location: Location {
                line: self.line,
                column: self.column - 1,
                file: self.file_name.clone(),
            },
        }
    }

    pub fn lex_operator(&mut self) -> Token {
        let c = self.peek();
        self.advance();

        Token {
            kind: TokenKind::Operator,
            lexeme: c.to_string(),
            location: Location {
                line: self.line,
                column: self.column - 1,
                file: self.file_name.clone(),
            },
        }
    }

    pub fn lex_string(&mut self) -> Token {
        let start = self.peek();
        let pos: Location = Location {
            line: self.line,
            column: self.column,
            file: self.file_name.clone(),
        };
        self.advance();

        let mut lexeme = String::new();
        let mut escape = false;

        while self.peek() != start || escape {
            if self.index >= self.source.len() {
                println!(
                    "In {}:{}:{}\n error: Unterminated string",
                    pos.file, pos.line, pos.column
                );
                std::process::exit(1);
            }

            let c = self.peek();
            self.advance();

            if escape {
                match c {
                    'n' => lexeme.push('\n'),
                    't' => lexeme.push('\t'),
                    _ => lexeme.push(c),
                }

                escape = false;
            } else {
                if c == '\\' {
                    escape = true;
                } else {
                    lexeme.push(c);
                }
            }
        }

        self.advance();

        Token {
            kind: TokenKind::String,
            lexeme,
            location: pos,
        }
    }

    pub fn is_keyword(&self, lexeme: &str) -> bool {
        match lexeme {
            "export" | "true" | "false" => true,
            _ => false,
        }
    }
}

pub fn lex(source: String, file_name: String) -> Vec<Token> {
    let mut lexer = Lexer::new(source, file_name);
    let mut tokens = Vec::new();

    while lexer.index < lexer.source.len() {
        let token = lexer.next_token();
        tokens.push(token);
    }

    tokens
}
