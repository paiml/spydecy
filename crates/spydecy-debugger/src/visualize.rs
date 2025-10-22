//! AST visualization for debugging
//!
//! This module provides formatted visualization of ASTs for debugging purposes.

use anyhow::{Context, Result};
use colored::Colorize;
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
    output.push_str(&format!("{}",  "╔══════════════════════════════════════════════════════════╗\n".cyan()));
    output.push_str(&format!("{}",  "║  Spydecy Debugger: Python AST Visualization             ║\n".cyan()));
    output.push_str(&format!("{}",  "╚══════════════════════════════════════════════════════════╝\n".cyan()));
    output.push('\n');

    // File info
    output.push_str(&format!("{} {}\n", "File:".bold(), file_path.display()));
    output.push_str(&format!("{} {} lines\n", "Size:".bold(), source.lines().count()));
    output.push('\n');

    // Source code preview
    output.push_str(&format!("{}\n", "═══ Source Code ═══".yellow().bold()));
    for (i, line) in source.lines().enumerate() {
        output.push_str(&format!("{:3} │ {}\n", (i + 1).to_string().dimmed(), line));
    }
    output.push('\n');

    // AST tree
    output.push_str(&format!("{}\n", "═══ Abstract Syntax Tree ═══".green().bold()));
    format_ast_node(&ast, 0, &mut output);
    output.push('\n');

    // Statistics
    output.push_str(&format!("{}\n", "═══ Statistics ═══".blue().bold()));
    let node_count = count_nodes(&ast);
    output.push_str(&format!("  {} {}\n", "Total AST nodes:".bold(), node_count));
    output.push_str(&format!("  {} {}\n", "Root node type:".bold(), ast.node_type));
    if !ast.children.is_empty() {
        output.push_str(&format!("  {} {}\n", "Direct children:".bold(), ast.children.len()));
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_visualize_simple_function() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(
            temp_file,
            "def my_len(x):\n    return len(x)"
        )
        .unwrap();

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
}
