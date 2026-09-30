use crate::lexer::{Lexer, Token};

// =========================================================================
// 1. STRUKTUR DATA AST (ABSTRACT SYNTAX TREE)
// =========================================================================

#[derive(Debug, PartialEq)]
pub enum EngineTool {
    // 1. AI LLM Engine (Gravity AI)
    AILLM {
        id: String,
        context_mode: String,
        ram_budget_mb: u32,
    },
    // 2. Bare Metal System Control
    BareMetal {
        id: String,
        arch: String,
        ram_budget_mb: u32,
    },
    // 3. Dynamic UI Engine
    UIEngine {
        id: String,
        renderer: String,
        ram_budget_mb: u32,
    },
    // 4. Camera & Spatial Vision Pipeline
    Camera {
        id: String,
        features: Vec<String>,
        ram_budget_mb: u32,
    },
    // 5. Crate / Package Manager
    Crate {
        id: String,
        cargo_bridge: bool,
        ram_budget_mb: u32,
    },
}

#[derive(Debug)]
pub struct VoltAST {
    pub kernel_name: String,
    pub ram_limit_mb: u32,
    pub tools: Vec<EngineTool>, // Memuatkan kesemua 5-in-1 Tools
}

// =========================================================================
// 2. LOGIK ENJIN PARSER
// =========================================================================

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    /// Inisialisasi parser dan baca token pertama daripada Lexer
    pub fn new(mut lexer: Lexer<'a>) -> Self {
        let current_token = lexer.next_token();
        Self { lexer, current_token }
    }

    /// Bergerak ke token seterusnya dalam skrip Volt
    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    /// Memproses skrip Volt DSL dan menghasilkan struktur AST lengkap
    pub fn parse_volt_system(&mut self) -> VoltAST {
        let kernel_name = String::from("GravityKernel");
        let mut tools = Vec::new();

        // Mengimbas kesemua token sehingga tamat fail (EOF)
        while self.current_token != Token::EOF {
            match &self.current_token {
                Token::Identifier(id) if id == "Tool_AILLM" => {
                    tools.push(EngineTool::AILLM {
                        id: String::from("gravity_ai_250b"),
                        context_mode: String::from("DynamicSlidingWindow"),
                        ram_budget_mb: 40,
                    });
                    self.advance();
                }
                Token::Identifier(id) if id == "Tool_BareMetal" => {
                    tools.push(EngineTool::BareMetal {
                        id: String::from("arm64_hal"),
                        arch: String::from("aarch64"),
                        ram_budget_mb: 15,
                    });
                    self.advance();
                }
                Token::Identifier(id) if id == "Tool_UIEngine" => {
                    tools.push(EngineTool::UIEngine {
                        id: String::from("xzuff_compositor"),
                        renderer: String::from("Volt_Native"),
                        ram_budget_mb: 20,
                    });
                    self.advance();
                }
                Token::Identifier(id) if id == "Tool_Camera" => {
                    tools.push(EngineTool::Camera {
                        id: String::from("spatial_vision"),
                        features: vec![String::from("OCR_Scanner"), String::from("CSI_Stream")],
                        ram_budget_mb: 15,
                    });
                    self.advance();
                }
                Token::Identifier(id) if id == "Tool_Crate" => {
                    tools.push(EngineTool::Crate {
                        id: String::from("xzuff_crate_hub"),
                        cargo_bridge: true,
                        ram_budget_mb: 10,
                    });
                    self.advance();
                }
                _ => self.advance(),
            }
        }

        // Kembalikan AST sistem dengan peruntukan RAM tepat 100MB
        VoltAST {
            kernel_name,
            ram_limit_mb: 100, // Total: 40 + 15 + 20 + 15 + 10 = 100MB
            tools,
        }
    }
}
