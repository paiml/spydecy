//! End-to-end tests for common patterns
//!
//! Tests: dict.items(), list.remove(), list.sort()

use spydecy_c::parse_c;
use spydecy_codegen::generate_rust;
use spydecy_hir::python::PythonHIR;
use spydecy_hir::unified::Unifier;
use spydecy_optimizer::OptimizationPipeline;
use spydecy_python::parse_python;

#[test]
fn test_dict_items_unification_end_to_end() {
    // Python code using dict.items()
    let python_source = r#"
def iterate_items(my_dict):
    return dict_items(my_dict)
"#;

    // C implementation (simplified PyDict_Items)
    let c_source = r#"
static PyObject* PyDict_Items(PyDictObject *mp) {
    return NULL;
}
"#;

    // Parse Python
    let python_hir =
        parse_python(python_source, "test_dict_items.py").expect("Failed to parse Python");

    // Parse C
    let c_hir_module = parse_c(c_source, "dict_items.c").expect("Failed to parse C");

    // Extract the callable parts
    let python_call = extract_python_call(python_hir);
    let c_function = extract_c_function(c_hir_module);

    // Unify
    let mut unifier = Unifier::new();
    let unified_hir = unifier
        .unify(&python_call, &c_function)
        .expect("Failed to unify");

    // Check that we got the DictItemsPattern
    if let spydecy_hir::unified::UnifiedHIR::Call {
        cross_mapping: Some(mapping),
        ..
    } = &unified_hir
    {
        assert_eq!(
            mapping.pattern,
            spydecy_hir::unified::UnificationPattern::DictItemsPattern,
            "Expected DictItemsPattern"
        );
    }

    // Optimize
    let pipeline = OptimizationPipeline::standard();
    let optimized = pipeline.run(unified_hir).expect("Failed to optimize");

    // Generate Rust
    let rust_code = generate_rust(&optimized).expect("Failed to generate Rust code");

    // Verify the generated code contains HashMap::iter or x.iter()
    assert!(
        rust_code.contains("iter"),
        "Generated code should contain iter call: {rust_code}"
    );

    println!("✅ Dict items pattern test passed!");
    println!("Generated Rust code:\n{rust_code}");
}

#[test]
fn test_list_remove_unification_end_to_end() {
    // Python code using list.remove()
    let python_source = r#"
def remove_item(my_list, item):
    return remove(my_list, item)
"#;

    // C implementation (simplified list_remove)
    let c_source = r#"
static int list_remove(PyListObject *self, PyObject *value) {
    Py_ssize_t i;

    for (i = 0; i < Py_SIZE(self); i++) {
        int cmp = PyObject_RichCompareBool(PyList_GET_ITEM(self, i), value, Py_EQ);
        if (cmp > 0) {
            if (list_ass_slice(self, i, i+1, (PyObject *)NULL) == 0)
                return 0;
            return -1;
        }
        else if (cmp < 0)
            return -1;
    }
    return -1;
}
"#;

    // Parse Python
    let python_hir = parse_python(python_source, "test_remove.py").expect("Failed to parse Python");

    // Parse C
    let c_hir_module = parse_c(c_source, "list_remove.c").expect("Failed to parse C");

    // Extract the callable parts
    let python_call = extract_python_call(python_hir);
    let c_function = extract_c_function(c_hir_module);

    // Unify
    let mut unifier = Unifier::new();
    let unified_hir = unifier
        .unify(&python_call, &c_function)
        .expect("Failed to unify");

    // Check that we got the ListRemovePattern
    if let spydecy_hir::unified::UnifiedHIR::Call {
        cross_mapping: Some(mapping),
        ..
    } = &unified_hir
    {
        assert_eq!(
            mapping.pattern,
            spydecy_hir::unified::UnificationPattern::ListRemovePattern,
            "Expected ListRemovePattern"
        );
    }

    // Optimize
    let pipeline = OptimizationPipeline::standard();
    let optimized = pipeline.run(unified_hir).expect("Failed to optimize");

    // Generate Rust
    let rust_code = generate_rust(&optimized).expect("Failed to generate Rust code");

    // Verify the generated code contains retain (Rust idiom for removing items)
    assert!(
        rust_code.contains("retain"),
        "Generated code should contain retain call: {rust_code}"
    );

    println!("✅ List remove pattern test passed!");
    println!("Generated Rust code:\n{rust_code}");
}

#[test]
fn test_list_sort_unification_end_to_end() {
    // Python code using list.sort()
    let python_source = r#"
def sort_list(my_list):
    return sort(my_list)
"#;

    // C implementation (simplified list_sort)
    let c_source = r#"
static int list_sort(PyListObject *self) {
    return 0;
}
"#;

    // Parse Python
    let python_hir = parse_python(python_source, "test_sort.py").expect("Failed to parse Python");

    // Parse C
    let c_hir_module = parse_c(c_source, "list_sort.c").expect("Failed to parse C");

    // Extract the callable parts
    let python_call = extract_python_call(python_hir);
    let c_function = extract_c_function(c_hir_module);

    // Unify
    let mut unifier = Unifier::new();
    let unified_hir = unifier
        .unify(&python_call, &c_function)
        .expect("Failed to unify");

    // Check that we got the ListSortPattern
    if let spydecy_hir::unified::UnifiedHIR::Call {
        cross_mapping: Some(mapping),
        ..
    } = &unified_hir
    {
        assert_eq!(
            mapping.pattern,
            spydecy_hir::unified::UnificationPattern::ListSortPattern,
            "Expected ListSortPattern"
        );
    }

    // Optimize
    let pipeline = OptimizationPipeline::standard();
    let optimized = pipeline.run(unified_hir).expect("Failed to optimize");

    // Generate Rust
    let rust_code = generate_rust(&optimized).expect("Failed to generate Rust code");

    // Verify the generated code contains sort
    assert!(
        rust_code.contains("sort"),
        "Generated code should contain sort call: {rust_code}"
    );

    println!("✅ List sort pattern test passed!");
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
