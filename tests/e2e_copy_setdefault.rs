//! End-to-end tests for copy and setdefault patterns
//!
//! Tests: list.copy(), dict.copy(), dict.setdefault()

use spydecy_c::parse_c;
use spydecy_codegen::generate_rust;
use spydecy_hir::python::PythonHIR;
use spydecy_hir::unified::Unifier;
use spydecy_optimizer::OptimizationPipeline;
use spydecy_python::parse_python;

#[test]
fn test_list_copy_unification_end_to_end() {
    // Python code using list.copy()
    let python_source = r#"
def copy_list(my_list):
    return copy(my_list)
"#;

    // C implementation (simplified PyList_Copy)
    let c_source = r#"
static PyObject* list_copy(PyListObject *self) {
    PyListObject *np;
    PyObject **src, **dest;
    Py_ssize_t i, n;

    n = Py_SIZE(self);
    np = (PyListObject *)PyList_New(n);
    if (np == NULL)
        return NULL;

    src = self->ob_item;
    dest = np->ob_item;
    for (i = 0; i < n; i++) {
        PyObject *v = src[i];
        Py_INCREF(v);
        dest[i] = v;
    }
    Py_SET_SIZE(np, n);
    return (PyObject *)np;
}
"#;

    // Parse Python
    let python_hir = parse_python(python_source, "test_copy.py").expect("Failed to parse Python");

    // Parse C
    let c_hir_module = parse_c(c_source, "list_copy.c").expect("Failed to parse C");

    // Extract the callable parts
    let python_call = extract_python_call(python_hir);
    let c_function = extract_c_function(c_hir_module);

    // Unify
    let mut unifier = Unifier::new();
    let unified_hir = unifier
        .unify(&python_call, &c_function)
        .expect("Failed to unify");

    // Check that we got the ListCopyPattern
    if let spydecy_hir::unified::UnifiedHIR::Call {
        cross_mapping: Some(mapping),
        ..
    } = &unified_hir
    {
        assert_eq!(
            mapping.pattern,
            spydecy_hir::unified::UnificationPattern::ListCopyPattern,
            "Expected ListCopyPattern"
        );
    }

    // Optimize
    let pipeline = OptimizationPipeline::standard();
    let optimized = pipeline.run(unified_hir).expect("Failed to optimize");

    // Generate Rust
    let rust_code = generate_rust(&optimized).expect("Failed to generate Rust code");

    // Verify the generated code contains clone
    assert!(
        rust_code.contains("clone"),
        "Generated code should contain clone call: {rust_code}"
    );

    println!("✅ List copy pattern test passed!");
    println!("Generated Rust code:\n{rust_code}");
}

#[test]
fn test_dict_copy_unification_end_to_end() {
    // Python code using dict.copy()
    let python_source = r#"
def copy_dict(my_dict):
    return dict_copy(my_dict)
"#;

    // C implementation (simplified PyDict_Copy)
    let c_source = r#"
static PyObject* dict_copy(PyDictObject *mp) {
    PyObject *copy;
    Py_ssize_t i, n;

    copy = PyDict_New();
    if (copy == NULL)
        return NULL;

    if (PyDict_Merge(copy, (PyObject *)mp, 1) == 0)
        return copy;

    Py_DECREF(copy);
    return NULL;
}
"#;

    // Parse Python
    let python_hir =
        parse_python(python_source, "test_dict_copy.py").expect("Failed to parse Python");

    // Parse C
    let c_hir_module = parse_c(c_source, "dict_copy.c").expect("Failed to parse C");

    // Extract the callable parts
    let python_call = extract_python_call(python_hir);
    let c_function = extract_c_function(c_hir_module);

    // Unify
    let mut unifier = Unifier::new();
    let unified_hir = unifier
        .unify(&python_call, &c_function)
        .expect("Failed to unify");

    // Check that we got the DictCopyPattern
    if let spydecy_hir::unified::UnifiedHIR::Call {
        cross_mapping: Some(mapping),
        ..
    } = &unified_hir
    {
        assert_eq!(
            mapping.pattern,
            spydecy_hir::unified::UnificationPattern::DictCopyPattern,
            "Expected DictCopyPattern"
        );
    }

    // Optimize
    let pipeline = OptimizationPipeline::standard();
    let optimized = pipeline.run(unified_hir).expect("Failed to optimize");

    // Generate Rust
    let rust_code = generate_rust(&optimized).expect("Failed to generate Rust code");

    // Verify the generated code contains clone
    assert!(
        rust_code.contains("clone"),
        "Generated code should contain clone call: {rust_code}"
    );

    println!("✅ Dict copy pattern test passed!");
    println!("Generated Rust code:\n{rust_code}");
}

#[test]
fn test_dict_setdefault_unification_end_to_end() {
    // Python code using dict.setdefault()
    let python_source = r#"
def set_default_value(my_dict, key, default):
    return setdefault(my_dict, key, default)
"#;

    // C implementation (simplified PyDict_SetDefault)
    let c_source = r#"
static PyObject* dict_setdefault(PyDictObject *mp, PyObject *key, PyObject *default_value) {
    PyObject *value;
    Py_hash_t hash;

    if (mp->ma_keys == Py_EMPTY_KEYS) {
        hash = PyObject_Hash(key);
        if (hash == -1)
            return NULL;
    }

    value = PyDict_GetItem((PyObject *)mp, key);
    if (value != NULL) {
        Py_INCREF(value);
        return value;
    }

    if (PyDict_SetItem((PyObject *)mp, key, default_value) < 0)
        return NULL;

    Py_INCREF(default_value);
    return default_value;
}
"#;

    // Parse Python
    let python_hir =
        parse_python(python_source, "test_setdefault.py").expect("Failed to parse Python");

    // Parse C
    let c_hir_module = parse_c(c_source, "dict_setdefault.c").expect("Failed to parse C");

    // Extract the callable parts
    let python_call = extract_python_call(python_hir);
    let c_function = extract_c_function(c_hir_module);

    // Unify
    let mut unifier = Unifier::new();
    let unified_hir = unifier
        .unify(&python_call, &c_function)
        .expect("Failed to unify");

    // Check that we got the DictSetDefaultPattern
    if let spydecy_hir::unified::UnifiedHIR::Call {
        cross_mapping: Some(mapping),
        ..
    } = &unified_hir
    {
        assert_eq!(
            mapping.pattern,
            spydecy_hir::unified::UnificationPattern::DictSetDefaultPattern,
            "Expected DictSetDefaultPattern"
        );
    }

    // Optimize
    let pipeline = OptimizationPipeline::standard();
    let optimized = pipeline.run(unified_hir).expect("Failed to optimize");

    // Generate Rust
    let rust_code = generate_rust(&optimized).expect("Failed to generate Rust code");

    // Verify the generated code contains entry().or_insert()
    assert!(
        rust_code.contains("entry") && rust_code.contains("or_insert"),
        "Generated code should contain entry().or_insert() pattern: {rust_code}"
    );

    println!("✅ Dict setdefault pattern test passed!");
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
