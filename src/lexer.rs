#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Kernel,
    Identifier(String),
    HardwareRamCap,
    SizeMB(usize),
    Tools,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    Colon,
    Comma,
    Eof,
}

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, position: 0 }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        if self.position >= self.input.len() {
            return Token::Eof;
        }

        let ch = self.input.as_bytes()[self.position] as char;

        match ch {
            '{' => { self.position += 1; Token::OpenBrace }
            '}' => { self.position += 1; Token::CloseBrace }
            '[' => { self.position += 1; Token::OpenBracket }
            ']' => { self.position += 1; Token::CloseBracket }
            ':' => { self.position += 1; Token::Colon }
            ',' => { self.position += 1; Token::Comma }
            '@' => {
                self.position += 1;
                let ident = self.read_identifier();
                if ident == "Kernel" {
                    Token::Kernel
                } else {
                    Token::Identifier(ident)
                }
            }
            _ => {
                if ch.is_alphabetic() || ch == '_' {
                    let ident = self.read_identifier();
                    match ident.as_str() {
                        "hardware_ram_cap" => Token::HardwareRamCap,
                        "tools" => Token::Tools,
                        _ => Token::Identifier(ident),
                    }
                } else if ch.is_numeric() {
                    let num_str = self.read_number();
                    if self.input[self.position..].starts_with("MB") {
                        self.position += 2;
                        Token::SizeMB(num_str.parse().unwrap_or(0))
                    } else {
                        Token::SizeMB(num_str.parse().unwrap_or(0))
                    }
                } else {
                    self.position += 1;
                    self.next_token()
                }
            }
        }
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.input.len() 
            && self.input.as_bytes()[self.position].is_ascii_whitespace() 
        {
            self.position += 1;
        }
    }

    fn read_identifier(&mut self) -> String {
        let start = self.position;
        while self.position < self.input.len() {
            let ch = self.input.as_bytes()[self.position] as char;
            if ch.is_alphanumeric() || ch == '_' || ch == '-' {
                self.position += 1;
            } else {
                break;
            }
        }
        self.input[start..self.position].to_string()
    }

    fn read_number(&mut self) -> String {
        let start = self.position;
        while self.position < self.input.len() 
            && (self.input.as_bytes()[self.position] as char).is_numeric() 
        {
            self.position += 1;
        }
        self.input[start..self.position].to_string()
    }
}
