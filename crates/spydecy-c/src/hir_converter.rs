//! C AST to HIR converter
//!
//! Converts parsed C AST into Spydecy's C HIR representation.

use crate::parser::CAST;
use anyhow::{bail, Result};
use spydecy_hir::{
    c::{Parameter, StorageClass, CHIR},
    metadata::Metadata,
    types::{CType, Type},
    NodeId, Visibility,
};

/// Convert C AST to HIR
///
/// # Errors
///
/// Returns an error if the AST cannot be converted
pub fn convert_to_hir(ast: &CAST) -> Result<CHIR> {
    let mut id_counter = 1;
    convert_node(ast, &mut id_counter)
}

fn convert_node(ast: &CAST, id_counter: &mut u64) -> Result<CHIR> {
    match ast.node_type.as_str() {
        "TranslationUnit" => convert_translation_unit(ast, id_counter),
        "FunctionDecl" => convert_function_decl(ast, id_counter),
        "ReturnStmt" => convert_return_stmt(ast, id_counter),
        "CallExpr" => convert_call_expr(ast, id_counter),
        "DeclRefExpr" => convert_decl_ref_expr(ast, id_counter),
        _ => bail!("Unsupported C AST node type: {}", ast.node_type),
    }
}

/// Convert TranslationUnit node
fn convert_translation_unit(ast: &CAST, id_counter: &mut u64) -> Result<CHIR> {
    let mut declarations = Vec::new();
    for child in &ast.children {
        // Only process function declarations for now
        if child.node_type == "FunctionDecl" {
            declarations.push(convert_node(child, id_counter)?);
        }
    }

    Ok(CHIR::TranslationUnit {
        name: "main".to_string(),
        declarations,
        meta: Metadata::new(),
    })
}

/// Convert FunctionDecl node
#[allow(clippy::unnecessary_wraps)]
fn convert_function_decl(ast: &CAST, id_counter: &mut u64) -> Result<CHIR> {
    let name = ast.name.clone().unwrap_or_else(|| "unknown".to_string());
    let return_type = parse_type(&ast.return_type);

    let params = ast
        .params
        .iter()
        .map(|p| Parameter {
            name: p.name.clone(),
            param_type: parse_type(&Some(p.param_type.clone())),
        })
        .collect();

    // Convert function body (simplified for now)
    let mut body = Vec::new();
    for child in &ast.children {
        if child.node_type.contains("Stmt") || child.node_type == "ReturnStmt" {
            if let Ok(stmt) = convert_node(child, id_counter) {
                body.push(stmt);
            }
        }
    }

    let id = next_id(id_counter);
    Ok(CHIR::Function {
        id,
        name,
        return_type,
        params,
        body,
        storage_class: StorageClass::Static,
        visibility: Visibility::Private,
        meta: Metadata::new(),
    })
}

/// Convert ReturnStmt node
fn convert_return_stmt(ast: &CAST, id_counter: &mut u64) -> Result<CHIR> {
    let value = if ast.children.is_empty() {
        None
    } else {
        Some(Box::new(convert_node(&ast.children[0], id_counter)?))
    };

    let id = next_id(id_counter);
    Ok(CHIR::Return {
        id,
        value,
        meta: Metadata::new(),
    })
}

/// Convert CallExpr node
fn convert_call_expr(ast: &CAST, id_counter: &mut u64) -> Result<CHIR> {
    if ast.children.is_empty() {
        bail!("CallExpr must have at least one child (callee)");
    }

    let callee = Box::new(convert_node(&ast.children[0], id_counter)?);

    let args = ast.children[1..]
        .iter()
        .filter_map(|child| convert_node(child, id_counter).ok())
        .collect();

    let id = next_id(id_counter);
    Ok(CHIR::Call {
        id,
        callee,
        args,
        inferred_type: None,
        meta: Metadata::new(),
    })
}

/// Convert DeclRefExpr node
#[allow(clippy::unnecessary_wraps)]
fn convert_decl_ref_expr(ast: &CAST, id_counter: &mut u64) -> Result<CHIR> {
    let name = ast.name.clone().unwrap_or_else(|| "unknown".to_string());
    let id = next_id(id_counter);

    // Check if this is a CPython macro like Py_SIZE
    if name.starts_with("Py_") || name.starts_with("_Py") {
        Ok(CHIR::CPythonMacro {
            id,
            name,
            args: vec![],
            inferred_type: None,
            meta: Metadata::new(),
        })
    } else {
        Ok(CHIR::Variable {
            id,
            name,
            var_type: None,
            meta: Metadata::new(),
        })
    }
}

fn parse_type(type_str: &Option<String>) -> Type {
    match type_str.as_deref() {
        Some("int") => Type::C(CType::Int),
        Some("void") => Type::C(CType::Void),
        Some("size_t") => Type::C(CType::SizeT),
        Some("Py_ssize_t") => Type::C(CType::CPython(spydecy_hir::types::CPythonType::PySsizeT)),
        Some(s) if s.contains("PyListObject") => Type::C(CType::CPython(
            spydecy_hir::types::CPythonType::PyListObject,
        )),
        Some(s) if s.contains("PyObject") => {
            Type::C(CType::CPython(spydecy_hir::types::CPythonType::PyObject))
        }
        _ => Type::Unknown,
    }
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
    fn test_convert_empty_translation_unit() {
        let ast = CAST::new("TranslationUnit".to_string());
        let result = convert_to_hir(&ast);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_basic_types() {
        assert!(matches!(
            parse_type(&Some("int".to_string())),
            Type::C(CType::Int)
        ));
        assert!(matches!(
            parse_type(&Some("void".to_string())),
            Type::C(CType::Void)
        ));
        assert!(matches!(
            parse_type(&Some("size_t".to_string())),
            Type::C(CType::SizeT)
        ));
    }

    #[test]
    fn test_parse_cpython_types() {
        let py_ssize = parse_type(&Some("Py_ssize_t".to_string()));
        assert!(matches!(py_ssize, Type::C(CType::CPython(_))));

        let pylist = parse_type(&Some("PyListObject*".to_string()));
        assert!(matches!(pylist, Type::C(CType::CPython(_))));
    }
}
