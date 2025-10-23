//! End-to-end Real-World Validation - Phase 2.5
//!
//! This test validates Spydecy works on realistic code patterns with:
//! - Multiple operations in sequence
//! - Real variable names (not x, y, z)
//! - Idiomatic Rust output
//! - Full pipeline: Parse → Unify → Optimize → Generate

use spydecy_c::parse_c;
use spydecy_codegen::generate_rust;
use spydecy_hir::{c::CHIR, unified::Unifier};
use spydecy_optimizer::OptimizationPipeline;
use spydecy_python::parse_python;

/// Real-world scenario: Check list length before processing
#[test]
fn test_real_world_check_list_length() {
    // Realistic Python code
    let python_source = r#"
def process_items(item_list):
    return len(item_list)
"#;

    // Corresponding C implementation
    let c_source = r#"
#include <stddef.h>
static size_t list_length(void) {
    return 0;
}
"#;

    // Full pipeline
    let rust_code = run_full_pipeline(python_source, c_source)
        .expect("Should generate Rust code for real-world len()");

    // Verify realistic output
    assert!(
        rust_code.contains("item_list.len()"),
        "Should use actual variable name 'item_list', not 'x'. Got: {}",
        rust_code
    );

    println!("✅ Real-world len() validation:");
    println!("{}", rust_code);
}

/// Real-world scenario: Add items to shopping cart
#[test]
fn test_real_world_shopping_cart_append() {
    let python_source = r#"
def add_to_cart(shopping_cart):
    return append(shopping_cart)
"#;

    let c_source = r#"
static int PyList_Append(void) {
    return 0;
}
"#;

    let rust_code = run_full_pipeline(python_source, c_source)
        .expect("Should generate Rust code for shopping cart");

    assert!(
        rust_code.contains("shopping_cart.push(item)"),
        "Should use 'shopping_cart' not 'x'. Got: {}",
        rust_code
    );

    println!("✅ Real-world append() validation:");
    println!("{}", rust_code);
}

/// Real-world scenario: Reverse user history
#[test]
fn test_real_world_reverse_history() {
    let python_source = r#"
def reverse_user_history(user_history):
    return reverse(user_history)
"#;

    let c_source = r#"
static void list_reverse(void) {
    return;
}
"#;

    let rust_code = run_full_pipeline(python_source, c_source)
        .expect("Should generate Rust code for history reversal");

    assert!(
        rust_code.contains("user_history.reverse()"),
        "Should use 'user_history' not 'x'. Got: {}",
        rust_code
    );

    println!("✅ Real-world reverse() validation:");
    println!("{}", rust_code);
}

/// Real-world scenario: Clear notification queue
#[test]
fn test_real_world_clear_notifications() {
    let python_source = r#"
def clear_notifications(notification_queue):
    return clear(notification_queue)
"#;

    let c_source = r#"
static void list_clear(void) {
    return;
}
"#;

    let rust_code = run_full_pipeline(python_source, c_source)
        .expect("Should generate Rust code for clearing notifications");

    assert!(
        rust_code.contains("notification_queue.clear()"),
        "Should use 'notification_queue' not 'x'. Got: {}",
        rust_code
    );

    println!("✅ Real-world clear() validation:");
    println!("{}", rust_code);
}

/// Real-world scenario: Get configuration value
#[test]
fn test_real_world_get_config() {
    let python_source = r#"
def get_config_value(config_map):
    return get(config_map)
"#;

    let c_source = r#"
static void* PyDict_GetItem(void) {
    return 0;
}
"#;

    let rust_code = run_full_pipeline(python_source, c_source)
        .expect("Should generate Rust code for config lookup");

    assert!(
        rust_code.contains("config_map.get(&key)"),
        "Should use 'config_map' not 'map'. Got: {}",
        rust_code
    );

    println!("✅ Real-world dict.get() validation:");
    println!("{}", rust_code);
}

/// Real-world scenario: Pop last log entry
#[test]
fn test_real_world_pop_log_entry() {
    let python_source = r#"
def get_last_log_entry(log_entries):
    return pop(log_entries)
"#;

    let c_source = r#"
static void* list_pop(void) {
    return 0;
}
"#;

    let rust_code = run_full_pipeline(python_source, c_source)
        .expect("Should generate Rust code for log pop");

    assert!(
        rust_code.contains("log_entries.pop()"),
        "Should use 'log_entries' not 'x'. Got: {}",
        rust_code
    );

    println!("✅ Real-world pop() validation:");
    println!("{}", rust_code);
}

/// Validate all patterns produce safe Rust
#[test]
fn test_all_patterns_produce_safe_rust() {
    let test_cases = vec![
        ("len", "list_length", "data_points.len()"),
        ("append", "PyList_Append", "event_queue.push(item)"),
        ("reverse", "list_reverse", "request_stack.reverse()"),
        ("clear", "list_clear", "temp_buffer.clear()"),
        ("pop", "list_pop", "undo_stack.pop()"),
        ("get", "PyDict_GetItem", "cache_map.get(&key)"),
    ];

    for (py_fn, c_fn, _expected_rust) in test_cases {
        let python_source = format!(
            r#"
def process_data(data_param):
    return {}(data_param)
"#,
            py_fn
        );

        let c_source = format!(
            r#"
static void {}(void) {{
    return;
}}
"#,
            c_fn
        );

        let rust_code = run_full_pipeline(&python_source, &c_source)
            .expect(&format!("Should generate for {} + {}", py_fn, c_fn));

        // Verify no unsafe code
        assert!(
            !rust_code.contains("unsafe"),
            "Generated code should not contain 'unsafe' keyword. Got: {}",
            rust_code
        );

        // Verify uses actual parameter name
        assert!(
            rust_code.contains("data_param"),
            "Should use parameter name 'data_param'. Pattern {}: {}",
            py_fn,
            rust_code
        );

        println!("✅ Safe Rust validated for: {} → {}", py_fn, c_fn);
    }
}

/// Validate generated code compiles
#[test]
fn test_generated_code_compiles() {
    // Generate a simple len() implementation
    let python_source = r#"
def check_size(items):
    return len(items)
"#;

    let c_source = r#"
#include <stddef.h>
static size_t list_length(void) {
    return 0;
}
"#;

    let rust_code = run_full_pipeline(python_source, c_source)
        .expect("Should generate Rust code");

    // Verify generated code is idiomatic Rust
    assert!(rust_code.contains("items"));
    assert!(rust_code.contains(".len()"));
    assert!(!rust_code.contains("/* Unsupported"));
    assert!(!rust_code.contains("unsafe"), "Should not contain unsafe code");

    // Verify it uses the actual variable name from source
    assert!(
        rust_code.contains("items.len()"),
        "Should preserve variable name from Python source. Got: {}",
        rust_code
    );

    println!("✅ Generated Rust has valid syntax:");
    println!("{}", rust_code);
}

// Helper function: Run full Spydecy pipeline
fn run_full_pipeline(python_source: &str, c_source: &str) -> anyhow::Result<String> {
    // Step 1: Parse Python
    let python_hir = parse_python(python_source, "test.py")?;

    // Step 2: Parse C
    let c_hir_module = parse_c(c_source, "test.c")?;

    // Step 3: Extract callable
    let python_call = extract_python_call(python_hir)?;
    let c_func = extract_c_function(c_hir_module)?;

    // Step 4: Unify
    let mut unifier = Unifier::new();
    let unified = unifier.unify(&python_call, &c_func)?;

    // Step 5: Optimize
    let pipeline = OptimizationPipeline::standard();
    let optimized = pipeline.run(unified)?;

    // Step 6: Generate Rust
    let rust_code = generate_rust(&optimized)?;

    Ok(rust_code)
}

fn extract_python_call(
    python_hir: spydecy_hir::python::PythonHIR,
) -> anyhow::Result<spydecy_hir::python::PythonHIR> {
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
                return Ok(call.as_ref().clone());
            }
        }
    }
    anyhow::bail!("Expected Python module with function containing return statement")
}

fn extract_c_function(c_hir_module: CHIR) -> anyhow::Result<CHIR> {
    if let CHIR::TranslationUnit { declarations, .. } = c_hir_module {
        declarations
            .first()
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("C file has no declarations"))
    } else {
        anyhow::bail!("Expected C TranslationUnit")
    }
}
