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
#![allow(
    clippy::doc_markdown,
    clippy::single_match,
    clippy::module_name_repetitions,
    clippy::borrow_as_ptr,
    clippy::ptr_as_ptr,
    clippy::ptr_cast_constness,
    clippy::cast_sign_loss,
    clippy::str_to_string,
    clippy::unwrap_used,
    clippy::ref_option,
    clippy::wildcard_imports
)]

pub mod cpython;
pub mod decy_adapter;
pub mod hir_converter;
pub mod parser;

use anyhow::Result;
use spydecy_hir::c::CHIR;
use std::path::Path;

/// Parse C source code into HIR using decy-parser
///
/// # Errors
///
/// Returns an error if the C code cannot be parsed or converted to HIR
pub fn parse_c(source: &str, _filename: &str) -> Result<CHIR> {
    // Prepend CPython type declarations for decy-parser compatibility
    let cpython_types = r"
typedef long Py_ssize_t;
struct _object;
typedef struct _object PyObject;
struct PyListObject;
typedef struct PyListObject PyListObject;
struct PyDictObject;
typedef struct PyDictObject PyDictObject;
";
    let enhanced_source = format!("{cpython_types}{source}");

    // Use decy-parser for comprehensive C parsing
    let decy_parser = decy_parser::CParser::new()?;
    let decy_ast = decy_parser.parse(&enhanced_source)?;

    // Convert decy AST to spydecy CAST
    let cast = decy_adapter::convert_decy_ast_to_cast(&decy_ast)?;

    // Convert CAST to HIR (existing pipeline)
    hir_converter::convert_to_hir(&cast)
}

/// Parse C source code using legacy parser (for comparison/fallback)
///
/// # Errors
///
/// Returns an error if the C code cannot be parsed or converted to HIR
#[allow(dead_code)]
fn parse_c_legacy(source: &str, filename: &str) -> Result<CHIR> {
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
        assert!(
            result.is_ok(),
            "Failed to parse simple C: {:?}",
            result.err()
        );
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
        assert!(
            result.is_ok(),
            "Failed to parse CPython code: {:?}",
            result.err()
        );
    }
}
