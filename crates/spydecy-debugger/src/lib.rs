//! Spydecy Interactive Debugger
//!
//! This module implements the introspective debugger that allows stepping through
//! the transpilation process to identify issues. Built incrementally alongside
//! the transpiler (per Gemini's recommendation).
//!
//! # Sprint 2 Feature
//!
//! `visualize python-ast` - Display Python AST and HIR conversion

#![warn(missing_docs, clippy::all, clippy::pedantic)]
#![deny(unsafe_code)]
#![allow(
    clippy::module_name_repetitions,
    clippy::format_push_string,
    clippy::str_to_string,
    clippy::unwrap_used,
    clippy::uninlined_format_args
)]

pub mod commands;
pub mod repl;
pub mod state;
pub mod stepper;
pub mod visualize;

use anyhow::Result;
use state::TranspilationState;
use std::path::{Path, PathBuf};

/// Visualize Python AST for debugging
///
/// # Errors
///
/// Returns an error if the file cannot be read or parsed
pub fn visualize_python_ast(file_path: &Path) -> Result<String> {
    visualize::visualize_python(file_path)
}

/// Visualize C AST with `CPython` API annotations for debugging
///
/// # Errors
///
/// Returns an error if the file cannot be read or parsed
pub fn visualize_c_ast(file_path: &Path) -> Result<String> {
    visualize::visualize_c(file_path)
}

/// Start interactive step-through debugging session
///
/// # Errors
///
/// Returns an error if files cannot be read or REPL fails
pub fn start_interactive_debugger(python_file: PathBuf, c_file: PathBuf) -> Result<()> {
    let state = TranspilationState::new(python_file, c_file);
    let stepper = stepper::Stepper::new(state);
    repl::run_repl(stepper)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_visualize_simple_function() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "def my_len(x):\n    return len(x)").unwrap();

        let result = visualize_python_ast(temp_file.path());
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("Module"));
        assert!(output.contains("FunctionDef"));
    }

    #[test]
    fn test_visualize_invalid_syntax() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "def invalid syntax here").unwrap();

        let result = visualize_python_ast(temp_file.path());
        assert!(result.is_err());
    }

    #[test]
    fn test_visualize_c_simple_function() {
        use std::io::Write;
        use tempfile::Builder;

        let mut temp_file = Builder::new().suffix(".c").tempfile().unwrap();
        writeln!(temp_file, "int add(int a, int b) {{\n    return a + b;\n}}").unwrap();
        temp_file.flush().unwrap(); // Ensure content is written

        let result = visualize_c_ast(temp_file.path());
        assert!(
            result.is_ok(),
            "Failed to visualize C: {:?}",
            result.as_ref().err()
        );
        let output = result.unwrap();
        assert!(output.contains("C AST Visualization"));
        assert!(output.contains("FunctionDecl"));
    }
}
