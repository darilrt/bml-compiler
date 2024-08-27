#[derive(Debug, Clone, PartialEq)]
pub enum IR {
    Empty,
    Program(Vec<Box<IR>>),

    // Declaration Statements
    DeclFunc(String),
    // Expression Statements
}
