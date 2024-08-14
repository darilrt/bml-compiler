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

    pub fn peek(&mut self) -> Option<Token> {
        self.scanner.borrow_mut().peek()
    }

    pub fn advance(&mut self) {
        self.scanner.borrow_mut().advance();
    }

    pub fn is_at_end(&mut self) -> bool {
        self.scanner.borrow_mut().is_at_end()
    }

    pub fn expect(&mut self, lexeme: &str) -> Stmt {
        let token = self.peek();

        if token.is_none() {
            return Stmt::Empty;
        }

        let token = token.unwrap();

        if token.lexeme == lexeme {
            self.advance();
            Stmt::Empty
        } else {
            parse_error(&token, &format!("Expected '{}'", lexeme));
            Stmt::Empty
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

        if token.lexeme == "<" {
            return self.parse_component();
        }

        match token.kind {
            TokenKind::String => {
                self.advance();
                Stmt::String(token.lexeme)
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

    pub fn parse_component(&mut self) -> Stmt {
        self.expect("<");

        let token = self.peek().expect("Expected a component name");
        let name = token.lexeme.clone();

        if token.kind != TokenKind::Identifier {
            parse_error(&token, "Expected an identifier");
        }

        self.advance();

        self.expect(">");

        let token = self.peek();

        if token.is_none() {
            return Stmt::Empty;
        }

        let token = token.unwrap();
        let children: Vec<Stmt>;

        if token.lexeme == "{" {
            self.advance();

            children = self.parse_children();

            self.expect("}");

            return Stmt::Element(name, children);
        } else {
            parse_error(&token, "Expected a '{'");
            Stmt::Empty
        }
    }

    pub fn parse_children(&mut self) -> Vec<Stmt> {
        let mut children = vec![];

        loop {
            let token = self.peek();

            if token.is_none() {
                break;
            }

            let token = token.unwrap();

            if token.lexeme == "}" {
                break;
            }

            let stmt = self.parse_stmt();

            if stmt == Stmt::Empty {
                break;
            }

            children.push(stmt);
        }

        children
    }
}
