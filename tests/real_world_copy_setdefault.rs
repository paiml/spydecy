//! Real-world validation scenarios for copy and setdefault patterns
//!
//! Demonstrates practical usage of list.copy(), dict.copy(), dict.setdefault()

use spydecy_c::parse_c;
use spydecy_codegen::generate_rust;
use spydecy_hir::python::PythonHIR;
use spydecy_hir::unified::Unifier;
use spydecy_optimizer::OptimizationPipeline;
use spydecy_python::parse_python;

#[test]
fn real_world_list_copy_defensive_copy() {
    // Real-world: Making defensive copy of list before modification
    let python_source = r#"
def process_with_backup(data_list):
    # Common pattern: defensive copy before mutation
    return copy(data_list)
"#;

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

    // Parse and unify
    let python_hir = parse_python(python_source, "backup.py").expect("Failed to parse Python");
    let c_hir = parse_c(c_source, "list_copy.c").expect("Failed to parse C");

    let python_call = extract_python_call(python_hir);
    let c_function = extract_c_function(c_hir);

    let mut unifier = Unifier::new();
    let unified = unifier
        .unify(&python_call, &c_function)
        .expect("Failed to unify");

    // Optimize and generate
    let pipeline = OptimizationPipeline::standard();
    let optimized = pipeline.run(unified).expect("Failed to optimize");
    let rust_code = generate_rust(&optimized).expect("Failed to generate");

    // Validate: Should use Rust's idiomatic clone()
    assert!(
        rust_code.contains("clone"),
        "Defensive copy should use clone(): {rust_code}"
    );

    println!("✅ Real-world scenario: Defensive list copy");
    println!("Generated: {rust_code}");
}

#[test]
fn real_world_dict_copy_snapshot() {
    // Real-world: Taking snapshot of configuration/state
    let python_source = r#"
def snapshot_state(state_dict):
    # Common pattern: snapshot for rollback/undo
    return dict_copy(state_dict)
"#;

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

    // Parse and unify
    let python_hir = parse_python(python_source, "snapshot.py").expect("Failed to parse Python");
    let c_hir = parse_c(c_source, "dict_copy.c").expect("Failed to parse C");

    let python_call = extract_python_call(python_hir);
    let c_function = extract_c_function(c_hir);

    let mut unifier = Unifier::new();
    let unified = unifier
        .unify(&python_call, &c_function)
        .expect("Failed to unify");

    // Optimize and generate
    let pipeline = OptimizationPipeline::standard();
    let optimized = pipeline.run(unified).expect("Failed to optimize");
    let rust_code = generate_rust(&optimized).expect("Failed to generate");

    // Validate: Should use Rust's idiomatic clone()
    assert!(
        rust_code.contains("clone"),
        "State snapshot should use clone(): {rust_code}"
    );

    println!("✅ Real-world scenario: Dictionary state snapshot");
    println!("Generated: {rust_code}");
}

#[test]
fn real_world_dict_setdefault_cache() {
    // Real-world: Cache with default value initialization
    let python_source = r#"
def get_or_create_cache_entry(cache, key, default_value):
    # Common pattern: lazy initialization with default
    return setdefault(cache, key, default_value)
"#;

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

    // Parse and unify
    let python_hir = parse_python(python_source, "cache.py").expect("Failed to parse Python");
    let c_hir = parse_c(c_source, "dict_setdefault.c").expect("Failed to parse C");

    let python_call = extract_python_call(python_hir);
    let c_function = extract_c_function(c_hir);

    let mut unifier = Unifier::new();
    let unified = unifier
        .unify(&python_call, &c_function)
        .expect("Failed to unify");

    // Optimize and generate
    let pipeline = OptimizationPipeline::standard();
    let optimized = pipeline.run(unified).expect("Failed to optimize");
    let rust_code = generate_rust(&optimized).expect("Failed to generate");

    // Validate: Should use Rust's entry API
    assert!(
        rust_code.contains("entry") && rust_code.contains("or_insert"),
        "Cache initialization should use entry().or_insert(): {rust_code}"
    );

    println!("✅ Real-world scenario: Cache with lazy initialization");
    println!("Generated: {rust_code}");
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
