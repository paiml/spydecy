//! Python AST to HIR converter
//!
//! This module converts Python AST nodes into Spydecy's Python HIR.

use crate::parser::PythonAST;
use anyhow::{bail, Result};
use spydecy_hir::{
    metadata::Metadata,
    python::{Literal, PythonHIR},
    NodeId, Visibility,
};

/// Convert Python AST to HIR
///
/// # Errors
///
/// Returns an error if the AST cannot be converted to HIR
pub fn convert_to_hir(ast: &PythonAST) -> Result<PythonHIR> {
    let mut id_counter = 1;
    convert_node(ast, &mut id_counter)
}

fn convert_node(ast: &PythonAST, id_counter: &mut u64) -> Result<PythonHIR> {
    match ast.node_type.as_str() {
        "Module" => convert_module(ast, id_counter),
        "FunctionDef" => convert_function_def(ast, id_counter),
        "Return" => convert_return(ast, id_counter),
        "Call" => convert_call(ast, id_counter),
        "Name" => convert_name(ast, id_counter),
        "Constant" => convert_constant(id_counter),
        _ => bail!("Unsupported Python AST node type: {}", ast.node_type),
    }
}

/// Convert Module node
fn convert_module(ast: &PythonAST, id_counter: &mut u64) -> Result<PythonHIR> {
    let mut body = Vec::new();
    for child in &ast.children {
        body.push(convert_node(child, id_counter)?);
    }
    Ok(PythonHIR::Module {
        name: "main".to_string(),
        body,
        meta: Metadata::new(),
    })
}

/// Convert FunctionDef node
fn convert_function_def(ast: &PythonAST, id_counter: &mut u64) -> Result<PythonHIR> {
    let name = ast
        .attributes
        .get("name")
        .cloned()
        .unwrap_or_else(|| "unknown".to_string());

    let mut body = Vec::new();
    for child in &ast.children {
        body.push(convert_node(child, id_counter)?);
    }

    let id = next_id(id_counter);
    Ok(PythonHIR::Function {
        id,
        name,
        params: vec![],
        return_type: None,
        body,
        decorators: vec![],
        visibility: Visibility::Public,
        meta: Metadata::new(),
    })
}

/// Convert Return node
fn convert_return(ast: &PythonAST, id_counter: &mut u64) -> Result<PythonHIR> {
    let value = if ast.children.is_empty() {
        None
    } else {
        Some(Box::new(convert_node(&ast.children[0], id_counter)?))
    };

    let id = next_id(id_counter);
    Ok(PythonHIR::Return {
        id,
        value,
        meta: Metadata::new(),
    })
}

/// Convert Call node
fn convert_call(ast: &PythonAST, id_counter: &mut u64) -> Result<PythonHIR> {
    if ast.children.is_empty() {
        bail!("Call node must have at least one child (the callee)");
    }

    let callee = Box::new(convert_node(&ast.children[0], id_counter)?);

    let mut args = Vec::new();
    for child in &ast.children[1..] {
        args.push(convert_node(child, id_counter)?);
    }

    let id = next_id(id_counter);
    Ok(PythonHIR::Call {
        id,
        callee,
        args,
        kwargs: vec![],
        inferred_type: None,
        meta: Metadata::new(),
    })
}

/// Convert Name node
#[allow(clippy::unnecessary_wraps)]
fn convert_name(ast: &PythonAST, id_counter: &mut u64) -> Result<PythonHIR> {
    let name = ast
        .attributes
        .get("id")
        .cloned()
        .unwrap_or_else(|| "unknown".to_string());

    let id = next_id(id_counter);
    Ok(PythonHIR::Variable {
        id,
        name,
        inferred_type: None,
        meta: Metadata::new(),
    })
}

/// Convert Constant node
#[allow(clippy::unnecessary_wraps)]
fn convert_constant(id_counter: &mut u64) -> Result<PythonHIR> {
    let id = next_id(id_counter);
    Ok(PythonHIR::Literal {
        id,
        value: Literal::None, // Placeholder
        meta: Metadata::new(),
    })
}

fn next_id(counter: &mut u64) -> NodeId {
    let id = NodeId::new(*counter);
    *counter += 1;
    id
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_simple_function() {
        let mut ast = PythonAST::new("Module".to_string());
        let mut func = PythonAST::new("FunctionDef".to_string());
        func.attributes
            .insert("name".to_string(), "my_len".to_string());
        ast.children.push(func);

        let hir = convert_to_hir(&ast).unwrap();

        if let PythonHIR::Module { body, .. } = hir {
            assert_eq!(body.len(), 1);
        } else {
            panic!("Expected Module");
        }
    }

    #[test]
    fn test_convert_function_with_return() {
        let mut module = PythonAST::new("Module".to_string());
        let mut func = PythonAST::new("FunctionDef".to_string());
        func.attributes
            .insert("name".to_string(), "test".to_string());

        let ret = PythonAST::new("Return".to_string());
        func.children.push(ret);
        module.children.push(func);

        let hir = convert_to_hir(&module).unwrap();
        assert!(matches!(hir, PythonHIR::Module { .. }));
    }
}
