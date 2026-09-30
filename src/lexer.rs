extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier(String),
    Number(String),
    Comma,
    EOF,
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
        }
    }

    pub fn read_identifier(&mut self) -> String {
        let start = self.position;
        while self.position < self.input.len() && self.input[self.position].is_alphanumeric() {
            self.position += 1;
        }
        self.input[start..self.position].iter().collect()
    }

    pub fn read_number(&mut self) -> String {
        let start = self.position;
        while self.position < self.input.len() && self.input[self.position].is_numeric() {
            self.position += 1;
        }
        self.input[start..self.position].iter().collect()
    }
}
