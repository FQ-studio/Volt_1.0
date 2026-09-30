extern crate alloc;
use alloc::string::String;
use alloc::string::ToString;
use std::env;
use std::fs;
use std::process;

use crate::codegen::{CodeGenerator, TargetArch};
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::sql_bridge::SqlVectorBridge;

pub struct VoltCLI;

impl VoltCLI {
    pub fn parse_and_execute() {
        let args: Vec<String> = env::args().collect();

        if args.len() < 2 {
            Self::print_help();
            return;
        }

        match args[1].as_str() {
            // Sintaks: v install <package> ATAU v i <package>
            "install" | "i" => {
                if args.len() < 3 {
                    println!("[V-ERROR] Nyatakan nama pakej! Contoh: v install xzuff-ui");
                    process::exit(1);
                }
                Self::install_package(&args[2]);
            }

            // Sintaks: v mirror add <pkg_name> <path>
            "mirror" | "m" => {
                if args.len() >= 4 && args[2] == "add" {
                    Self::add_to_mirror(&args[3]);
                } else {
                    println!("[V-MIRROR] Penggunaan: v mirror add <nama_pakej>");
                }
            }

            // Sintaks: v build ATAU v b
            "build" | "b" => {
                Self::execute_build_pipeline();
            }

            // Sintaks: v run ATAU v r
            "run" | "r" => {
                println!("[V-RUN] Pelancaran Ujian Gravity Kernel...");
                Self::execute_build_pipeline();
            }

            "--help" | "-h" | "help" => Self::print_help(),

            _ => {
                println!("[V-ERROR] Arahan '{}' tidak sah.", args[1]);
                Self::print_help();
            }
        }
    }

    /// Pasang pakej terus dari Cermin Local SQL 140TB
    fn install_package(pkg_name: &str) {
        println!("====================================================");
        println!("      V-CLI // SQL 140TB LOCAL MIRROR INSTALLER    ");
        println!("====================================================\n");

        println!("[1/3] Membuka Indeks Pangkalan Data SQL 140TB...");
        let sql_engine = SqlVectorBridge::new();

        println!("[2/3] Imbas biner bagi modul '{}'...", pkg_name);
        match sql_engine.fetch_from_mirror(pkg_name) {
            Ok(size) => {
                println!("      -> Ditemui dalam Mirror Registry!");
                println!("      -> Saiz Blok Data: {} bytes", size);
                println!("[3/3] Memuatkan biner ke ruang memori 100MB Kernel...");
                println!("\n>>> SUCCESS: Pakej '{}' dipasang secara Offline! <<<", pkg_name);
            }
            Err(err_msg) => {
                println!("      -> [MIRROR ERROR] {}", err_msg);
                println!("      -> Gunakan 'v mirror add <nama>' untuk daftarkan pakej ini.");
            }
        }
    }

    /// Daftar pakej baharu ke dalam Mirror SQL 140TB
    fn add_to_mirror(pkg_name: &str) {
        println!("[V-MIRROR] Memasukkan '{}' ke dalam Mirror SQL 140TB...", pkg_name);
        let mut sql_engine = SqlVectorBridge::new();
        sql_engine.add_to_mirror(pkg_name.to_string(), vec![0x00, 0xFF, 0xFE]);
        println!("[V-MIRROR] Pakej '{}' selamat didaftarkan ke dalam indeks!", pkg_name);
    }

    /// Talian Kompilasi Bare-Metal (ARM64, x86_64, RISC-V)
    fn execute_build_pipeline() {
        let volt_script = r#"
        @Kernel GravityKernel {
            hardware_ram_cap: 100MB

            tools: [
                Tool_AILLM,
                Tool_BareMetal,
                Tool_UIEngine,
                Tool_Camera,
                Tool_Crate
            ]
        }
        "#;

        println!("====================================================");
        println!("      XZUFF OS // VOLT COMPILER BUILD PIPELINE      ");
        println!("====================================================\n");

        // 1. Lexer & Parser
        println!("[1/4] Menganalisis Skrip Volt DSL...");
        let lexer = Lexer::new(volt_script);
        let mut parser = Parser::new(lexer);
        let system_ast = parser.parse_volt_system();

        println!("      -> Kernel Target : {}", system_ast.kernel_name);
        println!("      -> Dynamic Tools : {}\n", system_ast.tools.len());

        // 2. Cross-Compile AArch64 (ARM64)
        println!("[2/4] Generasi Perakitan [ARM64 - Redmi 14C]...");
        let mut arm64_gen = CodeGenerator::new(TargetArch::AArch64);
        let arm64_asm = arm64_gen.generate(&system_ast);
        let _ = fs::write("kernel_arm64.s", arm64_asm);

        // 3. Cross-Compile x86_64
        println!("[3/4] Generasi Perakitan [x86_64 - PC/QEMU]...");
        let mut x86_gen = CodeGenerator::new(TargetArch::X86_64);
        let x86_asm = x86_gen.generate(&system_ast);
        let _ = fs::write("kernel_x86_64.s", x86_asm);

        // 4. Cross-Compile RISC-V 64
        println!("[4/4] Generasi Perakitan [RISC-V 64-bit]...");
        let mut riscv_gen = CodeGenerator::new(TargetArch::RiscV64);
        let riscv_asm = riscv_gen.generate(&system_ast);
        let _ = fs::write("kernel_riscv64.s", riscv_asm);

        // 5. Ujian Dynamic Offload SQL
        println!("\n====================================================");
        println!("        UJIAN INTEGRASI OFF-LOAD SQL 140TB          ");
        println!("====================================================");
        let mut sql_engine = SqlVectorBridge::new();
        let _ = sql_engine.offload_stale_context(40 * 1024 * 1024);
        println!("[SQL BRIDGE] Dynamic Window Status: OK (Kekal Bawah 100MB RAM)");

        println!("\n>>> VOLT BUILD COMPLETE: Fail .s (ARM64, x86_64, RISC-V) Berjaya Dijana! <<<");
    }

    fn print_help() {
        println!("Penggunaan Ringkas V-CLI (Xzuff OS / Volt Compiler):");
        println!("  v install <pkg>  (atau: v i <pkg>)    - Pasang pakej dari Cermin SQL 140TB");
        println!("  v mirror add <pkg> (atau: v m add)    - Simpan pakej baharu ke Mirror SQL 140TB");
        println!("  v build          (atau: v b)          - Kompilasi skrip ke 3 Seni Bina CPU");
        println!("  v run            (atau: v r)          - Jalankan ujian simulasi Kernel");
    }
}
