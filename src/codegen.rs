extern crate alloc;
use alloc::string::String;
use alloc::format;

use crate::parser::{EngineTool, VoltAST};

pub struct CodeGenerator {
    asm_output: String,
}

impl CodeGenerator {
    pub fn new() -> Self {
        Self {
            asm_output: String::new(),
        }
    }

    pub fn generate(&mut self, ast: &VoltAST) -> String {
        self.asm_output.clear();
        self.asm_output.push_str("// Volt Bare-Metal Generated ASM\n");
        self.asm_output.push_str(".global _start\n_start:\n");

        for tool in &ast.tools {
            match tool {
                EngineTool::AILLM { id, ram_budget_mb } => {
                    self.asm_output.push_str(&format!("// Tool AILLM: {} ({} MB)\n", id, ram_budget_mb));
                }
                EngineTool::BareMetal { id, ram_budget_mb } => {
                    self.asm_output.push_str(&format!("// Tool BareMetal: {} ({} MB)\n", id, ram_budget_mb));
                }
                EngineTool::UIEngine { id, ram_budget_mb } => {
                    self.asm_output.push_str(&format!("// Tool UIEngine: {} ({} MB)\n", id, ram_budget_mb));
                }
                EngineTool::Camera { id, ram_budget_mb } => {
                    self.asm_output.push_str(&format!("// Tool Camera: {} ({} MB)\n", id, ram_budget_mb));
                }
                EngineTool::Crate { id, ram_budget_mb } => {
                    self.asm_output.push_str(&format!("// Tool Crate: {} ({} MB)\n", id, ram_budget_mb));
                }
            }
        }

        self.asm_output.clone()
    }
}
