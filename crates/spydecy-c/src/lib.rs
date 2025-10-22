//! C transpiler - converts C AST to Spydecy HIR
//!
//! This module uses clang-sys to parse C code (especially CPython implementation)
//! and converts it to Spydecy's Unified HIR for cross-layer optimization.
//!
//! # Sprint 3 Deliverables
//!
//! - C AST parser (clang-sys, following decy's approach)
//! - CPython API pattern recognition
//! - C → HIR conversion
//! - C-API trait system (pluggable architecture)

#![warn(missing_docs, clippy::all, clippy::pedantic)]
// Note: clang-sys requires unsafe for FFI, allowed only in this crate
#![allow(unsafe_code)]

pub mod parser;
pub mod cpython;
pub mod hir_converter;

use anyhow::Result;
use spydecy_hir::c::CHIR;
use std::path::Path;

/// Parse C source code into HIR
///
/// # Errors
///
/// Returns an error if the C code cannot be parsed or converted to HIR
pub fn parse_c(source: &str, filename: &str) -> Result<CHIR> {
    let ast = parser::parse(source, filename)?;
    hir_converter::convert_to_hir(&ast)
}

/// Parse C file into HIR
///
/// # Errors
///
/// Returns an error if the file cannot be read, parsed, or converted to HIR
pub fn parse_c_file(file_path: &Path) -> Result<CHIR> {
    let source = std::fs::read_to_string(file_path)?;
    let filename = file_path.to_string_lossy().to_string();
    parse_c(&source, &filename)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_function() {
        let source = r"
int add(int a, int b) {
    return a + b;
}
";
        let result = parse_c(source, "test.c");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_cpython_function() {
        let source = r"
static Py_ssize_t
list_length(PyListObject *self) {
    return Py_SIZE(self);
}
";
        let result = parse_c(source, "listobject.c");
        assert!(result.is_ok());
    }
}
