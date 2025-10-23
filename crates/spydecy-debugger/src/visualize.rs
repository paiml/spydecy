//! AST visualization for debugging
//!
//! This module provides formatted visualization of ASTs for debugging purposes.

use anyhow::{Context, Result};
use colored::Colorize;
use spydecy_c::{cpython, parser::CAST};
use spydecy_python::parser::PythonAST;
use std::fs;
use std::path::Path;

/// Visualize Python source as AST
///
/// # Errors
///
/// Returns an error if the file cannot be read or parsed
pub fn visualize_python(file_path: &Path) -> Result<String> {
    // Read the source file
    let source = fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read file: {}", file_path.display()))?;

    // Parse to AST
    let filename = file_path.to_string_lossy().to_string();
    let ast = spydecy_python::parser::parse(&source, &filename)
        .context("Failed to parse Python source")?;

    // Format the output
    let mut output = String::new();

    // Header
    output.push_str(&format!(
        "{}",
        "╔══════════════════════════════════════════════════════════╗\n".cyan()
    ));
    output.push_str(&format!(
        "{}",
        "║  Spydecy Debugger: Python AST Visualization             ║\n".cyan()
    ));
    output.push_str(&format!(
        "{}",
        "╚══════════════════════════════════════════════════════════╝\n".cyan()
    ));
    output.push('\n');

    // File info
    output.push_str(&format!("{} {}\n", "File:".bold(), file_path.display()));
    output.push_str(&format!(
        "{} {} lines\n",
        "Size:".bold(),
        source.lines().count()
    ));
    output.push('\n');

    // Source code preview
    output.push_str(&format!("{}\n", "═══ Source Code ═══".yellow().bold()));
    for (i, line) in source.lines().enumerate() {
        output.push_str(&format!("{:3} │ {}\n", (i + 1).to_string().dimmed(), line));
    }
    output.push('\n');

    // AST tree
    output.push_str(&format!(
        "{}\n",
        "═══ Abstract Syntax Tree ═══".green().bold()
    ));
    format_ast_node(&ast, 0, &mut output);
    output.push('\n');

    // Statistics
    output.push_str(&format!("{}\n", "═══ Statistics ═══".blue().bold()));
    let node_count = count_nodes(&ast);
    output.push_str(&format!("  {} {}\n", "Total AST nodes:".bold(), node_count));
    output.push_str(&format!(
        "  {} {}\n",
        "Root node type:".bold(),
        ast.node_type
    ));
    if !ast.children.is_empty() {
        output.push_str(&format!(
            "  {} {}\n",
            "Direct children:".bold(),
            ast.children.len()
        ));
    }

    Ok(output)
}

/// Format an AST node with indentation
fn format_ast_node(node: &PythonAST, depth: usize, output: &mut String) {
    let indent = "  ".repeat(depth);
    let connector = if depth > 0 { "├─ " } else { "" };

    // Node type (colored)
    let node_type_colored = match node.node_type.as_str() {
        "Module" => node.node_type.cyan().bold(),
        "FunctionDef" => node.node_type.green().bold(),
        "ClassDef" => node.node_type.yellow().bold(),
        "Call" => node.node_type.magenta(),
        "Return" => node.node_type.red(),
        "Name" => node.node_type.blue(),
        _ => node.node_type.white(),
    };

    output.push_str(&format!("{}{}{}", indent, connector, node_type_colored));

    // Node attributes
    if !node.attributes.is_empty() {
        output.push_str(" (");
        let mut first = true;
        for (key, value) in &node.attributes {
            if !first {
                output.push_str(", ");
            }
            output.push_str(&format!("{}={}", key.dimmed(), value.bright_white()));
            first = false;
        }
        output.push(')');
    }

    // Source location
    if let Some(lineno) = node.lineno {
        output.push_str(&format!(" {}", format!("@L{lineno}").dimmed()));
    }

    output.push('\n');

    // Recursively format children
    for child in &node.children {
        format_ast_node(child, depth + 1, output);
    }
}

/// Count total nodes in AST
fn count_nodes(node: &PythonAST) -> usize {
    1 + node.children.iter().map(count_nodes).sum::<usize>()
}

/// Visualize C source as AST with `CPython` API annotations
///
/// # Errors
///
/// Returns an error if the file cannot be read or parsed
pub fn visualize_c(file_path: &Path) -> Result<String> {
    // Read the source file
    let source = fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read file: {}", file_path.display()))?;

    // Parse to AST
    let filename = file_path.to_string_lossy().to_string();
    let parser = spydecy_c::parser::CParser::new().context("Failed to create C parser")?;
    let ast = parser
        .parse(&source, &filename)
        .context("Failed to parse C source")?;

    // Format the output
    let mut output = String::new();

    // Header
    output.push_str(&format!(
        "{}",
        "╔══════════════════════════════════════════════════════════╗\n".cyan()
    ));
    output.push_str(&format!(
        "{}",
        "║  Spydecy Debugger: C AST Visualization                  ║\n".cyan()
    ));
    output.push_str(&format!(
        "{}",
        "╚══════════════════════════════════════════════════════════╝\n".cyan()
    ));
    output.push('\n');

    // File info
    output.push_str(&format!("{} {}\n", "File:".bold(), file_path.display()));
    output.push_str(&format!(
        "{} {} lines\n",
        "Size:".bold(),
        source.lines().count()
    ));
    output.push('\n');

    // Source code preview
    output.push_str(&format!("{}\n", "═══ Source Code ═══".yellow().bold()));
    for (i, line) in source.lines().enumerate() {
        output.push_str(&format!("{:3} │ {}\n", (i + 1).to_string().dimmed(), line));
    }
    output.push('\n');

    // AST tree with CPython annotations
    output.push_str(&format!(
        "{}\n",
        "═══ Abstract Syntax Tree ═══".green().bold()
    ));
    format_c_ast_node(&ast, 0, &mut output);
    output.push('\n');

    // CPython API analysis
    output.push_str(&format!(
        "{}\n",
        "═══ CPython API Analysis ═══".magenta().bold()
    ));
    let cpython_calls = collect_cpython_calls(&ast);
    if cpython_calls.is_empty() {
        output.push_str(&format!(
            "  {} No CPython API calls detected\n",
            "ℹ".dimmed()
        ));
    } else {
        for (pattern, name) in cpython_calls {
            output.push_str(&format!(
                "  {} {} → {:?}\n",
                "⚡".bright_yellow(),
                name.bright_white().bold(),
                pattern
            ));
        }
    }
    output.push('\n');

    // PyObject tracking
    output.push_str(&format!("{}\n", "═══ PyObject* Tracking ═══".blue().bold()));
    let pyobject_params = collect_pyobject_params(&ast);
    if pyobject_params.is_empty() {
        output.push_str(&format!(
            "  {} No PyObject* parameters detected\n",
            "ℹ".dimmed()
        ));
    } else {
        for (func_name, param_name, param_type) in pyobject_params {
            output.push_str(&format!(
                "  {} {}::{} ({})\n",
                "🐍".bright_cyan(),
                func_name.yellow(),
                param_name.bright_white(),
                param_type.dimmed()
            ));
        }
    }
    output.push('\n');

    // Statistics
    output.push_str(&format!("{}\n", "═══ Statistics ═══".blue().bold()));
    let node_count = count_c_nodes(&ast);
    output.push_str(&format!("  {} {}\n", "Total AST nodes:".bold(), node_count));
    output.push_str(&format!(
        "  {} {}\n",
        "Root node type:".bold(),
        ast.node_type
    ));
    if !ast.children.is_empty() {
        output.push_str(&format!(
            "  {} {}\n",
            "Direct children:".bold(),
            ast.children.len()
        ));
    }

    Ok(output)
}

/// Format a C AST node with indentation and `CPython` API highlighting
fn format_c_ast_node(node: &CAST, depth: usize, output: &mut String) {
    let indent = "  ".repeat(depth);
    let connector = if depth > 0 { "├─ " } else { "" };
    let pattern = cpython::identify_pattern(node);

    // Format node type with color
    let node_type_colored = colorize_c_node_type(&node.node_type, pattern.is_some());
    output.push_str(&format!("{indent}{connector}{node_type_colored}"));

    // Add node details
    format_c_node_details(node, pattern, output);
    output.push('\n');

    // Recursively format children
    for child in &node.children {
        format_c_ast_node(child, depth + 1, output);
    }
}

/// Colorize C node type based on type and `CPython` status
fn colorize_c_node_type(node_type: &str, is_cpython: bool) -> colored::ColoredString {
    use colored::Colorize;

    match node_type {
        "TranslationUnit" => node_type.cyan().bold(),
        "FunctionDecl" if is_cpython => node_type.magenta().bold(),
        "FunctionDecl" => node_type.green().bold(),
        "CallExpr" if is_cpython => node_type.magenta(),
        "CallExpr" => node_type.blue(),
        "ReturnStmt" => node_type.red(),
        "VarDecl" => node_type.yellow(),
        "ParmDecl" => node_type.cyan(),
        _ => node_type.white(),
    }
}

/// Format C node details (name, pattern, return type, parameters)
fn format_c_node_details(
    node: &CAST,
    pattern: Option<cpython::CPythonPattern>,
    output: &mut String,
) {
    // Node name
    if let Some(ref name) = node.name {
        output.push_str(&format!(" {}", name.bright_white().bold()));
    }

    // CPython pattern annotation
    if let Some(p) = pattern {
        output.push_str(&format!(" {} {p:?}", "⚡".bright_yellow()));
    }

    // Return type for functions
    if let Some(ref ret_type) = node.return_type {
        output.push_str(&format!(" → {}", ret_type.dimmed()));
    }

    // Parameters
    if !node.params.is_empty() {
        format_c_parameters(&node.params, output);
    }
}

/// Format C function parameters with `PyObject` highlighting
fn format_c_parameters(params: &[spydecy_c::parser::CParam], output: &mut String) {
    output.push_str(" (");
    for (i, param) in params.iter().enumerate() {
        if i > 0 {
            output.push_str(", ");
        }
        format_c_parameter(param, output);
    }
    output.push(')');
}

/// Format a single C parameter with appropriate highlighting
fn format_c_parameter(param: &spydecy_c::parser::CParam, output: &mut String) {
    let is_pyobject = param.param_type.contains("PyObject") || param.param_type.contains("PyList");
    if is_pyobject {
        output.push_str(&format!(
            "{}: {}",
            param.name.bright_cyan().bold(),
            param.param_type.cyan()
        ));
    } else {
        output.push_str(&format!("{}: {}", param.name, param.param_type.dimmed()));
    }
}

/// Collect `CPython` API calls from AST
fn collect_cpython_calls(node: &CAST) -> Vec<(cpython::CPythonPattern, String)> {
    let mut calls = Vec::new();

    if let Some(pattern) = cpython::identify_pattern(node) {
        if let Some(ref name) = node.name {
            calls.push((pattern, name.clone()));
        }
    }

    for child in &node.children {
        calls.extend(collect_cpython_calls(child));
    }

    calls
}

/// Collect `PyObject*` parameters from functions
fn collect_pyobject_params(node: &CAST) -> Vec<(String, String, String)> {
    let mut params = Vec::new();

    if node.node_type == "FunctionDecl" {
        if let Some(ref func_name) = node.name {
            for param in &node.params {
                if param.param_type.contains("PyObject")
                    || param.param_type.contains("PyList")
                    || param.param_type.contains("PyDict")
                {
                    params.push((
                        func_name.clone(),
                        param.name.clone(),
                        param.param_type.clone(),
                    ));
                }
            }
        }
    }

    for child in &node.children {
        params.extend(collect_pyobject_params(child));
    }

    params
}

/// Count total nodes in C AST
fn count_c_nodes(node: &CAST) -> usize {
    1 + node.children.iter().map(count_c_nodes).sum::<usize>()
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

        let result = visualize_python(temp_file.path());
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(output.contains("Module"));
        assert!(output.contains("FunctionDef"));
        assert!(output.contains("Return"));
        assert!(output.contains("Call"));
        assert!(output.contains("my_len"));
    }

    #[test]
    fn test_count_nodes() {
        let ast = PythonAST {
            node_type: "Module".to_string(),
            lineno: None,
            col_offset: None,
            children: vec![
                PythonAST::new("FunctionDef".to_string()),
                PythonAST::new("FunctionDef".to_string()),
            ],
            attributes: std::collections::HashMap::new(),
        };

        assert_eq!(count_nodes(&ast), 3); // Module + 2 FunctionDef
    }

    #[test]
    fn test_visualize_simple_c_function() {
        use std::io::Write;
        use tempfile::Builder;

        let mut temp_file = Builder::new().suffix(".c").tempfile().unwrap();
        writeln!(temp_file, "int add(int a, int b) {{\n    return a + b;\n}}").unwrap();
        temp_file.flush().unwrap(); // Ensure content is written

        let result = visualize_c(temp_file.path());
        assert!(
            result.is_ok(),
            "Should visualize C code: {:?}",
            result.as_ref().err()
        );

        let output = result.unwrap();
        assert!(output.contains("C AST Visualization"));
        assert!(output.contains("FunctionDecl"));
        assert!(output.contains("add"));
    }

    #[test]
    fn test_visualize_cpython_function() {
        use std::io::Write;
        use tempfile::Builder;

        let mut temp_file = Builder::new().suffix(".c").tempfile().unwrap();
        writeln!(
            temp_file,
            "static Py_ssize_t list_length(PyListObject *self) {{\n    return Py_SIZE(self);\n}}"
        )
        .unwrap();
        temp_file.flush().unwrap(); // Ensure content is written

        let result = visualize_c(temp_file.path());
        assert!(
            result.is_ok(),
            "Should visualize CPython code: {:?}",
            result.as_ref().err()
        );

        let output = result.unwrap();
        assert!(output.contains("CPython API Analysis"));
        assert!(output.contains("PyObject* Tracking"));
        assert!(output.contains("list_length"));
    }

    #[test]
    fn test_collect_cpython_calls() {
        let mut ast = CAST::new("FunctionDecl".to_owned());
        ast.name = Some("list_length".to_owned());

        let mut child = CAST::new("CallExpr".to_owned());
        child.name = Some("PyList_Append".to_owned());
        ast.children.push(child);

        let calls = collect_cpython_calls(&ast);
        assert_eq!(calls.len(), 2);
        assert!(calls.iter().any(|(_, name)| name == "list_length"));
        assert!(calls.iter().any(|(_, name)| name == "PyList_Append"));
    }

    #[test]
    fn test_collect_pyobject_params() {
        let mut ast = CAST::new("FunctionDecl".to_owned());
        ast.name = Some("test_func".to_owned());
        ast.params.push(spydecy_c::parser::CParam {
            name: "obj".to_owned(),
            param_type: "PyObject*".to_owned(),
        });
        ast.params.push(spydecy_c::parser::CParam {
            name: "x".to_owned(),
            param_type: "int".to_owned(),
        });

        let params = collect_pyobject_params(&ast);
        assert_eq!(params.len(), 1);
        assert_eq!(params[0].1, "obj");
        assert_eq!(params[0].2, "PyObject*");
    }

    #[test]
    fn test_count_c_nodes() {
        let mut ast = CAST::new("TranslationUnit".to_owned());
        ast.children.push(CAST::new("FunctionDecl".to_owned()));
        ast.children.push(CAST::new("FunctionDecl".to_owned()));

        assert_eq!(count_c_nodes(&ast), 3); // TranslationUnit + 2 FunctionDecl
    }
}
