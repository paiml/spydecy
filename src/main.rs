//! Spydecy - Self-Hosted Python/C-to-Rust Compiler-Debugger
//!
//! Main CLI entry point for the Spydecy transpiler.

#![warn(missing_docs, clippy::all, clippy::pedantic)]
#![deny(unsafe_code)]

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};

/// Spydecy CLI
#[derive(Parser)]
#[command(name = "spydecy")]
#[command(about = "Self-hosted Python/C-to-Rust compiler-debugger", long_about = None)]
#[command(version)]
struct Cli {
    /// Subcommand to execute
    #[command(subcommand)]
    command: Commands,
}

/// Available commands
#[derive(Subcommand)]
enum Commands {
    /// Compile Python+C to Rust using full pipeline
    Compile {
        /// Python source file
        #[arg(long)]
        python: PathBuf,

        /// C source file
        #[arg(long)]
        c: PathBuf,

        /// Output Rust file
        #[arg(short, long)]
        output: PathBuf,

        /// Enable verbose output
        #[arg(short, long)]
        verbose: bool,
    },

    /// Start interactive debugger
    Debug {
        /// Subcommand for debug mode
        #[command(subcommand)]
        mode: DebugMode,
    },

    /// Display version and status information
    Info,
}

/// Debug mode subcommands
#[derive(Subcommand)]
enum DebugMode {
    /// Visualize AST of a file
    Visualize {
        /// Source file to visualize
        file: PathBuf,
    },

    /// Step through transpilation interactively
    Step {
        /// Python source file
        #[arg(long)]
        python: PathBuf,

        /// C source file
        #[arg(long)]
        c: PathBuf,
    },
}

fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Compile {
            python,
            c,
            output,
            verbose,
        } => compile_command(&python, &c, &output, verbose),
        Commands::Debug { mode } => match mode {
            DebugMode::Visualize { file } => debug_visualize_command(&file),
            DebugMode::Step { python, c } => debug_step_command(python, c),
        },
        Commands::Info => {
            info_command();
            Ok(())
        }
    };

    if let Err(e) = result {
        eprintln!("❌ Error: {e:#}");
        std::process::exit(1);
    }
}

/// Extract Python call from module
fn extract_python_call(
    python_hir: spydecy_hir::python::PythonHIR,
) -> Result<spydecy_hir::python::PythonHIR> {
    use spydecy_hir::python::PythonHIR;

    if let PythonHIR::Module { body, .. } = python_hir {
        if let Some(PythonHIR::Function {
            body: func_body, ..
        }) = body.first()
        {
            // Extract the Call from inside the Return statement
            if let Some(PythonHIR::Return {
                value: Some(call), ..
            }) = func_body.first()
            {
                return Ok(call.as_ref().clone());
            }
            anyhow::bail!("Expected return statement with call in function body");
        }
        anyhow::bail!("Expected function in Python module");
    }
    anyhow::bail!("Expected Python module");
}

/// Extract C function from translation unit
fn extract_c_function(c_hir_module: spydecy_hir::c::CHIR) -> Result<spydecy_hir::c::CHIR> {
    use spydecy_hir::c::CHIR;

    if let CHIR::TranslationUnit { declarations, .. } = c_hir_module {
        declarations
            .first()
            .context("C file has no declarations")
            .cloned()
    } else {
        anyhow::bail!("Expected C TranslationUnit")
    }
}

/// Parse Python file to HIR
fn parse_python_file(path: &Path) -> Result<spydecy_hir::python::PythonHIR> {
    use spydecy_python::parse_python;

    let python_source = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read Python file: {}", path.display()))?;

    parse_python(&python_source, path.to_str().unwrap_or("input.py"))
        .context("Failed to parse Python source")
}

/// Parse C file to HIR
fn parse_c_file(path: &Path) -> Result<spydecy_hir::c::CHIR> {
    use spydecy_c::parse_c;

    let c_source = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read C file: {}", path.display()))?;

    parse_c(&c_source, path.to_str().unwrap_or("input.c")).context("Failed to parse C source")
}

/// Helper for verbose logging
struct VerboseLogger {
    enabled: bool,
}

impl VerboseLogger {
    const fn new(enabled: bool) -> Self {
        Self { enabled }
    }

    fn log(&self, msg: &str) {
        if self.enabled {
            println!("{msg}");
        }
    }

    fn header(&self) {
        self.log("🚀 Spydecy Full Pipeline Compilation");
        self.log("═══════════════════════════════════════");
    }

    fn step(&self, num: usize, desc: &str) {
        self.log(&format!("\n📝 Step {num}: {desc}"));
    }

    fn success(&self, msg: &str) {
        self.log(&format!("   ✅ {msg}"));
    }

    fn input(&self, path: &Path) {
        self.log(&format!("   Input: {}", path.display()));
    }

    fn output_path(&self, path: &Path) {
        self.log(&format!("   Output: {}", path.display()));
    }

    fn footer(&self) {
        self.log("\n═══════════════════════════════════════");
        self.log("🎉 Compilation successful!");
        self.log("═══════════════════════════════════════\n");
    }
}

/// Compile Python + C to Rust using the full pipeline
fn compile_command(python: &Path, c: &Path, output: &Path, verbose: bool) -> Result<()> {
    use spydecy_codegen::generate_rust;
    use spydecy_hir::unified::Unifier;
    use spydecy_optimizer::OptimizationPipeline;

    let log = VerboseLogger::new(verbose);
    log.header();

    // Step 1: Parse Python
    log.step(1, "Parsing Python source...");
    log.input(python);

    let python_hir = parse_python_file(python)?;

    log.success("Python HIR created");

    // Step 2: Parse C
    log.step(2, "Parsing C source...");
    log.input(c);

    let c_hir_module = parse_c_file(c)?;
    let c_hir = extract_c_function(c_hir_module)?;

    log.success("C HIR created");

    // Step 3: Extract callable from Python (simplified for now)
    log.step(3, "Unifying Python + C...");

    let python_call = extract_python_call(python_hir)?;
    let mut unifier = Unifier::new();
    let unified_hir = unifier
        .unify(&python_call, &c_hir)
        .context("Failed to unify Python and C")?;

    log.success("Unified HIR created");

    // Step 4: Optimize
    log.step(4, "Running optimizer...");

    let pipeline = OptimizationPipeline::standard();
    let optimized = pipeline
        .run(unified_hir)
        .context("Failed to optimize UnifiedHIR")?;

    log.success("Boundary elimination complete");

    // Step 5: Generate Rust code
    log.step(5, "Generating Rust code...");

    let rust_code = generate_rust(&optimized).context("Failed to generate Rust code")?;

    log.success("Rust code generated");

    // Step 6: Write output
    log.step(6, "Writing output...");
    log.output_path(output);

    std::fs::write(output, rust_code.as_bytes())
        .with_context(|| format!("Failed to write output file: {}", output.display()))?;

    log.success("Output written");

    if verbose {
        log.footer();
    } else {
        println!(
            "✅ Compiled: {} + {} → {}",
            python.display(),
            c.display(),
            output.display()
        );
    }

    Ok(())
}

/// Debug visualize command - visualize AST
fn debug_visualize_command(file: &Path) -> Result<()> {
    tracing::info!("Visualizing: {}", file.display());

    // Determine file type by extension
    let extension = file.extension().and_then(|ext| ext.to_str()).unwrap_or("");

    let output = match extension {
        "py" => {
            // Visualize Python AST
            spydecy_debugger::visualize_python_ast(file)
                .context("Failed to visualize Python AST")?
        }
        "c" | "h" => {
            // Visualize C AST with CPython annotations
            spydecy_debugger::visualize_c_ast(file).context("Failed to visualize C AST")?
        }
        _ => {
            anyhow::bail!("Unsupported file extension: '{extension}'. Supported: .py, .c, .h");
        }
    };

    println!("{output}");
    Ok(())
}

/// Debug step command - interactive step-through debugging
fn debug_step_command(python: PathBuf, c: PathBuf) -> Result<()> {
    tracing::info!(
        "Starting interactive debugger: {} + {}",
        python.display(),
        c.display()
    );

    println!("🐛 Starting interactive debugger...\n");
    println!("   Python: {}", python.display());
    println!("   C:      {}", c.display());

    spydecy_debugger::start_interactive_debugger(python, c)
}

/// Info command - display project status
fn info_command() {
    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║         Spydecy - Python/C-to-Rust Transpiler            ║");
    println!("╚═══════════════════════════════════════════════════════════╝");
    println!();
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
    println!("EXTREME TDD - Zero Tolerance Quality");
    println!();
    println!("📊 Status:");
    println!("   ✅ 81/81 tests passing (100%)");
    println!("   ✅ Sprint 4 Complete: Interactive Debugger");
    println!("   ✅ Full pipeline working");
    println!("   ✅ 3 core patterns implemented");
    println!("   ✅ Step-through debugging functional");
    println!();
    println!("🦀 Core Patterns:");
    println!("   • len()       Python len() + C list_length()   → Vec::len()");
    println!("   • append()    Python append() + C PyList_Append() → Vec::push()");
    println!("   • dict.get()  Python get() + C PyDict_GetItem() → HashMap::get()");
    println!();
    println!("📝 Pipeline:");
    println!("   Python source  → PythonHIR   ✅");
    println!("   C source       → CHIR         ✅");
    println!("   Python + C     → UnifiedHIR   ✅");
    println!("   UnifiedHIR     → Optimized    ✅");
    println!("   Optimized      → Rust code    ✅");
    println!();
    println!("🐛 Interactive Debugger:");
    println!("   spydecy debug step --python <file.py> --c <file.c>");
    println!("   • Step through transpilation phases");
    println!("   • Set breakpoints on optimizations");
    println!("   • Visualize state at each step");
    println!();
    println!("Result: Pure Rust code with ZERO FFI, ZERO unsafe!");
    println!();
    println!("📖 Documentation: https://github.com/noahgift/spydecy");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parsing() {
        // Basic smoke test
        let _cli = Cli::parse_from(["spydecy", "info"]);
    }

    #[test]
    fn test_info_command() {
        let cli = Cli::parse_from(["spydecy", "info"]);
        assert!(matches!(cli.command, Commands::Info));
    }
}
