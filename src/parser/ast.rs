#[derive(Debug, Clone, PartialEq)]
pub enum Ast {
    Program(Vec<Stmt>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expr(Expr),
    Export(Expr),
    Empty,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(f64),
    String(String),
}
