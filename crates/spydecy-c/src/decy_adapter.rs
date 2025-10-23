//! Adapter layer to convert decy-parser AST to spydecy CAST
//!
//! This module provides seamless integration between decy's comprehensive C parser
//! and spydecy's C HIR representation, enabling full C language support while
//! maintaining backward compatibility with existing spydecy code.

use crate::parser::{CParam, CAST};
use anyhow::Result;

/// Convert decy-parser `Ast` to spydecy `CAST`
///
/// This adapter enables spydecy to leverage decy's production-grade C parser
/// while maintaining its existing HIR interface.
///
/// # Errors
///
/// Returns an error if the conversion fails
pub fn convert_decy_ast_to_cast(decy_ast: &decy_parser::Ast) -> Result<CAST> {
    let mut root = CAST::new("TranslationUnit".to_owned());

    // Convert all functions
    for func in decy_ast.functions() {
        let func_node = convert_function(func);
        root.children.push(func_node);
    }

    // Convert all structs
    for struct_def in decy_ast.structs() {
        let struct_node = convert_struct(struct_def);
        root.children.push(struct_node);
    }

    // Convert all variables
    for var in decy_ast.variables() {
        let var_node = convert_variable(var);
        root.children.push(var_node);
    }

    // Convert all macros
    for macro_def in decy_ast.macros() {
        let macro_node = convert_macro(macro_def);
        root.children.push(macro_node);
    }

    Ok(root)
}

/// Convert decy `Function` to spydecy `CAST` function node
fn convert_function(func: &decy_parser::Function) -> CAST {
    let mut func_node = CAST::new("FunctionDecl".to_owned());
    func_node.name = Some(func.name.clone());
    func_node.return_type = Some(type_to_string(&func.return_type));

    // Convert parameters
    for param in &func.parameters {
        func_node.params.push(CParam {
            name: param.name.clone(),
            param_type: type_to_string(&param.param_type),
        });
    }

    // Convert body statements to children
    for stmt in &func.body {
        if let Some(stmt_node) = convert_statement(stmt) {
            func_node.children.push(stmt_node);
        }
    }

    func_node
}

/// Convert decy `Struct` to spydecy `CAST` struct node
fn convert_struct(struct_def: &decy_parser::Struct) -> CAST {
    let mut struct_node = CAST::new("StructDecl".to_owned());
    struct_node.name = Some(struct_def.name.clone());

    // Add field information as attributes
    for (i, field) in struct_def.fields.iter().enumerate() {
        struct_node.attributes.insert(
            format!("field_{i}"),
            format!("{}: {}", field.name, type_to_string(&field.field_type)),
        );
    }

    struct_node
}

/// Convert decy `Variable` to spydecy `CAST` variable node
fn convert_variable(var: &decy_parser::Variable) -> CAST {
    let mut var_node = CAST::new("VarDecl".to_owned());
    var_node.name = Some(var.name().to_owned());
    var_node.return_type = Some(type_to_string(var.var_type()));

    var_node
}

/// Convert decy `MacroDefinition` to spydecy `CAST` macro node
fn convert_macro(macro_def: &decy_parser::parser::MacroDefinition) -> CAST {
    let mut macro_node = CAST::new("macro definition".to_owned());
    macro_node.name = Some(macro_def.name().to_owned());
    macro_node
        .attributes
        .insert("body".to_owned(), macro_def.body().to_owned());

    if macro_def.is_function_like() {
        macro_node
            .attributes
            .insert("parameters".to_owned(), macro_def.parameters().join(", "));
    }

    macro_node
}

/// Convert decy `Statement` to spydecy `CAST` node
fn convert_statement(stmt: &decy_parser::Statement) -> Option<CAST> {
    use decy_parser::Statement;

    match stmt {
        Statement::Return(value) => {
            let mut ret_node = CAST::new("ReturnStmt".to_owned());
            if let Some(expr) = value {
                if let Some(expr_node) = convert_expression(expr) {
                    ret_node.children.push(expr_node);
                }
            }
            Some(ret_node)
        }
        Statement::FunctionCall {
            function,
            arguments,
        } => {
            let mut call_node = CAST::new("CallExpr".to_owned());
            call_node.name = Some(function.clone());
            // Convert arguments
            for arg in arguments {
                if let Some(arg_node) = convert_expression(arg) {
                    call_node.children.push(arg_node);
                }
            }
            Some(call_node)
        }
        Statement::VariableDeclaration { name, var_type, .. } => {
            let mut decl_node = CAST::new("DeclStmt".to_owned());
            decl_node.name = Some(name.clone());
            decl_node.return_type = Some(type_to_string(var_type));
            Some(decl_node)
        }
        Statement::Assignment { target, .. } => {
            let mut assign_node = CAST::new("BinaryOperator".to_owned());
            assign_node.name = Some(target.clone());
            assign_node
                .attributes
                .insert("opcode".to_owned(), "=".to_owned());
            Some(assign_node)
        }
        Statement::If { .. } => Some(CAST::new("IfStmt".to_owned())),
        Statement::While { .. } => Some(CAST::new("WhileStmt".to_owned())),
        Statement::For { .. } => Some(CAST::new("ForStmt".to_owned())),
        _ => None, // Other statement types not yet needed for CPython detection
    }
}

/// Convert decy `Expression` to spydecy `CAST` node
fn convert_expression(expr: &decy_parser::Expression) -> Option<CAST> {
    use decy_parser::Expression;

    match expr {
        Expression::FunctionCall {
            function,
            arguments,
        } => {
            let mut call_node = CAST::new("CallExpr".to_owned());
            call_node.name = Some(function.clone());
            // Convert arguments
            for arg in arguments {
                if let Some(arg_node) = convert_expression(arg) {
                    call_node.children.push(arg_node);
                }
            }
            Some(call_node)
        }
        Expression::Variable(name) => {
            let mut var_node = CAST::new("DeclRefExpr".to_owned());
            var_node.name = Some(name.clone());
            Some(var_node)
        }
        Expression::IntLiteral(val) => {
            let mut lit_node = CAST::new("IntegerLiteral".to_owned());
            lit_node
                .attributes
                .insert("value".to_owned(), val.to_string());
            Some(lit_node)
        }
        Expression::StringLiteral(val) => {
            let mut lit_node = CAST::new("StringLiteral".to_owned());
            lit_node.attributes.insert("value".to_owned(), val.clone());
            Some(lit_node)
        }
        Expression::BinaryOp { .. } => Some(CAST::new("BinaryOperator".to_owned())),
        _ => None, // Other expression types
    }
}

/// Convert decy `Type` to string representation for compatibility
fn type_to_string(ty: &decy_parser::Type) -> String {
    use decy_parser::Type;

    match ty {
        Type::Void => "void".to_owned(),
        Type::Int => "int".to_owned(),
        Type::Float => "float".to_owned(),
        Type::Double => "double".to_owned(),
        Type::Char => "char".to_owned(),
        Type::Pointer(inner) => format!("{}*", type_to_string(inner)),
        Type::Struct(name) => format!("struct {name}"),
        Type::Array { element_type, size } => {
            if let Some(s) = size {
                format!("{}[{s}]", type_to_string(element_type))
            } else {
                format!("{}[]", type_to_string(element_type))
            }
        }
        Type::FunctionPointer { .. } => "function_pointer".to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_simple_function() {
        // Create a simple decy function
        let func = decy_parser::Function::new(
            "add".to_owned(),
            decy_parser::Type::Int,
            vec![
                decy_parser::Parameter::new("a".to_owned(), decy_parser::Type::Int),
                decy_parser::Parameter::new("b".to_owned(), decy_parser::Type::Int),
            ],
        );

        let cast = convert_function(&func);

        assert_eq!(cast.node_type, "FunctionDecl");
        assert_eq!(cast.name, Some("add".to_owned()));
        assert_eq!(cast.return_type, Some("int".to_owned()));
        assert_eq!(cast.params.len(), 2);
        assert_eq!(cast.params[0].name, "a");
        assert_eq!(cast.params[1].name, "b");
    }

    #[test]
    fn test_type_to_string() {
        assert_eq!(type_to_string(&decy_parser::Type::Int), "int");
        assert_eq!(type_to_string(&decy_parser::Type::Void), "void");
        assert_eq!(
            type_to_string(&decy_parser::Type::Pointer(Box::new(
                decy_parser::Type::Char
            ))),
            "char*"
        );
        assert_eq!(
            type_to_string(&decy_parser::Type::Struct("Point".to_owned())),
            "struct Point"
        );
    }

    #[test]
    fn test_convert_macro() {
        let macro_def = decy_parser::parser::MacroDefinition::new_object_like(
            "MAX".to_owned(),
            "100".to_owned(),
        );

        let cast = convert_macro(&macro_def);

        assert_eq!(cast.node_type, "macro definition");
        assert_eq!(cast.name, Some("MAX".to_owned()));
        assert_eq!(cast.attributes.get("body"), Some(&"100".to_owned()));
    }
}
