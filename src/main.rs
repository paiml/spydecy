//! Spydecy - Self-Hosted Python/C-to-Rust Compiler-Debugger
//!
//! Main CLI entry point for the Spydecy transpiler.

#![warn(missing_docs, clippy::all, clippy::pedantic)]
#![deny(unsafe_code)]

use clap::{Parser, Subcommand};

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
    /// Transpile Python source to Rust
    TranspilePython {
        /// Input Python file
        input: String,
        /// Output Rust file
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Transpile C source to Rust
    TranspileC {
        /// Input C file
        input: String,
        /// Output Rust file
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Transpile unified Python + C project
    TranspileUnified {
        /// Project directory
        project: String,
        /// Output directory
        #[arg(short, long)]
        output: String,
    },

    /// Start interactive debugger
    Debug {
        /// Source file to debug
        file: String,
        /// Enable visualization
        #[arg(short, long)]
        visualize: bool,
    },

    /// Run Sprint 0 tracer bullet validation
    TracerBullet,

    /// Display version information
    Version,
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

    match cli.command {
        Commands::TranspilePython { input, output } => {
            tracing::info!("Transpiling Python: {} → {:?}", input, output);
            eprintln!("❌ Not yet implemented - Sprint 2");
            eprintln!("📋 See: docs/specification/SPRINT-0-TRACER-BULLET.md");
        }
        Commands::TranspileC { input, output } => {
            tracing::info!("Transpiling C: {} → {:?}", input, output);
            eprintln!("❌ Not yet implemented - Sprint 3");
        }
        Commands::TranspileUnified { project, output } => {
            tracing::info!("Transpiling unified: {} → {}", project, output);
            eprintln!("❌ Not yet implemented - Sprint 4+");
        }
        Commands::Debug { file, visualize } => {
            tracing::info!("Debugging: {} (visualize: {})", file, visualize);

            if visualize {
                // Sprint 2 feature: Visualize Python AST
                match spydecy_debugger::visualize_python_ast(std::path::Path::new(&file)) {
                    Ok(output) => {
                        println!("{output}");
                    }
                    Err(e) => {
                        eprintln!("❌ Error visualizing AST: {e}");
                        std::process::exit(1);
                    }
                }
            } else {
                eprintln!("ℹ️  Use --visualize flag to see AST visualization");
                eprintln!("   Example: spydecy debug --visualize your_file.py");
            }
        }
        Commands::TracerBullet => {
            tracing::info!("Running Sprint 0 tracer bullet");
            eprintln!("🎯 Sprint 0: Tracer Bullet Validation");
            eprintln!("📋 See: docs/specification/SPRINT-0-TRACER-BULLET.md");
            eprintln!();
            eprintln!("This will validate the Unified HIR concept:");
            eprintln!("  Python len() → C list_length() → Rust Vec::len()");
            eprintln!();
            eprintln!("❌ Not yet implemented");
        }
        Commands::Version => {
            println!("spydecy {}", env!("CARGO_PKG_VERSION"));
            println!("EXTREME TDD - Zero Tolerance Quality");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parsing() {
        // Basic smoke test
        let _cli = Cli::parse_from(["spydecy", "version"]);
    }

    #[test]
    fn test_version_command() {
        let cli = Cli::parse_from(["spydecy", "version"]);
        assert!(matches!(cli.command, Commands::Version));
    }
}
