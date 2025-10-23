//! End-to-end test for Phase 2.3: Error Messages
//!
//! This test verifies that unification failures produce helpful, actionable error messages.

use spydecy_c::parse_c;
use spydecy_hir::{c::CHIR, unified::Unifier};
use spydecy_python::parse_python;

#[test]
fn test_no_pattern_match_error_message() {
    // Python code with unknown function
    let python_source = r#"
def test_unknown():
    return unknown_function(x)
"#;

    // C code with unknown function
    let c_source = r#"
static int unknown_c_function(void) {
    return 0;
}
"#;

    // Parse
    let python_hir = parse_python(python_source, "test.py").expect("Failed to parse Python");
    let c_hir_module = parse_c(c_source, "test.c").expect("Failed to parse C");

    // Extract
    let python_call = extract_python_call(python_hir);
    let c_func = extract_c_function(c_hir_module);

    // Unify - should fail with helpful error
    let mut unifier = Unifier::new();
    let result = unifier.unify(&python_call, &c_func);

    assert!(result.is_err(), "Should fail to unify unknown functions");

    let error_msg = result.unwrap_err().to_string();

    // Verify error message contains:
    assert!(
        error_msg.contains("Cannot match Python function"),
        "Error should identify it's a pattern matching failure"
    );
    assert!(
        error_msg.contains("unknown_function"),
        "Error should show Python function name"
    );
    assert!(
        error_msg.contains("unknown_c_function"),
        "Error should show C function name"
    );
    assert!(
        error_msg.contains("Supported patterns"),
        "Error should list supported patterns"
    );
    assert!(
        error_msg.contains("len()"),
        "Error should mention len() pattern as example"
    );
    assert!(
        error_msg.contains("Vec::len()"),
        "Error should show Rust output for patterns"
    );
    assert!(
        error_msg.contains("custom-patterns") || error_msg.contains("github.com/noahgift/spydecy"),
        "Error should link to documentation"
    );
}

#[test]
fn test_similar_pattern_suggestions() {
    // Python code: something similar to "append" but not exact
    let python_source = r#"
def test_add():
    return add(x)
"#;

    // C code: something similar to PyList
    let c_source = r#"
static int PyList_Add(void) {
    return 0;
}
"#;

    // Parse
    let python_hir = parse_python(python_source, "test.py").expect("Failed to parse Python");
    let c_hir_module = parse_c(c_source, "test.c").expect("Failed to parse C");

    // Extract
    let python_call = extract_python_call(python_hir);
    let c_func = extract_c_function(c_hir_module);

    // Unify - should fail with suggestions
    let mut unifier = Unifier::new();
    let result = unifier.unify(&python_call, &c_func);

    assert!(result.is_err());

    let error_msg = result.unwrap_err().to_string();

    // Should suggest similar patterns (likely append/PyList_Append)
    assert!(
        error_msg.contains("Supported patterns"),
        "Should show pattern suggestions"
    );
}

#[test]
fn test_incompatible_nodes_error() {
    // Try to unify Python literal with C function (incompatible types)
    let python_source = r#"
def test_literal():
    return 42
"#;

    let c_source = r#"
static int some_function(void) {
    return 0;
}
"#;

    // Parse
    let python_hir = parse_python(python_source, "test.py").expect("Failed to parse Python");
    let c_hir_module = parse_c(c_source, "test.c").expect("Failed to parse C");

    // Extract literal from return statement
    let python_literal = if let spydecy_hir::python::PythonHIR::Module { body, .. } = python_hir {
        if let Some(spydecy_hir::python::PythonHIR::Function {
            body: func_body, ..
        }) = body.first()
        {
            if let Some(spydecy_hir::python::PythonHIR::Return {
                value: Some(literal),
                ..
            }) = func_body.first()
            {
                literal.as_ref().clone()
            } else {
                panic!("Expected return with value");
            }
        } else {
            panic!("Expected function");
        }
    } else {
        panic!("Expected module");
    };

    let c_func = extract_c_function(c_hir_module);

    // Try to unify literal with function (incompatible)
    let mut unifier = Unifier::new();
    let result = unifier.unify(&python_literal, &c_func);

    assert!(
        result.is_err(),
        "Should fail to unify literal with function"
    );

    let error_msg = result.unwrap_err().to_string();

    // Should mention incompatibility
    assert!(
        error_msg.contains("incompatible") || error_msg.contains("Cannot"),
        "Error should mention incompatibility: {}",
        error_msg
    );
}

// Helper functions

fn extract_python_call(
    python_hir: spydecy_hir::python::PythonHIR,
) -> spydecy_hir::python::PythonHIR {
    use spydecy_hir::python::PythonHIR;

    if let PythonHIR::Module { body, .. } = python_hir {
        if let Some(PythonHIR::Function {
            body: func_body, ..
        }) = body.first()
        {
            if let Some(PythonHIR::Return {
                value: Some(call), ..
            }) = func_body.first()
            {
                return call.as_ref().clone();
            }
        }
    }
    panic!("Expected Python module with function containing return statement");
}

fn extract_c_function(c_hir_module: CHIR) -> CHIR {
    if let CHIR::TranslationUnit { declarations, .. } = c_hir_module {
        declarations
            .first()
            .expect("C file has no declarations")
            .clone()
    } else {
        panic!("Expected C TranslationUnit");
    }
}
