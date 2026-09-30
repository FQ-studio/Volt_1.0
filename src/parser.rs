use crate::lexer::{Lexer, Token};

#[derive(Debug, Clone)]
pub enum EngineTool {
    AILLM { id: String, ram_budget_mb: u32 },
    BareMetal { id: String, ram_budget_mb: u32 },
    UIEngine { id: String, ram_budget_mb: u32 },
    Camera { id: String, ram_budget_mb: u32 },
    Crate { id: String, ram_budget_mb: u32 },
}

#[derive(Debug, Clone)]
pub struct VoltAST {
    pub kernel_name: String,
    pub ram_limit_mb: u32,
    pub tools: Vec<EngineTool>,
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Self {
        let current_token = lexer.next_token();
        Self { lexer, current_token }
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    pub fn parse_volt_system(&mut self) -> VoltAST {
        let mut kernel_name = String::from("GravityKernel");
        let mut ram_limit_mb = 100;
        let mut tools = Vec::new();

        while self.current_token != Token::Eof {
            match &self.current_token {
                Token::Kernel => {
                    self.advance();
                    if let Token::Identifier(name) = &self.current_token {
                        kernel_name = name.clone();
                    }
                }
                Token::HardwareRamCap => {
                    self.advance();
                    if let Token::Colon = self.current_token {
                        self.advance();
                    }
                    if let Token::SizeMB(size) = self.current_token {
                        ram_limit_mb = size as u32;
                    }
                }
                Token::Tools => {
                    self.advance();
                    if let Token::Colon = self.current_token {
                        self.advance();
                    }
                    if let Token::OpenBracket = self.current_token {
                        self.advance();
                        while self.current_token != Token::CloseBracket && self.current_token != Token::Eof {
                            if let Token::Identifier(tool_str) = &self.current_token {
                                let default_id = tool_str.clone();
                                let tool = match tool_str.as_str() {
                                    "Tool_AILLM" => EngineTool::AILLM { id: default_id, ram_budget_mb: 20 },
                                    "Tool_BareMetal" => EngineTool::BareMetal { id: default_id, ram_budget_mb: 15 },
                                    "Tool_UIEngine" => EngineTool::UIEngine { id: default_id, ram_budget_mb: 25 },
                                    "Tool_Camera" => EngineTool::Camera { id: default_id, ram_budget_mb: 10 },
                                    "Tool_Crate" => EngineTool::Crate { id: default_id, ram_budget_mb: 30 },
                                    _ => EngineTool::BareMetal { id: default_id, ram_budget_mb: 10 },
                                };
                                tools.push(tool);
                            }
                            self.advance();
                            if let Token::Comma = self.current_token {
                                self.advance();
                            }
                        }
                    }
                }
                _ => {}
            }
            self.advance();
        }

        VoltAST {
            kernel_name,
            ram_limit_mb,
            tools,
        }
    }
}
