//! Debugger Command Structures
//!
//! Defines commands available in the interactive debugger REPL.

use std::fmt;

/// Debugger commands
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    /// Step to next transpilation phase
    Step,
    /// Continue until breakpoint or completion
    Continue,
    /// Visualize current state
    Visualize,
    /// Inspect a specific target
    Inspect(String),
    /// Add a breakpoint
    Break(Breakpoint),
    /// List all breakpoints
    ListBreakpoints,
    /// Clear a breakpoint
    ClearBreakpoint(usize),
    /// Show help
    Help,
    /// Quit debugger
    Quit,
}

/// Breakpoint types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Breakpoint {
    /// Break when optimizer eliminates a boundary
    BoundaryElimination,
    /// Break when entering a specific phase
    Phase(String),
    /// Break when processing specific function
    Function(String),
}

impl fmt::Display for Breakpoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BoundaryElimination => write!(f, "Boundary Elimination"),
            Self::Phase(phase) => write!(f, "Phase: {phase}"),
            Self::Function(func) => write!(f, "Function: {func}"),
        }
    }
}

/// Parse command from user input
///
/// # Errors
///
/// Returns error if command syntax is invalid
pub fn parse_command(input: &str) -> Result<Command, String> {
    let input = input.trim();

    if input.is_empty() {
        return Ok(Command::Step); // Default to step
    }

    let parts: Vec<&str> = input.split_whitespace().collect();

    match parts[0] {
        "step" | "s" => Ok(Command::Step),
        "continue" | "c" => Ok(Command::Continue),
        "visualize" | "v" => Ok(Command::Visualize),
        "inspect" | "i" => {
            if parts.len() < 2 {
                Err("inspect requires a target".to_owned())
            } else {
                Ok(Command::Inspect(parts[1..].join(" ")))
            }
        }
        "break" | "b" => {
            if parts.len() < 2 {
                Err("break requires a breakpoint type".to_owned())
            } else {
                parse_breakpoint(&parts[1..])
            }
        }
        "list" | "l" => Ok(Command::ListBreakpoints),
        "clear" => {
            if parts.len() < 2 {
                Err("clear requires breakpoint number".to_owned())
            } else {
                parts[1]
                    .parse::<usize>()
                    .map(Command::ClearBreakpoint)
                    .map_err(|_| "Invalid breakpoint number".to_owned())
            }
        }
        "help" | "h" | "?" => Ok(Command::Help),
        "quit" | "q" | "exit" => Ok(Command::Quit),
        _ => Err(format!(
            "Unknown command: '{}'. Type 'help' for commands.",
            parts[0]
        )),
    }
}

fn parse_breakpoint(parts: &[&str]) -> Result<Command, String> {
    match parts[0] {
        "boundary" => Ok(Command::Break(Breakpoint::BoundaryElimination)),
        "phase" => {
            if parts.len() < 2 {
                Err("break phase requires phase name".to_owned())
            } else {
                Ok(Command::Break(Breakpoint::Phase(parts[1..].join(" "))))
            }
        }
        "function" | "fn" => {
            if parts.len() < 2 {
                Err("break function requires function name".to_owned())
            } else {
                Ok(Command::Break(Breakpoint::Function(parts[1].to_owned())))
            }
        }
        _ => Err(format!("Unknown breakpoint type: '{}'", parts[0])),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_step() {
        assert_eq!(parse_command("step").unwrap(), Command::Step);
        assert_eq!(parse_command("s").unwrap(), Command::Step);
        assert_eq!(parse_command("").unwrap(), Command::Step);
    }

    #[test]
    fn test_parse_quit() {
        assert_eq!(parse_command("quit").unwrap(), Command::Quit);
        assert_eq!(parse_command("q").unwrap(), Command::Quit);
    }

    #[test]
    fn test_parse_inspect() {
        assert_eq!(
            parse_command("inspect python_hir").unwrap(),
            Command::Inspect("python_hir".to_owned())
        );
    }

    #[test]
    fn test_parse_breakpoint() {
        assert_eq!(
            parse_command("break boundary").unwrap(),
            Command::Break(Breakpoint::BoundaryElimination)
        );
    }
}
