#[derive(Debug, Clone, PartialEq)]
pub enum Ast {
    Program(Vec<Stmt>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Element(String, Vec<Stmt>),
    String(String),
    Empty,
}
