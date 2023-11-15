use std::{cell::Cell, num::IntErrorKind};

/// 字符串转换整数 (atoi)
/// https://leetcode.cn/problems/string-to-integer-atoi/description/

/// 使用确定性有限自动机（Deterministic Finite Automation DFA）解决

#[derive(Debug, Clone, Copy)]
enum AtoiState {
    Start,
    Integer,
    End,
}

pub struct Atoi {
    current_state: Cell<AtoiState>,
    source: String,
}

impl Atoi {
    pub fn new(s: String) -> Self {
        Atoi {
            current_state: Cell::new(AtoiState::Start),
            source: s,
        }
    }
}

impl Atoi {
    pub fn parse(&self) -> i32 {
        let mut result = String::new();
        for c in self.source.chars() {
            println!("{}", c);
            self.process_input(c, &mut result);
        }
        let num = result.parse::<i32>();
        let num = match num {
            Ok(num) => num,
            Err(e) => match e.kind() {
                IntErrorKind::NegOverflow => i32::MIN,
                IntErrorKind::PosOverflow => i32::MAX,
                _ => 0,
            },
        };
        num
    }

    fn process_input(&self, c: char, res: &mut String) {
        self.current_state.set(match (self.current_state.get(), c) {
            (AtoiState::Start, ' ' | '+') => AtoiState::Start,
            (AtoiState::Start, '-') => {
                res.push(c);
                AtoiState::Start
            }
            (AtoiState::Start, c) if c.is_digit(10) => {
                res.push(c);
                AtoiState::Integer
            }
            (AtoiState::Integer, c) if c.is_digit(10) => {
                res.push(c);
                AtoiState::Integer
            }
            _ => AtoiState::End,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Test: " -222 abc"
    fn atoi_with_whitespace_and_dot() {
        let atoi = Atoi::new(" -22 abc".to_string());
        assert_eq!(-22, atoi.parse());

        let atoi = Atoi::new("-22.3333abc".to_string());
        assert_eq!(-22, atoi.parse());
    }

    #[test]
    /// Test: "2000000000000000000000"
    fn atoi_with_overflow() {
        let atoi = Atoi::new("2000000000000000000000".to_string());
        assert_eq!(i32::MAX, atoi.parse());

        let atoi = Atoi::new("-2000000000000000000000".to_string());
        assert_eq!(i32::MIN, atoi.parse());
    }

    #[test]
    fn atoi_with_invalid() {
        let atoi = Atoi::new("abc".to_string());
        assert_eq!(0, atoi.parse());
    }
}
