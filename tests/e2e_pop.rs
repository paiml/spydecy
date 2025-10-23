//! End-to-end test for list.pop() unification pattern
//!
//! Tests: Python list.pop() + C list_pop() → Rust Vec::pop()

use spydecy_c::parse_c;
use spydecy_codegen::generate_rust;
use spydecy_hir::python::PythonHIR;
use spydecy_hir::unified::Unifier;
use spydecy_optimizer::OptimizationPipeline;
use spydecy_python::parse_python;

#[test]
fn test_pop_unification_end_to_end() {
    // Python code using pop() function
    let python_source = r#"
def my_pop(lst):
    return pop(lst)
"#;

    // C implementation (ultra-simplified for current parser)
    let c_source = r#"
static PyObject* list_pop(PyListObject *self) {
    Py_SIZE(self);
    return 0;
}
"#;

    // Parse Python
    let python_hir = parse_python(python_source, "test_pop.py").expect("Failed to parse Python");

    // Parse C
    let c_hir_module = parse_c(c_source, "list_pop.c").expect("Failed to parse C");

    // Extract the callable parts
    let python_call = extract_python_call(python_hir);
    let c_function = extract_c_function(c_hir_module);

    // Unify
    let mut unifier = Unifier::new();
    let unified_hir = unifier
        .unify(&python_call, &c_function)
        .expect("Failed to unify");

    // Check that we got the PopPattern
    if let spydecy_hir::unified::UnifiedHIR::Call {
        cross_mapping: Some(mapping),
        ..
    } = &unified_hir
    {
        assert_eq!(
            mapping.pattern,
            spydecy_hir::unified::UnificationPattern::PopPattern,
            "Expected PopPattern"
        );
    }

    // Optimize
    let pipeline = OptimizationPipeline::standard();
    let optimized = pipeline.run(unified_hir).expect("Failed to optimize");

    // Generate Rust
    let rust_code = generate_rust(&optimized).expect("Failed to generate Rust code");

    // Verify the generated code contains Vec::pop or x.pop()
    assert!(
        rust_code.contains("pop") || rust_code.contains("Vec::pop"),
        "Generated code should contain pop call: {rust_code}"
    );

    println!("✅ Pop pattern test passed!");
    println!("Generated Rust code:\n{rust_code}");
}

// Helper: Extract Python call from module
fn extract_python_call(python_hir: PythonHIR) -> PythonHIR {
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
    panic!("Could not extract Python call");
}

// Helper: Extract C function from translation unit
fn extract_c_function(c_hir: spydecy_hir::c::CHIR) -> spydecy_hir::c::CHIR {
    use spydecy_hir::c::CHIR;

    if let CHIR::TranslationUnit { declarations, .. } = c_hir {
        return declarations.first().cloned().expect("No C declarations");
    }
    panic!("Expected C TranslationUnit");
}
