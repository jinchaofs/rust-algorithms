use std::cell::{Cell, RefCell};

use super::token::{Position, Token, TokenType};

/// Nondeterministic finite Automation
/// lexer scanner using NFA
///
///

#[derive(Debug, Clone, Copy)]
pub enum ScanState {
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
    InOr,
}

pub struct LexerScanner {
    tokens: RefCell<Vec<Token>>,
    source: String,
    current_state: Cell<ScanState>,
    current_buffer: RefCell<String>,
    pub position: Cell<Position>,
}

impl LexerScanner {
    pub fn new(source: String) -> Self {
        let mut new_source = source;
        new_source.push('\0');
        LexerScanner {
            tokens: RefCell::new(vec![]),
            source: new_source,
            current_state: Cell::new(ScanState::Initial),
            current_buffer: RefCell::new(String::new()),
            position: Cell::new(Position::new(0)),
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
            ScanState::Initial => match c {
                '(' => self.add_token(TokenType::LeftParen),
                ')' => self.add_token(TokenType::RightParen),
                '+' => self.add_token(TokenType::Plus),
                '-' => self.add_token(TokenType::Minus),
                '*' => self.add_token(TokenType::Star),
                '/' => self.add_token(TokenType::Slash),
                ';' => self.add_token(TokenType::Semicolon),
                '!' => self.set_state(ScanState::InBang),
                '>' => self.set_state(ScanState::InGreater),
                '<' => self.set_state(ScanState::InLess),
                '=' => self.set_state(ScanState::InEqual),
                '"' => self.set_state(ScanState::InString),
                '&' => self.set_state(ScanState::InAnd),
                '|' => self.set_state(ScanState::InOr),
                '0'..='9' => {
                    self.set_state(ScanState::InNumber);
                    self.process_input(c);
                }
                c if c.is_alphabetic() => {
                    self.set_state(ScanState::InIdentifier);
                    self.process_input(c);
                }
                '\n' => {
                    self.line_advance();
                }
                '\0' | ' ' => {
                    self.clear_buffer();
                    self.state_initial()
                }
                _ => panic!("Expression invalid."),
            },
            ScanState::InNumber => {
                if c.is_digit(10) {
                    self.push_buffer(c);
                } else if c == '.' {
                    self.push_buffer(c);
                    self.set_state(ScanState::InNumberFloat);
                } else {
                    self.add_token_with_value(TokenType::Number, Some(self.get_buffer()));
                    self.process_input(c);
                }
            }
            ScanState::InNumberFloat => {
                if c.is_digit(10) {
                    self.set_state(ScanState::InNumber);
                    self.process_input(c);
                } else {
                    panic!("Invalid float number.");
                }
            }
            ScanState::InIdentifier => {
                if c.is_alphabetic() {
                    self.push_buffer(c);
                } else {
                    self.add_token_with_value(TokenType::Identifier, Some(self.get_buffer()));
                    self.process_input(c);
                }
            }
            ScanState::InBang => {
                if c == '=' {
                    self.add_token(TokenType::BangEqual);
                } else {
                    self.add_token(TokenType::Bang);
                    self.process_input(c);
                }
            }
            ScanState::InLess => {
                if c == '=' {
                    self.add_token(TokenType::LessEqual);
                } else {
                    self.add_token(TokenType::Less);
                    self.process_input(c);
                }
            }
            ScanState::InGreater => {
                if c == '=' {
                    self.add_token(TokenType::GreaterEqual);
                } else {
                    self.add_token(TokenType::Greater);
                    self.process_input(c);
                }
            }
            ScanState::InEqual => {
                if c == '=' {
                    self.add_token(TokenType::EqualEqual);
                } else {
                    self.add_token(TokenType::Equal);
                    self.process_input(c);
                }
            }
            ScanState::InString => {
                if c != '"' {
                    self.push_buffer(c);
                } else if c == ';' {
                    panic!("String must end with '\"'");
                } else {
                    self.add_token_with_value(TokenType::String, Some(self.get_buffer()));
                }
            }
            ScanState::InAnd => {
                if c == '&' {
                    self.add_token(TokenType::And);
                } else {
                    panic!("Expression invalid.");
                }
            }
            ScanState::InOr => {
                if c == '|' {
                    self.add_token(TokenType::Or);
                } else {
                    panic!("Expression invalid.");
                }
            }
        }
    }

    fn line_advance(&self) {
        self.position
            .set(Position::new(self.position.get().line + 1));
    }

    fn state_initial(&self) {
        self.set_state(ScanState::Initial);
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

    fn set_state(&self, state: ScanState) {
        self.current_state.set(state);
    }

    fn add_token_with_value(&self, token_type: TokenType, value: Option<String>) {
        self.tokens.borrow_mut().push(Token {
            token_type,
            lexeme: value,
            position: self.position.get(),
        });
        self.clear_buffer();
        self.state_initial();
    }

    fn add_token(&self, token_type: TokenType) {
        self.add_token_with_value(token_type, None);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn new_token(token_type: TokenType) -> Token {
        Token::new(token_type, None, Position::new(0))
    }
    #[test]
    fn scan_math_expression() {
        let scanner = LexerScanner::new("(1 + 1) * 3 / 2".to_string());
        let position = Position::new(0);
        let tokens: Vec<Token> = vec![
            new_token(TokenType::LeftParen),
            Token::new(TokenType::Number, Some('1'.to_string()), position),
            new_token(TokenType::Plus),
            Token::new(TokenType::Number, Some('1'.to_string()), position),
            new_token(TokenType::RightParen),
            new_token(TokenType::Star),
            Token::new(TokenType::Number, Some('3'.to_string()), position),
            new_token(TokenType::Slash),
            Token::new(TokenType::Number, Some('2'.to_string()), position),
        ];
        assert_eq!(tokens, scanner.scan());
    }

    #[test]
    fn scan_complex_expression() {
        let scanner = LexerScanner::new(
            "(1 + 1) * 3 / 2 - 3 != 10001.22 && !false && \"string\" || true;".to_string(),
        );
        let position = Position::new(0);
        let tokens: Vec<Token> = vec![
            new_token(TokenType::LeftParen),
            Token::new(TokenType::Number, Some('1'.to_string()), position),
            new_token(TokenType::Plus),
            Token::new(TokenType::Number, Some('1'.to_string()), position),
            new_token(TokenType::RightParen),
            new_token(TokenType::Star),
            Token::new(TokenType::Number, Some('3'.to_string()), position),
            new_token(TokenType::Slash),
            Token::new(TokenType::Number, Some('2'.to_string()), position),
            new_token(TokenType::Minus),
            Token::new(TokenType::Number, Some('3'.to_string()), position),
            new_token(TokenType::BangEqual),
            Token::new(TokenType::Number, Some("10001.22".to_string()), position),
            new_token(TokenType::And),
            new_token(TokenType::Bang),
            Token::new(TokenType::Identifier, Some("false".to_string()), position),
            new_token(TokenType::And),
            Token::new(TokenType::String, Some("string".to_string()), position),
            new_token(TokenType::Or),
            Token::new(TokenType::Identifier, Some("true".to_string()), position),
            new_token(TokenType::Semicolon),
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
