#[derive(Debug, Clone, PartialEq)]
pub enum Ast {
    Program(Vec<Stmt>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Empty,

    // Declaration Statements
    DeclFunc(String),
    // Expression Statements
}
