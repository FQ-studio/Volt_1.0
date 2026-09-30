extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use crate::lexer::Lexer;

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

#[allow(dead_code)]
pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Self { lexer }
    }

    pub fn parse_volt_system(&mut self) -> VoltAST {
        VoltAST {
            kernel_name: String::from("GravityKernel"),
            ram_limit_mb: 100,
            tools: Vec::new(),
        }
    }
}
