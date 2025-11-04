//! Real-world validation scenarios for common patterns
//!
//! Demonstrates practical usage of dict.items(), list.remove(), list.sort()

use spydecy_c::parse_c;
use spydecy_codegen::generate_rust;
use spydecy_hir::python::PythonHIR;
use spydecy_hir::unified::Unifier;
use spydecy_optimizer::OptimizationPipeline;
use spydecy_python::parse_python;

#[test]
fn real_world_dict_items_config_iteration() {
    // Real-world: Iterating over configuration key-value pairs
    let python_source = r#"
def process_config(config_dict):
    # Common pattern: iterate over config items to validate/process
    return dict_items(config_dict)
"#;

    let c_source = r#"
static PyObject* PyDict_Items(PyDictObject *mp) {
    PyObject *v;
    Py_ssize_t n = mp->ma_used;
    v = PyList_New(n);
    if (v == NULL)
        return NULL;
    if (n != mp->ma_used) {
        PyErr_SetString(PyExc_RuntimeError,
                       "dictionary changed size during iteration");
        Py_DECREF(v);
        return NULL;
    }
    return v;
}
"#;

    // Parse and unify
    let python_hir = parse_python(python_source, "config.py").expect("Failed to parse Python");
    let c_hir = parse_c(c_source, "dictobject.c").expect("Failed to parse C");

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

    // Validate: Should use Rust's idiomatic HashMap::iter()
    assert!(
        rust_code.contains("iter"),
        "Config iteration should use iter(): {rust_code}"
    );

    println!("✅ Real-world scenario: Config iteration");
    println!("Generated: {rust_code}");
}

#[test]
fn real_world_list_remove_event_queue_cleanup() {
    // Real-world: Removing processed events from a queue
    let python_source = r#"
def remove_processed_event(event_queue, processed_event):
    # Common pattern: remove specific item from list/queue
    return remove(event_queue, processed_event)
"#;

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
    PyErr_SetString(PyExc_ValueError, "list.remove(x): x not in list");
    return -1;
}
"#;

    // Parse and unify
    let python_hir = parse_python(python_source, "events.py").expect("Failed to parse Python");
    let c_hir = parse_c(c_source, "listobject.c").expect("Failed to parse C");

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

    // Validate: Should use Rust's retain pattern
    assert!(
        rust_code.contains("retain"),
        "Event removal should use retain(): {rust_code}"
    );

    println!("✅ Real-world scenario: Event queue cleanup");
    println!("Generated: {rust_code}");
}

#[test]
fn real_world_list_sort_priority_queue() {
    // Real-world: Sorting tasks by priority
    let python_source = r#"
def sort_tasks_by_priority(task_list):
    # Common pattern: sort list in-place
    return sort(task_list)
"#;

    let c_source = r#"
static int list_sort(PyListObject *self) {
    MergeState ms;
    Py_ssize_t nremaining;
    Py_ssize_t minrun;
    sortslice lo;
    Py_ssize_t saved_ob_size, saved_allocated;
    PyObject **saved_ob_item;
    PyObject **final_ob_item;
    PyObject *result = NULL;
    Py_ssize_t i;

    // ... TimSort implementation ...

    return 0;
}
"#;

    // Parse and unify
    let python_hir = parse_python(python_source, "tasks.py").expect("Failed to parse Python");
    let c_hir = parse_c(c_source, "listobject.c").expect("Failed to parse C");

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

    // Validate: Should use Rust's sort
    assert!(
        rust_code.contains("sort"),
        "Task sorting should use sort(): {rust_code}"
    );

    println!("✅ Real-world scenario: Priority queue sorting");
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
