use std::cell::{Cell, RefCell};

/// NFA Nondeterministic finite Automation
/// 非确定性有限自动机实现表达式解析
///

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    Number,       // 0..9
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
    Identifier,   // [a-z].
    String,       // "[a-z]."
    Semicolon,    // ;
    And,          // &
}

#[derive(Debug, Clone, Copy)]
pub enum LexerState {
    Initial,
    InNumber,
    InNumberFloat,
    InString,
    InIdentifier,
    InBang,
    InGreater,
    InLess,
    InEqual,
    InAnd,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
}

impl Token {
    pub fn new(token_type: TokenType, value: String) -> Self {
        Token {
            token_type,
            lexeme: value,
        }
    }
}

pub struct LexerScanner {
    tokens: RefCell<Vec<Token>>,
    source: String,
    current_state: Cell<LexerState>,
    current_buffer: RefCell<String>,
}

impl LexerScanner {
    pub fn new(source: String) -> Self {
        let mut new_source = source;
        new_source.push('\0');
        LexerScanner {
            tokens: RefCell::new(vec![]),
            source: new_source,
            current_state: Cell::new(LexerState::Initial),
            current_buffer: RefCell::new(String::new()),
        }
    }

    pub fn scan(&self) -> Vec<Token> {
        for c in self.source.chars() {
            self.process_input(c);
        }

        self.tokens.borrow().to_vec()
    }

    pub fn process_input(&self, c: char) {
        match self.current_state.get() {
            LexerState::Initial => match c {
                '(' => self.add_token(TokenType::LeftParen, "(".to_string()),
                ')' => self.add_token(TokenType::RightParen, ")".to_string()),
                '+' => self.add_token(TokenType::Plus, "+".to_string()),
                '-' => self.add_token(TokenType::Minus, "-".to_string()),
                '*' => self.add_token(TokenType::Star, "*".to_string()),
                '/' => self.add_token(TokenType::Slash, "/".to_string()),
                '!' => self.set_state(LexerState::InBang),
                '>' => self.set_state(LexerState::InGreater),
                '<' => self.set_state(LexerState::InLess),
                '=' => self.set_state(LexerState::InEqual),
                '"' => self.set_state(LexerState::InString),
                ';' => self.add_token(TokenType::Semicolon, ';'.to_string()),
                '&' => self.set_state(LexerState::InAnd),
                '0'..='9' => {
                    self.set_state(LexerState::InNumber);
                    self.process_input(c);
                }
                c if c.is_alphabetic() => {
                    self.set_state(LexerState::InIdentifier);
                    self.process_input(c);
                }
                '\0' | ' ' => {
                    self.clear_buffer();
                    self.state_initial()
                }
                _ => panic!("Expression invalid."),
            },
            LexerState::InNumber => {
                if c.is_digit(10) {
                    self.push_buffer(c);
                } else if c == '.' {
                    self.push_buffer(c);
                    self.set_state(LexerState::InNumberFloat);
                } else {
                    self.add_token(TokenType::Number, self.get_buffer());
                    self.process_input(c);
                }
            }
            LexerState::InNumberFloat => {
                if c.is_digit(10) {
                    self.set_state(LexerState::InNumber);
                    self.process_input(c);
                } else {
                    panic!("Invalid float number.");
                }
            }
            LexerState::InIdentifier => {
                if c.is_alphabetic() {
                    self.push_buffer(c);
                } else {
                    self.add_token(TokenType::Identifier, self.get_buffer());
                    self.process_input(c);
                }
            }
            LexerState::InBang => {
                if c == '=' {
                    self.add_token(TokenType::BangEqual, "!=".to_string());
                } else {
                    self.add_token(TokenType::Bang, "!".to_string());
                    self.process_input(c);
                }
            }
            LexerState::InLess => {
                if c == '=' {
                    self.add_token(TokenType::LessEqual, "<=".to_string());
                } else {
                    self.add_token(TokenType::Less, "<".to_string());
                    self.process_input(c);
                }
            }
            LexerState::InGreater => {
                if c == '=' {
                    self.add_token(TokenType::GreaterEqual, ">=".to_string());
                } else {
                    self.add_token(TokenType::Greater, ">".to_string());
                    self.process_input(c);
                }
            }
            LexerState::InEqual => {
                if c == '=' {
                    self.add_token(TokenType::EqualEqual, "==".to_string());
                } else {
                    self.add_token(TokenType::Equal, "=".to_string());
                    self.process_input(c);
                }
            }
            LexerState::InString => {
                if c != '"' {
                    self.push_buffer(c);
                } else if c == ';' {
                    panic!("String must end with '\"'");
                } else {
                    self.add_token(TokenType::String, self.get_buffer());
                }
            }
            LexerState::InAnd => {
                if c == '&' {
                    self.add_token(TokenType::And, "&&".to_string());
                } else {
                    panic!("Expression invalid.");
                }
            }
        }
    }

    fn state_initial(&self) {
        self.set_state(LexerState::Initial);
    }
    fn get_buffer(&self) -> String {
        let buffer = self.current_buffer.borrow().clone();
        return buffer;
    }

    fn push_buffer(&self, c: char) {
        self.current_buffer.borrow_mut().push(c);
    }

    fn clear_buffer(&self) {
        self.current_buffer.borrow_mut().clear();
    }

    fn set_state(&self, state: LexerState) {
        self.current_state.set(state);
    }

    fn add_token(&self, token_type: TokenType, lexeme: String) {
        self.tokens.borrow_mut().push(Token { token_type, lexeme });
        self.clear_buffer();
        self.state_initial();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn scan_math_expression() {
        let scanner = LexerScanner::new("(1 + 1) * 3 / 2".to_string());
        let tokens: Vec<Token> = vec![
            Token::new(TokenType::LeftParen, '('.to_string()),
            Token::new(TokenType::Number, '1'.to_string()),
            Token::new(TokenType::Plus, '+'.to_string()),
            Token::new(TokenType::Number, '1'.to_string()),
            Token::new(TokenType::RightParen, ')'.to_string()),
            Token::new(TokenType::Star, '*'.to_string()),
            Token::new(TokenType::Number, '3'.to_string()),
            Token::new(TokenType::Slash, '/'.to_string()),
            Token::new(TokenType::Number, '2'.to_string()),
        ];
        assert_eq!(tokens, scanner.scan());
    }

    #[test]
    fn scan_complex_expression() {
        let scanner =
            LexerScanner::new("(1 + 1) * 3 / 2 != 10 && !false && \"string\";".to_string());
        let tokens: Vec<Token> = vec![
            Token::new(TokenType::LeftParen, '('.to_string()),
            Token::new(TokenType::Number, '1'.to_string()),
            Token::new(TokenType::Plus, '+'.to_string()),
            Token::new(TokenType::Number, '1'.to_string()),
            Token::new(TokenType::RightParen, ')'.to_string()),
            Token::new(TokenType::Star, '*'.to_string()),
            Token::new(TokenType::Number, '3'.to_string()),
            Token::new(TokenType::Slash, '/'.to_string()),
            Token::new(TokenType::Number, '2'.to_string()),
            Token::new(TokenType::BangEqual, "!=".to_string()),
            Token::new(TokenType::Number, "10".to_string()),
            Token::new(TokenType::And, "&&".to_string()),
            Token::new(TokenType::Bang, '!'.to_string()),
            Token::new(TokenType::Identifier, "false".to_string()),
            Token::new(TokenType::And, "&&".to_string()),
            Token::new(TokenType::String, "string".to_string()),
            Token::new(TokenType::Semicolon, ';'.to_string()),
        ];
        let scan_tokens = scanner.scan();
        assert_eq!(tokens, scan_tokens);
    }

    #[test]
    #[should_panic]
    fn scan_invalid_expression() {
        let scanner = LexerScanner::new("2. + 3".to_string());
        scanner.scan();
    }
}
