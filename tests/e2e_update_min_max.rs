//! End-to-end tests for update, min, and max patterns
//!
//! Tests: dict.update(), list.min(), list.max()

use spydecy_c::parse_c;
use spydecy_codegen::generate_rust;
use spydecy_hir::python::PythonHIR;
use spydecy_hir::unified::Unifier;
use spydecy_optimizer::OptimizationPipeline;
use spydecy_python::parse_python;

#[test]
fn test_dict_update_unification_end_to_end() {
    // Python code using dict.update()
    let python_source = r#"
def merge_dicts(target, source):
    return update(target, source)
"#;

    // C implementation (simplified PyDict_Update)
    let c_source = r#"
static int dict_update(PyDictObject *mp, PyObject *other) {
    if (PyDict_CheckExact(other)) {
        PyDictObject *odict = (PyDictObject *)other;
        Py_ssize_t i, n;

        n = odict->ma_used;
        for (i = 0; i < n; i++) {
            PyDictKeyEntry *entry = &odict->ma_keys->dk_entries[i];
            if (entry->me_value != NULL) {
                if (PyDict_SetItem((PyObject *)mp, entry->me_key, entry->me_value) < 0)
                    return -1;
            }
        }
        return 0;
    }

    return PyDict_Merge(mp, other, 1);
}
"#;

    // Parse Python
    let python_hir = parse_python(python_source, "test_update.py").expect("Failed to parse Python");

    // Parse C
    let c_hir_module = parse_c(c_source, "dict_update.c").expect("Failed to parse C");

    // Extract the callable parts
    let python_call = extract_python_call(python_hir);
    let c_function = extract_c_function(c_hir_module);

    // Unify
    let mut unifier = Unifier::new();
    let unified_hir = unifier
        .unify(&python_call, &c_function)
        .expect("Failed to unify");

    // Check that we got the DictUpdatePattern
    if let spydecy_hir::unified::UnifiedHIR::Call {
        cross_mapping: Some(mapping),
        ..
    } = &unified_hir
    {
        assert_eq!(
            mapping.pattern,
            spydecy_hir::unified::UnificationPattern::DictUpdatePattern,
            "Expected DictUpdatePattern"
        );
    }

    // Optimize
    let pipeline = OptimizationPipeline::standard();
    let optimized = pipeline.run(unified_hir).expect("Failed to optimize");

    // Generate Rust
    let rust_code = generate_rust(&optimized).expect("Failed to generate Rust code");

    // Verify the generated code contains extend
    assert!(
        rust_code.contains("extend"),
        "Generated code should contain extend call: {rust_code}"
    );

    println!("✅ Dict update pattern test passed!");
    println!("Generated Rust code:\n{rust_code}");
}

#[test]
fn test_list_min_unification_end_to_end() {
    // Python code using min() on list
    let python_source = r#"
def find_minimum(numbers):
    return min_value(numbers)
"#;

    // C implementation (simplified list_min)
    let c_source = r#"
static PyObject* list_min(PyListObject *self) {
    Py_ssize_t i, n;
    PyObject *min_item, *item;
    int cmp;

    n = Py_SIZE(self);
    if (n == 0) {
        PyErr_SetString(PyExc_ValueError, "min() arg is an empty sequence");
        return NULL;
    }

    min_item = PyList_GET_ITEM(self, 0);
    Py_INCREF(min_item);

    for (i = 1; i < n; i++) {
        item = PyList_GET_ITEM(self, i);
        cmp = PyObject_RichCompareBool(item, min_item, Py_LT);
        if (cmp < 0) {
            Py_DECREF(min_item);
            return NULL;
        }
        if (cmp > 0) {
            Py_INCREF(item);
            Py_DECREF(min_item);
            min_item = item;
        }
    }

    return min_item;
}
"#;

    // Parse Python
    let python_hir = parse_python(python_source, "test_min.py").expect("Failed to parse Python");

    // Parse C
    let c_hir_module = parse_c(c_source, "list_min.c").expect("Failed to parse C");

    // Extract the callable parts
    let python_call = extract_python_call(python_hir);
    let c_function = extract_c_function(c_hir_module);

    // Unify
    let mut unifier = Unifier::new();
    let unified_hir = unifier
        .unify(&python_call, &c_function)
        .expect("Failed to unify");

    // Check that we got the ListMinPattern
    if let spydecy_hir::unified::UnifiedHIR::Call {
        cross_mapping: Some(mapping),
        ..
    } = &unified_hir
    {
        assert_eq!(
            mapping.pattern,
            spydecy_hir::unified::UnificationPattern::ListMinPattern,
            "Expected ListMinPattern"
        );
    }

    // Optimize
    let pipeline = OptimizationPipeline::standard();
    let optimized = pipeline.run(unified_hir).expect("Failed to optimize");

    // Generate Rust
    let rust_code = generate_rust(&optimized).expect("Failed to generate Rust code");

    // Verify the generated code contains iter().min()
    assert!(
        rust_code.contains("min"),
        "Generated code should contain min call: {rust_code}"
    );

    println!("✅ List min pattern test passed!");
    println!("Generated Rust code:\n{rust_code}");
}

#[test]
fn test_list_max_unification_end_to_end() {
    // Python code using max() on list
    let python_source = r#"
def find_maximum(numbers):
    return max_value(numbers)
"#;

    // C implementation (simplified list_max)
    let c_source = r#"
static PyObject* list_max(PyListObject *self) {
    Py_ssize_t i, n;
    PyObject *max_item, *item;
    int cmp;

    n = Py_SIZE(self);
    if (n == 0) {
        PyErr_SetString(PyExc_ValueError, "max() arg is an empty sequence");
        return NULL;
    }

    max_item = PyList_GET_ITEM(self, 0);
    Py_INCREF(max_item);

    for (i = 1; i < n; i++) {
        item = PyList_GET_ITEM(self, i);
        cmp = PyObject_RichCompareBool(item, max_item, Py_GT);
        if (cmp < 0) {
            Py_DECREF(max_item);
            return NULL;
        }
        if (cmp > 0) {
            Py_INCREF(item);
            Py_DECREF(max_item);
            max_item = item;
        }
    }

    return max_item;
}
"#;

    // Parse Python
    let python_hir = parse_python(python_source, "test_max.py").expect("Failed to parse Python");

    // Parse C
    let c_hir_module = parse_c(c_source, "list_max.c").expect("Failed to parse C");

    // Extract the callable parts
    let python_call = extract_python_call(python_hir);
    let c_function = extract_c_function(c_hir_module);

    // Unify
    let mut unifier = Unifier::new();
    let unified_hir = unifier
        .unify(&python_call, &c_function)
        .expect("Failed to unify");

    // Check that we got the ListMaxPattern
    if let spydecy_hir::unified::UnifiedHIR::Call {
        cross_mapping: Some(mapping),
        ..
    } = &unified_hir
    {
        assert_eq!(
            mapping.pattern,
            spydecy_hir::unified::UnificationPattern::ListMaxPattern,
            "Expected ListMaxPattern"
        );
    }

    // Optimize
    let pipeline = OptimizationPipeline::standard();
    let optimized = pipeline.run(unified_hir).expect("Failed to optimize");

    // Generate Rust
    let rust_code = generate_rust(&optimized).expect("Failed to generate Rust code");

    // Verify the generated code contains iter().max()
    assert!(
        rust_code.contains("max"),
        "Generated code should contain max call: {rust_code}"
    );

    println!("✅ List max pattern test passed!");
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
