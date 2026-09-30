use crate::parser::{EngineTool, VoltAST};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TargetArch {
    AArch64, // MediaTek Helio G81 Ultra / ARM64 (Redmi 14C)
    X86_64,  // PC / Laptop / QEMU Emulator
    RiscV64, // Papan RISC-V 64-bit Bare-Metal
}

pub struct CodeGenerator {
    target: TargetArch,
    asm_output: String,
}

impl CodeGenerator {
    pub fn new(target: TargetArch) -> Self {
        Self {
            target,
            asm_output: String::new(),
        }
    }

    /// Menterjemahkan VoltAST mengikut Seni Bina Sasaran (Cross-Compilation)
    pub fn generate(&mut self, ast: &VoltAST) -> String {
        self.asm_output.clear();

        match self.target {
            TargetArch::AArch64 => self.generate_aarch64(ast),
            TargetArch::X86_64 => self.generate_x86_64(ast),
            TargetArch::RiscV64 => self.generate_riscv64(ast),
        }
    }

    // =========================================================================
    // 1. KOD PERAKITAN AARCH64 (ARM64)
    // =========================================================================
    fn generate_aarch64(&mut self, ast: &VoltAST) -> String {
        self.asm_output.push_str("// === GRAVITY KERNEL [AARCH64 TARGET] ===\n");
        self.asm_output.push_str(".section .text\n.global _start\n\n_start:\n");
        self.asm_output.push_str("    ldr x0, =0x40000000\n");
        self.asm_output.push_str("    mov x1, #100\n");
        self.asm_output.push_str("    bl  init_memory_manager\n\n");

        for tool in &ast.tools {
            let (id, ram) = self.extract_tool_info(tool);
            self.asm_output.push_str(&format!(
                "    // Service: {}\n    mov x2, #{}\n    bl  spawn_service_{}\n\n",
                id, ram, id
            ));
        }

        self.asm_output.push_str("kernel_main_loop:\n    wfi\n    b   kernel_main_loop\n");
        self.asm_output.clone()
    }

    // =========================================================================
    // 2. KOD PERAKITAN X86_64 (INTEL / AMD 64-BIT)
    // =========================================================================
    fn generate_x86_64(&mut self, ast: &VoltAST) -> String {
        self.asm_output.push_str("// === GRAVITY KERNEL [X86_64 TARGET] ===\n");
        self.asm_output.push_str(".section .text\n.global _start\n\n_start:\n");
        self.asm_output.push_str("    mov rdi, 0x40000000\n");
        self.asm_output.push_str("    mov rsi, 100\n");
        self.asm_output.push_str("    call init_memory_manager\n\n");

        for tool in &ast.tools {
            let (id, ram) = self.extract_tool_info(tool);
            self.asm_output.push_str(&format!(
                "    // Service: {}\n    mov rdx, {}\n    call spawn_service_{}\n\n",
                id, ram, id
            ));
        }

        self.asm_output.push_str("kernel_main_loop:\n    hlt\n    jmp kernel_main_loop\n");
        self.asm_output.clone()
    }

    // =========================================================================
    // 3. KOD PERAKITAN RISC-V 64-BIT (RV64)
    // =========================================================================
    fn generate_riscv64(&mut self, ast: &VoltAST) -> String {
        self.asm_output.push_str("// === GRAVITY KERNEL [RISC-V 64 TARGET] ===\n");
        self.asm_output.push_str(".section .text\n.global _start\n\n_start:\n");
        self.asm_output.push_str("    li a0, 0x40000000\n");
        self.asm_output.push_str("    li a1, 100\n");
        self.asm_output.push_str("    jal ra, init_memory_manager\n\n");

        for tool in &ast.tools {
            let (id, ram) = self.extract_tool_info(tool);
            self.asm_output.push_str(&format!(
                "    // Service: {}\n    li a2, {}\n    jal ra, spawn_service_{}\n\n",
                id, ram, id
            ));
        }

        self.asm_output.push_str("kernel_main_loop:\n    wfi\n    j   kernel_main_loop\n");
        self.asm_output.clone()
    }

    /// Fungsi pembantu untuk mengekstrak maklumat ID dan RAM daripada enum EngineTool
    fn extract_tool_info(&self, tool: &EngineTool) -> (String, u32) {
        match tool {
            EngineTool::AILLM { id, ram_budget_mb, .. } => (id.clone(), *ram_budget_mb),
            EngineTool::BareMetal { id, ram_budget_mb, .. } => (id.clone(), *ram_budget_mb),
            EngineTool::UIEngine { id, ram_budget_mb, .. } => (id.clone(), *ram_budget_mb),
            EngineTool::Camera { id, ram_budget_mb, .. } => (id.clone(), *ram_budget_mb),
            EngineTool::Crate { id, ram_budget_mb, .. } => (id.clone(), *ram_budget_mb),
        }
    }
}
