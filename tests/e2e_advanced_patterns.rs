//! End-to-end tests for advanced patterns
//!
//! Tests: dict.values(), list.count(), list.index()

use spydecy_c::parse_c;
use spydecy_codegen::generate_rust;
use spydecy_hir::python::PythonHIR;
use spydecy_hir::unified::Unifier;
use spydecy_optimizer::OptimizationPipeline;
use spydecy_python::parse_python;

#[test]
fn test_dict_values_unification_end_to_end() {
    // Python code using dict.values()
    let python_source = r#"
def get_all_values(my_dict):
    return dict_values(my_dict)
"#;

    // C implementation (simplified PyDict_Values)
    let c_source = r#"
static PyObject* PyDict_Values(PyDictObject *mp) {
    return NULL;
}
"#;

    // Parse Python
    let python_hir =
        parse_python(python_source, "test_dict_values.py").expect("Failed to parse Python");

    // Parse C
    let c_hir_module = parse_c(c_source, "dict_values.c").expect("Failed to parse C");

    // Extract the callable parts
    let python_call = extract_python_call(python_hir);
    let c_function = extract_c_function(c_hir_module);

    // Unify
    let mut unifier = Unifier::new();
    let unified_hir = unifier
        .unify(&python_call, &c_function)
        .expect("Failed to unify");

    // Check that we got the DictValuesPattern
    if let spydecy_hir::unified::UnifiedHIR::Call {
        cross_mapping: Some(mapping),
        ..
    } = &unified_hir
    {
        assert_eq!(
            mapping.pattern,
            spydecy_hir::unified::UnificationPattern::DictValuesPattern,
            "Expected DictValuesPattern"
        );
    }

    // Optimize
    let pipeline = OptimizationPipeline::standard();
    let optimized = pipeline.run(unified_hir).expect("Failed to optimize");

    // Generate Rust
    let rust_code = generate_rust(&optimized).expect("Failed to generate Rust code");

    // Verify the generated code contains HashMap::values or x.values()
    assert!(
        rust_code.contains("values"),
        "Generated code should contain values call: {rust_code}"
    );

    println!("✅ Dict values pattern test passed!");
    println!("Generated Rust code:\n{rust_code}");
}

#[test]
fn test_list_count_unification_end_to_end() {
    // Python code using list.count()
    let python_source = r#"
def count_occurrences(my_list, item):
    return count(my_list, item)
"#;

    // C implementation (simplified list_count)
    let c_source = r#"
static Py_ssize_t list_count(PyListObject *self, PyObject *value) {
    Py_ssize_t count = 0;
    Py_ssize_t i;

    for (i = 0; i < Py_SIZE(self); i++) {
        int cmp = PyObject_RichCompareBool(PyList_GET_ITEM(self, i), value, Py_EQ);
        if (cmp > 0)
            count++;
        else if (cmp < 0)
            return -1;
    }
    return count;
}
"#;

    // Parse Python
    let python_hir = parse_python(python_source, "test_count.py").expect("Failed to parse Python");

    // Parse C
    let c_hir_module = parse_c(c_source, "list_count.c").expect("Failed to parse C");

    // Extract the callable parts
    let python_call = extract_python_call(python_hir);
    let c_function = extract_c_function(c_hir_module);

    // Unify
    let mut unifier = Unifier::new();
    let unified_hir = unifier
        .unify(&python_call, &c_function)
        .expect("Failed to unify");

    // Check that we got the ListCountPattern
    if let spydecy_hir::unified::UnifiedHIR::Call {
        cross_mapping: Some(mapping),
        ..
    } = &unified_hir
    {
        assert_eq!(
            mapping.pattern,
            spydecy_hir::unified::UnificationPattern::ListCountPattern,
            "Expected ListCountPattern"
        );
    }

    // Optimize
    let pipeline = OptimizationPipeline::standard();
    let optimized = pipeline.run(unified_hir).expect("Failed to optimize");

    // Generate Rust
    let rust_code = generate_rust(&optimized).expect("Failed to generate Rust code");

    // Verify the generated code contains count
    assert!(
        rust_code.contains("count"),
        "Generated code should contain count call: {rust_code}"
    );

    println!("✅ List count pattern test passed!");
    println!("Generated Rust code:\n{rust_code}");
}

#[test]
fn test_list_index_unification_end_to_end() {
    // Python code using list.index()
    let python_source = r#"
def find_index(my_list, item):
    return index(my_list, item)
"#;

    // C implementation (simplified list_index)
    let c_source = r#"
static Py_ssize_t list_index(PyListObject *self, PyObject *value) {
    Py_ssize_t i;

    for (i = 0; i < Py_SIZE(self); i++) {
        int cmp = PyObject_RichCompareBool(PyList_GET_ITEM(self, i), value, Py_EQ);
        if (cmp > 0)
            return i;
        else if (cmp < 0)
            return -1;
    }
    return -1;
}
"#;

    // Parse Python
    let python_hir = parse_python(python_source, "test_index.py").expect("Failed to parse Python");

    // Parse C
    let c_hir_module = parse_c(c_source, "list_index.c").expect("Failed to parse C");

    // Extract the callable parts
    let python_call = extract_python_call(python_hir);
    let c_function = extract_c_function(c_hir_module);

    // Unify
    let mut unifier = Unifier::new();
    let unified_hir = unifier
        .unify(&python_call, &c_function)
        .expect("Failed to unify");

    // Check that we got the ListIndexPattern
    if let spydecy_hir::unified::UnifiedHIR::Call {
        cross_mapping: Some(mapping),
        ..
    } = &unified_hir
    {
        assert_eq!(
            mapping.pattern,
            spydecy_hir::unified::UnificationPattern::ListIndexPattern,
            "Expected ListIndexPattern"
        );
    }

    // Optimize
    let pipeline = OptimizationPipeline::standard();
    let optimized = pipeline.run(unified_hir).expect("Failed to optimize");

    // Generate Rust
    let rust_code = generate_rust(&optimized).expect("Failed to generate Rust code");

    // Verify the generated code contains position
    assert!(
        rust_code.contains("position"),
        "Generated code should contain position call: {rust_code}"
    );

    println!("✅ List index pattern test passed!");
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
