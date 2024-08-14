#[derive(Debug, Clone, PartialEq)]
pub enum IR {
    Empty,
    Program(Vec<Box<IR>>),
    Element(String, Vec<Box<IR>>),
    String(String),
}
