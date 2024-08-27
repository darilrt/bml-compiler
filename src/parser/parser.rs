use std::cell::RefCell;

use super::{
    ast::*,
    scanner::Scanner,
    token::{Token, TokenKind},
};

pub fn parse_file(file_name: &str) -> Ast {
    // Read the source file
    let source = std::fs::read_to_string(file_name).expect("Failed to read the source file");

    // Create a new parser
    let mut parser = Parser::new(source, file_name.to_string());

    // Parse the source file
    parser.parse()
}

pub fn parse_error(token: &Token, message: &str) {
    eprintln!(
        "In {}:{}:{}\n error: {}",
        token.location.file, token.location.line, token.location.column, message
    );

    std::process::exit(1);
}

pub fn line_error(file_name: &str, line: usize, message: &str) {
    eprintln!("In {}:{}\n error: {}", file_name, line, message);

    std::process::exit(1);
}

pub struct Parser {
    scanner: RefCell<Scanner>,
}

impl Parser {
    pub fn new(source: String, file_name: String) -> Parser {
        let scanner = Scanner::new(source, file_name);

        Parser {
            scanner: RefCell::new(scanner),
        }
    }

    /// Get the next token from the scanner
    pub fn peek(&mut self) -> Option<Token> {
        self.scanner.borrow_mut().peek()
    }

    /// Advance the scanner to the next token
    pub fn advance(&mut self) {
        self.scanner.borrow_mut().advance();
    }

    /// Check if the scanner is at the end of the source file
    pub fn is_at_end(&mut self) -> bool {
        self.scanner.borrow_mut().is_at_end()
    }

    /// Check if the next token is the expected lexeme
    /// If it is, advance the scanner and return true
    /// Otherwise, return false and print an error message
    /// The error message will be printed by the parse_error function
    /// The parse_error function will also exit the program
    pub fn expect(&mut self, lexeme: &str) -> bool {
        let token = self.peek();

        if token.is_none() {
            return false;
        }

        let token = token.unwrap();

        if token.lexeme == lexeme {
            self.advance();
            return true;
        } else {
            parse_error(&token, &format!("Expected '{}'", lexeme));
            return false;
        }
    }

    /// Check if the next token is the expected lexeme
    /// If it is, advance the scanner and return true
    /// Otherwise, return false
    /// This function does not print an error message
    pub fn match_token(&mut self, lexeme: &str) -> bool {
        let token = self.peek();

        if token.is_none() {
            return false;
        }

        let token = token.unwrap();

        if token.lexeme == lexeme {
            self.advance();
            return true;
        } else {
            return false;
        }
    }

    pub fn parse(&mut self) -> Ast {
        let mut stmts = vec![];

        while !self.is_at_end() {
            let stmt = self.parse_stmt();

            stmts.push(stmt);
        }

        Ast::Program(stmts)
    }

    pub fn parse_stmt(&mut self) -> Stmt {
        let token = self.peek();

        if token.is_none() {
            return Stmt::Empty;
        }

        let token = token.unwrap();

        match token.kind {
            TokenKind::Identifier => {
                self.advance();

                if self.match_token(":") {
                    let name = token.lexeme.clone();

                    let kw = self.peek();

                    if kw.is_none() {
                        parse_error(
                            &token,
                            format!("Expected keyword after '{}:'", name).as_str(),
                        );
                        return Stmt::Empty;
                    }

                    let kw = kw.unwrap();

                    match kw.kind {
                        TokenKind::Keyword => self.parse_decl(name),
                        _ => Stmt::Empty,
                    }
                } else {
                    return Stmt::Empty;
                }
            }
            TokenKind::Eof => {
                self.advance();
                Stmt::Empty
            }
            _ => {
                parse_error(
                    &token,
                    format!("Unexpected token {}", token.lexeme).as_str(),
                );
                Stmt::Empty
            }
        }
    }

    fn parse_decl(&mut self, name: String) -> Stmt {
        let token = self.peek();

        if token.is_none() {
            return Stmt::Empty;
        }

        let token = token.unwrap();

        match token.lexeme.as_str() {
            "fn" => Stmt::DeclFunc(name),
            _ => {
                line_error(
                    &token.location.file,
                    token.location.line,
                    format!("Unexpected keyword '{}'", token.lexeme).as_str(),
                );
                Stmt::Empty
            }
        }
    }
}
