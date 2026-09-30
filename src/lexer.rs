pub #[derive(Debug, PartialEq, Clone)]
enum Token {
    // Kata Kunci Utama
    At,                // '@'
    Kernel,            // 'Kernel'
    UI,                // 'UI'
    Service,           // 'Service'
    
    // Tanda Baca
    LeftBrace,         // '{'
    RightBrace,        // '}'
    LeftBracket,       // '['
    RightBracket,      // ']'
    Colon,             // ':'
    Comma,             // ','

    // Data
    Identifier(String),
    StringLiteral(String),
    Number(u64),

    EOF,
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
            return Token::EOF;
        }

        let ch = self.current_char();

        match ch {
            '@' => { self.advance(); Token::At }
            '{' => { self.advance(); Token::LeftBrace }
            '}' => { self.advance(); Token::RightBrace }
            '[' => { self.advance(); Token::LeftBracket }
            ']' => { self.advance(); Token::RightBracket }
            ':' => { self.advance(); Token::Colon }
            ',' => { self.advance(); Token::Comma }
            '"' => self.read_string(),
            _ if ch.is_alphabetic() => self.read_identifier(),
            _ if ch.is_numeric() => self.read_number(),
            _ => {
                self.advance();
                Token::EOF
            }
        }
    }

    fn current_char(&self) -> char {
        self.input[self.position..].chars().next().unwrap_or('\0')
    }

    fn advance(&mut self) {
        if self.position < self.input.len() {
            self.position += self.current_char().len_utf8();
        }
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.input.len() && self.current_char().is_whitespace() {
            self.advance();
        }
    }

    fn read_identifier(&mut self) -> Token {
        let start = self.position;
        while self.position < self.input.len() && (self.current_char().is_alphanumeric() || self.current_char() == '_') {
            self.advance();
        }
        let text = &self.input[start..self.position];
        match text {
            "Kernel" => Token::Kernel,
            "UI" => Token::UI,
            "Service" => Token::Service,
            _ => Token::Identifier(text.to_string()),
        }
    }

    fn read_string(&mut self) -> Token {
        self.advance(); // Langkau tanda petik pembuka "
        let start = self.position;
        while self.position < self.input.len() && self.current_char() != '"' {
            self.advance();
        }
        let text = &self.input[start..self.position];
        self.advance(); // Langkau tanda petik penutup "
        Token::StringLiteral(text.to_string())
    }

    fn read_number(&mut self) -> Token {
        let start = self.position;
        while self.position < self.input.len() && self.current_char().is_numeric() {
            self.advance();
        }
        let num: u64 = self.input[start..self.position].parse().unwrap_or(0);
        Token::Number(num)
    }
}
