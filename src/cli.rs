extern crate alloc;
use alloc::string::ToString;

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::codegen::CodeGenerator;
use crate::sql_bridge::SQLBridge;

pub struct VoltCLI;

impl VoltCLI {
    pub fn parse_and_execute() {
        let sample_input = "@Kernel GravityKernel hardware_ram_cap: 100MB tools: [Tool_UIEngine, Tool_BareMetal]";
        let lexer = Lexer::new(sample_input);
        let mut parser = Parser::new(lexer);
        let ast = parser.parse_volt_system();

        let mut codegen = CodeGenerator::new();
        let _asm = codegen.generate(&ast);

        let mut sql_engine = SQLBridge::new();
        sql_engine.add_to_mirror("xzuff-ui".to_string(), alloc::vec![0x00, 0xFF, 0xFE]);
    }
}
