//! End-to-end test for Phase 2.1: Full Argument Support
//!
//! This test verifies that variable names from Python flow through:
//! Python HIR → Unified HIR → Optimized HIR → Generated Rust Code

use spydecy_c::parse_c;
use spydecy_codegen::generate_rust;
use spydecy_hir::{
    c::CHIR,
    unified::{UnifiedHIR, Unifier},
};
use spydecy_optimizer::OptimizationPipeline;
use spydecy_python::parse_python;

#[test]
fn test_len_pattern_preserves_variable_name() {
    // Python code: len(my_list)
    let python_source = r#"
def test_len():
    return len(my_list)
"#;

    // C code: list_length()
    let c_source = r#"
#include <stddef.h>
static size_t list_length(void) {
    return 0;
}
"#;

    // Parse Python
    let python_hir = parse_python(python_source, "test.py").expect("Failed to parse Python");

    // Parse C
    let c_hir_module = parse_c(c_source, "test.c").expect("Failed to parse C");

    // Extract function from Python module
    let python_call = extract_python_call(python_hir);

    // Extract C function
    let c_func = extract_c_function(c_hir_module);

    // Unify
    let mut unifier = Unifier::new();
    let unified = unifier
        .unify(&python_call, &c_func)
        .expect("Unification should succeed");

    // Verify arguments are present in UnifiedHIR
    if let UnifiedHIR::Call { args, .. } = &unified {
        assert!(!args.is_empty(), "Args should be preserved in UnifiedHIR");
        assert_eq!(
            args.len(),
            1,
            "Should have exactly 1 argument (my_list variable)"
        );

        // Check that the argument is a Variable with the correct name
        if let UnifiedHIR::Variable { name, .. } = &args[0] {
            assert_eq!(name, "my_list", "Variable name should be 'my_list'");
        } else {
            panic!("First argument should be a Variable");
        }
    } else {
        panic!("Expected UnifiedHIR::Call");
    }

    // Optimize (boundary elimination)
    let pipeline = OptimizationPipeline::standard();
    let optimized = pipeline.run(unified).expect("Optimization should succeed");

    // Generate Rust code
    let rust_code = generate_rust(&optimized).expect("Codegen should succeed");

    // Verify the generated code uses the actual variable name
    assert!(
        rust_code.contains("my_list.len()"),
        "Generated code should use 'my_list.len()' not 'x.len()'. Got: {rust_code}"
    );
}

#[test]
fn test_append_pattern_preserves_variable_name() {
    // Python code: append(my_vector, item)
    let python_source = r#"
def test_append():
    return append(my_vector)
"#;

    // C code: PyList_Append()
    let c_source = r#"
static int PyList_Append(void) {
    return 0;
}
"#;

    // Parse
    let python_hir = parse_python(python_source, "test.py").expect("Failed to parse Python");
    let c_hir_module = parse_c(c_source, "test.c").expect("Failed to parse C");

    // Extract
    let python_call = extract_python_call(python_hir);
    let c_func = extract_c_function(c_hir_module);

    // Unify
    let mut unifier = Unifier::new();
    let unified = unifier
        .unify(&python_call, &c_func)
        .expect("Unification should succeed");

    // Verify arguments
    if let UnifiedHIR::Call { args, .. } = &unified {
        assert!(!args.is_empty(), "Args should be preserved");
        if let UnifiedHIR::Variable { name, .. } = &args[0] {
            assert_eq!(name, "my_vector");
        }
    }

    // Optimize & generate
    let pipeline = OptimizationPipeline::standard();
    let optimized = pipeline.run(unified).expect("Optimization should succeed");
    let rust_code = generate_rust(&optimized).expect("Codegen should succeed");

    // Verify the generated code uses the actual variable name
    assert!(
        rust_code.contains("my_vector.push(item)"),
        "Generated code should use 'my_vector.push(item)'. Got: {rust_code}"
    );
}

// Helper: Extract Python call from module
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

// Helper: Extract C function
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
