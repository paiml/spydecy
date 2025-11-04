//! End-to-end test for contains() unification pattern
//!
//! Tests: Python 'x in list' + C list_contains() → Rust Vec::contains()

use spydecy_c::parse_c;
use spydecy_codegen::generate_rust;
use spydecy_hir::python::PythonHIR;
use spydecy_hir::unified::Unifier;
use spydecy_optimizer::OptimizationPipeline;
use spydecy_python::parse_python;

#[test]
fn test_list_contains_unification_end_to_end() {
    // Python code using 'in' operator for list containment
    let python_source = r#"
def check_contains(lst, item):
    return contains(lst, item)
"#;

    // C implementation (simplified list_contains pattern)
    let c_source = r#"
static int list_contains(PyListObject *a, PyObject *el) {
    Py_ssize_t i;
    int cmp;

    for (i = 0, cmp = 0 ; cmp == 0 && i < Py_SIZE(a); ++i)
        cmp = PyObject_RichCompareBool(el, PyList_GET_ITEM(a, i), Py_EQ);
    return cmp;
}
"#;

    // Parse Python
    let python_hir =
        parse_python(python_source, "test_contains.py").expect("Failed to parse Python");

    // Parse C
    let c_hir_module = parse_c(c_source, "list_contains.c").expect("Failed to parse C");

    // Extract the callable parts
    let python_call = extract_python_call(python_hir);
    let c_function = extract_c_function(c_hir_module);

    // Unify
    let mut unifier = Unifier::new();
    let unified_hir = unifier
        .unify(&python_call, &c_function)
        .expect("Failed to unify");

    // Check that we got the ListContainsPattern
    if let spydecy_hir::unified::UnifiedHIR::Call {
        cross_mapping: Some(mapping),
        ..
    } = &unified_hir
    {
        assert_eq!(
            mapping.pattern,
            spydecy_hir::unified::UnificationPattern::ListContainsPattern,
            "Expected ListContainsPattern"
        );
    }

    // Optimize
    let pipeline = OptimizationPipeline::standard();
    let optimized = pipeline.run(unified_hir).expect("Failed to optimize");

    // Generate Rust
    let rust_code = generate_rust(&optimized).expect("Failed to generate Rust code");

    // Verify the generated code contains Vec::contains or x.contains()
    assert!(
        rust_code.contains("contains"),
        "Generated code should contain contains call: {rust_code}"
    );

    println!("✅ List contains pattern test passed!");
    println!("Generated Rust code:\n{rust_code}");
}

#[test]
fn test_dict_contains_unification_end_to_end() {
    // Python code using 'in' operator for dict key containment
    let python_source = r#"
def check_has_key(dct, key):
    return dict_contains(dct, key)
"#;

    // C implementation (simplified PyDict_Contains pattern)
    let c_source = r#"
static int dict_contains(PyDictObject *mp, PyObject *key) {
    Py_hash_t hash;
    PyDictKeyEntry *ep;

    hash = PyObject_Hash(key);
    if (hash == -1)
        return -1;
    ep = (mp->ma_keys->dk_lookup)(mp, key, hash);
    return (ep->me_value != NULL);
}
"#;

    // Parse Python
    let python_hir =
        parse_python(python_source, "test_dict_contains.py").expect("Failed to parse Python");

    // Parse C
    let c_hir_module = parse_c(c_source, "dict_contains.c").expect("Failed to parse C");

    // Extract the callable parts
    let python_call = extract_python_call(python_hir);
    let c_function = extract_c_function(c_hir_module);

    // Unify
    let mut unifier = Unifier::new();
    let unified_hir = unifier
        .unify(&python_call, &c_function)
        .expect("Failed to unify");

    // Check that we got the DictContainsPattern
    if let spydecy_hir::unified::UnifiedHIR::Call {
        cross_mapping: Some(mapping),
        ..
    } = &unified_hir
    {
        assert_eq!(
            mapping.pattern,
            spydecy_hir::unified::UnificationPattern::DictContainsPattern,
            "Expected DictContainsPattern"
        );
    }

    // Optimize
    let pipeline = OptimizationPipeline::standard();
    let optimized = pipeline.run(unified_hir).expect("Failed to optimize");

    // Generate Rust
    let rust_code = generate_rust(&optimized).expect("Failed to generate Rust code");

    // Verify the generated code contains HashMap::contains_key or x.contains_key()
    assert!(
        rust_code.contains("contains_key"),
        "Generated code should contain contains_key call: {rust_code}"
    );

    println!("✅ Dict contains pattern test passed!");
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
