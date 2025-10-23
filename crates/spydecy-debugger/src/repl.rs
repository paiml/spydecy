//! Interactive REPL for stepping through transpilation
//!
//! Provides a command-line interface for debugging.

use crate::commands::{parse_command, Command};
use crate::stepper::Stepper;
use anyhow::Result;
use colored::Colorize;
use std::io::{self, Write};

/// Run interactive REPL session
///
/// # Errors
///
/// Returns error if I/O fails
pub fn run_repl(mut stepper: Stepper) -> Result<()> {
    print_header();
    print_help_hint();

    loop {
        print!("\n{} ", "(spydecy-debug)".blue().bold());
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let command = match parse_command(&input) {
            Ok(cmd) => cmd,
            Err(e) => {
                eprintln!("{} {}", "Error:".red().bold(), e);
                continue;
            }
        };

        match handle_command(command, &mut stepper) {
            Ok(true) => break, // Quit
            Ok(false) => {}
            Err(e) => {
                eprintln!("{} {e:#}", "Error:".red().bold());
            }
        }
    }

    println!("\n{}", "Exiting debugger.".dimmed());
    Ok(())
}

fn handle_command(command: Command, stepper: &mut Stepper) -> Result<bool> {
    match command {
        Command::Step => {
            let phase = stepper.step()?;
            println!(
                "\n{}",
                format!("═══ Step {} ═══", stepper.state().step_count)
                    .cyan()
                    .bold()
            );
            println!("{} {}", "Phase:".green(), phase.name());
            print_current_state(stepper);
        }
        Command::Continue => {
            stepper.continue_execution()?;
            println!("{}", "Execution continued.".green());
            print_current_state(stepper);
        }
        Command::Visualize => {
            visualize_state(stepper);
        }
        Command::Inspect(target) => {
            inspect_target(&target, stepper);
        }
        Command::Break(bp) => {
            stepper.add_breakpoint(bp.clone());
            println!("{} {bp}", "Breakpoint added:".green().bold());
        }
        Command::ListBreakpoints => {
            list_breakpoints(stepper);
        }
        Command::ClearBreakpoint(idx) => {
            if stepper.clear_breakpoint(idx) {
                println!("{} {idx}", "Cleared breakpoint:".green().bold());
            } else {
                eprintln!("{} {idx}", "Invalid breakpoint:".red().bold());
            }
        }
        Command::Help => {
            print_help();
        }
        Command::Quit => {
            return Ok(true);
        }
    }
    Ok(false)
}

fn print_header() {
    println!(
        "{}",
        "═══════════════════════════════════════".cyan().bold()
    );
    println!("{}", "   Spydecy Interactive Debugger".cyan().bold());
    println!(
        "{}",
        "═══════════════════════════════════════".cyan().bold()
    );
}

fn print_help_hint() {
    println!(
        "\nType {} for help, {} to step, {} to quit",
        "'help'".yellow(),
        "'step'".yellow(),
        "'quit'".yellow()
    );
}

fn print_help() {
    println!("\n{}", "Available Commands:".green().bold());
    println!(
        "  {} {}     Step to next phase",
        "step, s".yellow(),
        " ".repeat(7)
    );
    println!(
        "  {} {}  Continue until breakpoint",
        "continue, c".yellow(),
        " ".repeat(2)
    );
    println!("  {}  Visualize current state", "visualize, v".yellow());
    println!(
        "  {}  Inspect target (python_hir, c_hir, etc.)",
        "inspect <target>".yellow()
    );
    println!("  {}    Add breakpoint", "break <type>".yellow());
    println!(
        "  {} {}     List breakpoints",
        "list, l".yellow(),
        " ".repeat(7)
    );
    println!(
        "  {} {}  Clear breakpoint",
        "clear <num>".yellow(),
        " ".repeat(3)
    );
    println!(
        "  {} {}    Show this help",
        "help, h, ?".yellow(),
        " ".repeat(4)
    );
    println!("  {} {}  Exit debugger", "quit, q".yellow(), " ".repeat(7));
}

fn print_current_state(stepper: &Stepper) {
    let state = stepper.state();
    println!("  {} {}", "Step:".dimmed(), state.step_count);
    println!("  {} {}", "Phase:".dimmed(), state.phase.name());
}

fn visualize_state(stepper: &Stepper) {
    let state = stepper.state();
    println!(
        "\n{}",
        format!("═══ State at Step {} ═══", state.step_count)
            .cyan()
            .bold()
    );

    if let Some(ref hir) = state.python_hir {
        println!("\n{}", "📄 Python HIR:".green().bold());
        println!("{hir:#?}");
    }

    if let Some(ref hir) = state.c_hir {
        println!("\n{}", "🔧 C HIR:".green().bold());
        println!("{hir:#?}");
    }

    if let Some(ref hir) = state.unified_hir {
        println!("\n{}", "🔗 Unified HIR:".green().bold());
        println!("{hir:#?}");
    }

    if let Some(ref code) = state.rust_code {
        println!("\n{}", "🦀 Rust Code:".green().bold());
        println!("{code}");
    }
}

fn inspect_target(target: &str, stepper: &Stepper) {
    let state = stepper.state();

    match target {
        "python" | "python_hir" => {
            if let Some(ref hir) = state.python_hir {
                println!("{hir:#?}");
            } else {
                println!("{}", "Python HIR not yet available".dimmed());
            }
        }
        "c" | "c_hir" => {
            if let Some(ref hir) = state.c_hir {
                println!("{hir:#?}");
            } else {
                println!("{}", "C HIR not yet available".dimmed());
            }
        }
        "unified" => {
            if let Some(ref hir) = state.unified_hir {
                println!("{hir:#?}");
            } else {
                println!("{}", "Unified HIR not yet available".dimmed());
            }
        }
        "rust" => {
            if let Some(ref code) = state.rust_code {
                println!("{code}");
            } else {
                println!("{}", "Rust code not yet generated".dimmed());
            }
        }
        _ => {
            eprintln!("{} {target}", "Unknown target:".red().bold());
        }
    }
}

fn list_breakpoints(stepper: &Stepper) {
    let breakpoints = stepper.breakpoints();

    if breakpoints.is_empty() {
        println!("{}", "No breakpoints set.".dimmed());
    } else {
        println!("\n{}", "Breakpoints:".green().bold());
        for (i, bp) in breakpoints.iter().enumerate() {
            println!("  {}: {bp}", format!("[{i}]").yellow());
        }
    }
}
