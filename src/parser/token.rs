#[derive(Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub location: Location,
}

#[derive(Clone, PartialEq)]
pub enum TokenKind {
    Unknown,
    Eof,
    Number,
    String,
    Identifier,
    Operator,
    Keyword,
    Punctuation,
    Symbol,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Location {
    pub file: String,
    pub line: usize,
    pub column: usize,
}
