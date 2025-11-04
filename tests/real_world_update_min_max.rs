//! Real-world validation for update, min, and max patterns

use spydecy_c::parse_c;
use spydecy_codegen::generate_rust;
use spydecy_hir::python::PythonHIR;
use spydecy_hir::unified::Unifier;
use spydecy_optimizer::OptimizationPipeline;
use spydecy_python::parse_python;

#[test]
fn real_world_dict_update_config_merge() {
    // Real-world: Merging configuration overrides
    let python_source = r#"
def apply_overrides(base_config, overrides):
    return update(base_config, overrides)
"#;

    let c_source = r#"
static int dict_update(PyDictObject *mp, PyObject *other) {
    if (PyDict_CheckExact(other)) {
        return PyDict_Merge(mp, other, 1);
    }
    return -1;
}
"#;

    let python_hir = parse_python(python_source, "config.py").expect("Failed to parse");
    let c_hir = parse_c(c_source, "dict_update.c").expect("Failed to parse");

    let python_call = extract_python_call(python_hir);
    let c_function = extract_c_function(c_hir);

    let mut unifier = Unifier::new();
    let unified = unifier
        .unify(&python_call, &c_function)
        .expect("Failed to unify");

    let pipeline = OptimizationPipeline::standard();
    let optimized = pipeline.run(unified).expect("Failed to optimize");
    let rust_code = generate_rust(&optimized).expect("Failed to generate");

    assert!(
        rust_code.contains("extend"),
        "Should use extend: {rust_code}"
    );
    println!("✅ Real-world: Config merge with extend()");
}

#[test]
fn real_world_list_min_temperature() {
    // Real-world: Finding minimum temperature reading
    let python_source = r#"
def find_lowest_temp(readings):
    return min_value(readings)
"#;

    let c_source = r#"
static PyObject* list_min(PyListObject *self) {
    Py_ssize_t i, n;
    PyObject *min_item;

    n = Py_SIZE(self);
    if (n == 0) return NULL;

    min_item = PyList_GET_ITEM(self, 0);
    for (i = 1; i < n; i++) {
        PyObject *item = PyList_GET_ITEM(self, i);
        if (PyObject_RichCompareBool(item, min_item, Py_LT) > 0)
            min_item = item;
    }
    return min_item;
}
"#;

    let python_hir = parse_python(python_source, "temps.py").expect("Failed to parse");
    let c_hir = parse_c(c_source, "list_min.c").expect("Failed to parse");

    let python_call = extract_python_call(python_hir);
    let c_function = extract_c_function(c_hir);

    let mut unifier = Unifier::new();
    let unified = unifier
        .unify(&python_call, &c_function)
        .expect("Failed to unify");

    let pipeline = OptimizationPipeline::standard();
    let optimized = pipeline.run(unified).expect("Failed to optimize");
    let rust_code = generate_rust(&optimized).expect("Failed to generate");

    assert!(rust_code.contains("min"), "Should use min: {rust_code}");
    println!("✅ Real-world: Temperature minimum with iter().min()");
}

#[test]
fn real_world_list_max_score() {
    // Real-world: Finding highest game score
    let python_source = r#"
def find_high_score(scores):
    return max_value(scores)
"#;

    let c_source = r#"
static PyObject* list_max(PyListObject *self) {
    Py_ssize_t i, n;
    PyObject *max_item;

    n = Py_SIZE(self);
    if (n == 0) return NULL;

    max_item = PyList_GET_ITEM(self, 0);
    for (i = 1; i < n; i++) {
        PyObject *item = PyList_GET_ITEM(self, i);
        if (PyObject_RichCompareBool(item, max_item, Py_GT) > 0)
            max_item = item;
    }
    return max_item;
}
"#;

    let python_hir = parse_python(python_source, "scores.py").expect("Failed to parse");
    let c_hir = parse_c(c_source, "list_max.c").expect("Failed to parse");

    let python_call = extract_python_call(python_hir);
    let c_function = extract_c_function(c_hir);

    let mut unifier = Unifier::new();
    let unified = unifier
        .unify(&python_call, &c_function)
        .expect("Failed to unify");

    let pipeline = OptimizationPipeline::standard();
    let optimized = pipeline.run(unified).expect("Failed to optimize");
    let rust_code = generate_rust(&optimized).expect("Failed to generate");

    assert!(rust_code.contains("max"), "Should use max: {rust_code}");
    println!("✅ Real-world: High score with iter().max()");
}

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

fn extract_c_function(c_hir: spydecy_hir::c::CHIR) -> spydecy_hir::c::CHIR {
    use spydecy_hir::c::CHIR;
    if let CHIR::TranslationUnit { declarations, .. } = c_hir {
        return declarations.first().cloned().expect("No C declarations");
    }
    panic!("Expected C TranslationUnit");
}
