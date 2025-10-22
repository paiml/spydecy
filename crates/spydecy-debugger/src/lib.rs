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

pub mod visualize;

use anyhow::Result;
use std::path::Path;

/// Visualize Python AST for debugging
///
/// # Errors
///
/// Returns an error if the file cannot be read or parsed
pub fn visualize_python_ast(file_path: &Path) -> Result<String> {
    visualize::visualize_python(file_path)
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
}
