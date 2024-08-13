use super::{ast::*, scanner::Scanner, token::TokenKind};

pub fn parse_file(file_name: &str) -> Ast {
    // Read the source file
    let source = std::fs::read_to_string(file_name).expect("Failed to read the source file");

    // Create a new parser
    let mut parser = Parser::new(source, file_name.to_string());

    // Parse the source file
    parser.parse()
}

pub struct Parser {
    scanner: Scanner,
}

impl Parser {
    pub fn new(source: String, file_name: String) -> Parser {
        let scanner = Scanner::new(source, file_name);

        Parser { scanner }
    }

    pub fn parse(&mut self) -> Ast {
        let mut stmts = vec![];

        while !self.scanner.is_at_end() {
            let stmt = self.parse_stmt();

            stmts.push(stmt);
        }

        Ast::Program(stmts)
    }

    pub fn parse_stmt(&mut self) -> Stmt {
        let token = self.scanner.peek();

        if token.is_none() {
            return Stmt::Empty;
        }

        let token = token.unwrap();

        match token.kind {
            TokenKind::Keyword => {
                if token.lexeme == "export" {
                    self.parse_export_stmt()
                }
            }
            _ => self.parse_expr_stmt(),
        }
    }

    pub fn parse_export_stmt(&mut self) -> Stmt {
        self.scanner.advance();

        let expr = self.parse_expr();

        Stmt::Export(expr)
    }

    pub fn parse_expr_stmt(&mut self) -> Stmt {
        let expr = self.parse_expr();

        Stmt::Expr(expr)
    }

    pub fn parse_expr(&mut self) -> Expr {
        let token = self.scanner.peek();

        if token.is_none() {
            panic!("Unexpected end of file");
        }

        let token = token.unwrap();

        match token.kind {
            TokenKind::Number => {
                self.scanner.advance();

                Expr::Number(token.lexeme.parse().unwrap())
            }
            TokenKind::String => {
                self.scanner.advance();

                Expr::String(token.lexeme.clone())
            }
            _ => panic!("Unexpected token: {:?}", token),
        }
    }
}
