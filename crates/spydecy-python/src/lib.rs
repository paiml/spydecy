//! Python transpiler - converts Python AST to Spydecy HIR
//!
//! This module uses PyO3 to parse Python code into AST, then converts
//! it to Spydecy's Unified HIR for cross-layer optimization.
//!
//! # Sprint 2 Deliverables
//!
//! - Python AST parser (PyO3)
//! - Type hint extraction
//! - Python → HIR conversion
//! - First debugger feature: `spydecy debug visualize python-ast`

#![warn(missing_docs, clippy::all, clippy::pedantic)]
#![deny(unsafe_code)]

pub mod parser;
pub mod type_extractor;
pub mod hir_converter;

use anyhow::Result;
use spydecy_hir::python::PythonHIR;

/// Parse Python source code into HIR
///
/// # Errors
///
/// Returns an error if the Python code cannot be parsed or converted to HIR
pub fn parse_python(source: &str, filename: &str) -> Result<PythonHIR> {
    let ast = parser::parse(source, filename)?;
    hir_converter::convert_to_hir(&ast)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_function() {
        let source = r"
def my_len(x):
    return len(x)
";
        let result = parse_python(source, "test.py");
        assert!(result.is_ok());
    }
}
