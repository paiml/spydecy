//! Python AST parser using PyO3
//!
//! This module uses PyO3 to invoke Python's `ast` module for parsing.

use anyhow::{Context, Result};
use pyo3::prelude::*;
use pyo3::types::PyModule;
use serde::{Deserialize, Serialize};

/// Python AST node (simplified representation)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PythonAST {
    /// Node type (e.g., "Module", "FunctionDef", "Call")
    pub node_type: String,
    /// Line number
    pub lineno: Option<usize>,
    /// Column offset
    pub col_offset: Option<usize>,
    /// Child nodes
    pub children: Vec<PythonAST>,
    /// Node attributes (name, value, etc.)
    pub attributes: std::collections::HashMap<String, String>,
}

impl PythonAST {
    /// Create a new AST node
    #[must_use]
    pub fn new(node_type: String) -> Self {
        Self {
            node_type,
            lineno: None,
            col_offset: None,
            children: Vec::new(),
            attributes: std::collections::HashMap::new(),
        }
    }
}

/// Parse Python source code into AST
///
/// # Errors
///
/// Returns an error if the Python code cannot be parsed
pub fn parse(source: &str, filename: &str) -> Result<PythonAST> {
    Python::with_gil(|py| parse_with_python(py, source, filename))
}

/// Parse Python source using Python's ast module
fn parse_with_python(py: Python<'_>, source: &str, filename: &str) -> Result<PythonAST> {
    // Import Python's ast module
    let ast_module =
        PyModule::import_bound(py, "ast").context("Failed to import Python ast module")?;

    // Parse the source code
    let ast_obj = ast_module
        .call_method1("parse", (source, filename))
        .context("Failed to parse Python source code")?;

    // Convert Python AST to our simplified AST representation
    extract_ast_node(&ast_obj)
}

/// Extract AST node information from Python object
fn extract_ast_node(obj: &Bound<'_, PyAny>) -> Result<PythonAST> {
    let node_type = obj
        .getattr("__class__")?
        .getattr("__name__")?
        .extract::<String>()?;

    let mut ast = PythonAST::new(node_type.clone());

    // Extract line number and column offset
    extract_location_info(obj, &mut ast);

    // Extract node-specific attributes
    extract_node_attributes(obj, &node_type, &mut ast)?;

    Ok(ast)
}

/// Extract location information (line number and column offset)
fn extract_location_info(obj: &Bound<'_, PyAny>, ast: &mut PythonAST) {
    if let Ok(lineno) = obj.getattr("lineno") {
        ast.lineno = lineno.extract().ok();
    }
    if let Ok(col_offset) = obj.getattr("col_offset") {
        ast.col_offset = col_offset.extract().ok();
    }
}

/// Extract node-specific attributes based on node type
fn extract_node_attributes(
    obj: &Bound<'_, PyAny>,
    node_type: &str,
    ast: &mut PythonAST,
) -> Result<()> {
    match node_type {
        "Module" => extract_module_attrs(obj, ast)?,
        "FunctionDef" => extract_function_def_attrs(obj, ast)?,
        "Return" => extract_return_attrs(obj, ast)?,
        "Call" => extract_call_attrs(obj, ast)?,
        "Name" => extract_name_attrs(obj, ast)?,
        _ => extract_default_attrs(obj, ast)?,
    }
    Ok(())
}

/// Extract Module node attributes
fn extract_module_attrs(obj: &Bound<'_, PyAny>, ast: &mut PythonAST) -> Result<()> {
    if let Ok(body) = obj.getattr("body") {
        ast.children = extract_list(&body)?;
    }
    Ok(())
}

/// Extract FunctionDef node attributes
fn extract_function_def_attrs(obj: &Bound<'_, PyAny>, ast: &mut PythonAST) -> Result<()> {
    if let Ok(name) = obj.getattr("name") {
        ast.attributes.insert("name".to_string(), name.extract()?);
    }
    if let Ok(body) = obj.getattr("body") {
        ast.children = extract_list(&body)?;
    }
    Ok(())
}

/// Extract Return node attributes
fn extract_return_attrs(obj: &Bound<'_, PyAny>, ast: &mut PythonAST) -> Result<()> {
    if let Ok(value) = obj.getattr("value") {
        if !value.is_none() {
            ast.children.push(extract_ast_node(&value)?);
        }
    }
    Ok(())
}

/// Extract Call node attributes
fn extract_call_attrs(obj: &Bound<'_, PyAny>, ast: &mut PythonAST) -> Result<()> {
    if let Ok(func) = obj.getattr("func") {
        ast.children.push(extract_ast_node(&func)?);
    }
    if let Ok(args) = obj.getattr("args") {
        ast.children.extend(extract_list(&args)?);
    }
    Ok(())
}

/// Extract Name node attributes
fn extract_name_attrs(obj: &Bound<'_, PyAny>, ast: &mut PythonAST) -> Result<()> {
    if let Ok(id) = obj.getattr("id") {
        ast.attributes.insert("id".to_string(), id.extract()?);
    }
    Ok(())
}

/// Extract default attributes for unknown node types
#[allow(clippy::unnecessary_wraps)]
fn extract_default_attrs(obj: &Bound<'_, PyAny>, ast: &mut PythonAST) -> Result<()> {
    if let Ok(value) = obj.getattr("value") {
        if !value.is_none() {
            if let Ok(child) = extract_ast_node(&value) {
                ast.children.push(child);
            }
        }
    }
    Ok(())
}

/// Extract a list of AST nodes
fn extract_list(list: &Bound<'_, PyAny>) -> Result<Vec<PythonAST>> {
    let mut nodes = Vec::new();
    for item in list.iter()? {
        let item = item?;
        nodes.push(extract_ast_node(&item)?);
    }
    Ok(nodes)
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
        let ast = parse(source, "test.py").unwrap();
        assert_eq!(ast.node_type, "Module");
        assert!(!ast.children.is_empty());
    }

    #[test]
    fn test_parse_with_type_hints() {
        let source = r"
def my_len(x: list) -> int:
    return len(x)
";
        let ast = parse(source, "test.py").unwrap();
        assert_eq!(ast.node_type, "Module");
    }

    #[test]
    fn test_parse_invalid_syntax() {
        let source = "def invalid syntax here";
        let result = parse(source, "test.py");
        assert!(result.is_err());
    }
}
