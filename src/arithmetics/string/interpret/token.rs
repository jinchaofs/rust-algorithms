#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    Number,       // [0-9]+
    LeftParen,    // (
    RightParen,   // )
    Minus,        // -
    Plus,         // +
    Star,         // *
    Slash,        // /
    Bang,         // !
    Equal,        // =
    EqualEqual,   // ==
    BangEqual,    // !=
    Greater,      // >
    GreaterEqual, // >=
    Less,         // <
    LessEqual,    // <=
    Identifier,   // [a-z]+
    String,       // "[a-z]+"
    Semicolon,    // ;
    And,          // &&
    Or,           // ||
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub line: u32,
}

impl Position {
    pub fn new(line: u32) -> Self {
        Position { line }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: Option<String>,
    pub position: Position,
}

impl Token {
    pub fn new(token_type: TokenType, value: Option<String>, position: Position) -> Self {
        Token {
            token_type,
            lexeme: value,
            position,
        }
    }
}
