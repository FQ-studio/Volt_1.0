extern crate alloc;
use alloc::string::String;
use alloc::string::ToString;
mod lexer;
mod parser;
mod codegen;
mod sql_bridge;
mod cli;

use cli::VoltCLI;

fn main() {
    VoltCLI::parse_and_execute();
}
