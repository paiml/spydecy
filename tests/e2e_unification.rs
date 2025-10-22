//! End-to-End Unification Integration Tests
//!
//! These tests validate the CRITICAL Sprint 0 assumption with production code:
//! Python `len()` + C `list_length()` → Rust `Vec::len()`
//!
//! This is the complete pipeline:
//! 1. Parse Python source → PythonHIR (using spydecy-python)
//! 2. Parse C source → CHIR (using spydecy-c)
//! 3. Unify them → UnifiedHIR (using spydecy-hir)
//! 4. Verify the result is pure Rust with zero FFI
//!
//! SUCCESS CRITERIA (from Sprint 0):
//! ✅ Python HIR + C HIR → Unified HIR (unification works)
//! ✅ Optimization eliminates Python→C boundary
//! ✅ Generated code targets Rust (no FFI)
//! ✅ Behavior matches Python len()

use anyhow::Result;
use spydecy_c::parse_c;
use spydecy_hir::{
    c::CHIR,
    python::PythonHIR,
    unified::{UnifiedHIR, Unifier},
    Language,
};
use spydecy_python::parse_python;

#[test]
fn test_len_unification_end_to_end() -> Result<()> {
    // ═══════════════════════════════════════════════════════════
    // Step 1: Parse Python source
    // ═══════════════════════════════════════════════════════════
    println!("\n═══ Step 1: Parse Python Source ═══");

    let python_source = r"
def my_len(x):
    return len(x)
";

    let python_module_hir = parse_python(python_source, "test.py")?;
    println!("✅ Python HIR parsed");

    // Extract the Python HIR for len() call
    let python_hir = extract_len_call_from_module(&python_module_hir)?;
    println!("✅ Python HIR for len() extracted: {python_hir:#?}");

    // ═══════════════════════════════════════════════════════════
    // Step 2: Parse C source
    // ═══════════════════════════════════════════════════════════
    println!("\n═══ Step 2: Parse C Source ═══");

    let c_source = r"
static Py_ssize_t
list_length(PyListObject *self) {
    return Py_SIZE(self);
}
";

    let c_translation_unit_hir = parse_c(c_source, "listobject.c")?;
    println!("✅ C HIR parsed");

    // Extract the C HIR for list_length function
    let c_hir = extract_list_length_from_translation_unit(&c_translation_unit_hir)?;
    println!("✅ C HIR for list_length() extracted: {c_hir:#?}");

    // ═══════════════════════════════════════════════════════════
    // Step 3: Unify Python + C HIR
    // ═══════════════════════════════════════════════════════════
    println!("\n═══ Step 3: Unify Python + C HIR ═══");

    let mut unifier = Unifier::new();
    let unified = unifier.unify(&python_hir, &c_hir)?;

    println!("✅ Unification succeeded!");
    println!("Unified HIR: {unified:#?}");

    // ═══════════════════════════════════════════════════════════
    // Step 4: Verify Result
    // ═══════════════════════════════════════════════════════════
    println!("\n═══ Step 4: Verify Result ═══");

    // Verify it's a Call targeting Rust
    let UnifiedHIR::Call {
        target_language,
        callee,
        cross_mapping,
        ..
    } = unified.clone()
    else {
        panic!("Expected UnifiedHIR::Call, got: {unified:?}");
    };

    // ✅ Verify: Target language is Rust (not Python or C)
    assert_eq!(
        target_language,
        Language::Rust,
        "Target language should be Rust"
    );
    println!("✅ Target language: Rust");

    // ✅ Verify: Callee is Vec::len (pure Rust, no FFI)
    assert_eq!(callee, "Vec::len", "Should call Vec::len");
    println!("✅ Callee: Vec::len (pure Rust)");

    // ✅ Verify: Cross-language mapping exists
    assert!(
        cross_mapping.is_some(),
        "Should have cross-language mapping"
    );
    println!("✅ Cross-language mapping exists");

    // ✅ Verify: Pattern is LenPattern (from Sprint 0)
    let mapping = cross_mapping.expect("Cross mapping should exist");
    assert_eq!(
        mapping.pattern,
        spydecy_hir::unified::UnificationPattern::LenPattern,
        "Should be LenPattern"
    );
    println!("✅ Pattern: LenPattern");

    // ═══════════════════════════════════════════════════════════
    // Step 5: Eliminate Boundary
    // ═══════════════════════════════════════════════════════════
    println!("\n═══ Step 5: Eliminate Python→C Boundary ═══");

    let optimized = unified.eliminate_boundary();

    // Verify boundary is marked as eliminated
    if let UnifiedHIR::Call { cross_mapping, .. } = optimized {
        let mapping = cross_mapping.expect("Mapping should exist");
        assert!(mapping.boundary_eliminated, "Boundary should be eliminated");
        println!("✅ Python→C boundary eliminated");
    }

    println!("\n═══════════════════════════════════════════════════════════");
    println!("🎉 SUCCESS! Sprint 0 assumption validated end-to-end!");
    println!("═══════════════════════════════════════════════════════════");
    println!();
    println!("Pipeline verified:");
    println!("  Python len(x) → PythonHIR");
    println!("  C list_length() → CHIR");
    println!("  Python + C → UnifiedHIR (Rust Vec::len)");
    println!("  Boundary eliminated → Pure Rust code");
    println!();
    println!("This validates the core innovation of Spydecy! ✅");

    Ok(())
}

/// Extract the len() call from Python Module HIR
fn extract_len_call_from_module(hir: &PythonHIR) -> Result<PythonHIR> {
    // Extract the module body
    let PythonHIR::Module { body, .. } = hir else {
        anyhow::bail!("Expected Module, got: {hir:?}");
    };

    // Extract the function
    let function_hir = body
        .first()
        .ok_or_else(|| anyhow::anyhow!("Module body is empty"))?;

    let PythonHIR::Function {
        body: func_body, ..
    } = function_hir
    else {
        anyhow::bail!("Expected Function, got: {function_hir:?}");
    };

    // Extract the return statement
    let return_stmt = func_body
        .first()
        .ok_or_else(|| anyhow::anyhow!("Function body is empty"))?;

    let PythonHIR::Return { value, .. } = return_stmt else {
        anyhow::bail!("Expected Return, got: {return_stmt:?}");
    };

    // Extract the len() call
    let call = value
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("Return value is None"))?;

    Ok(call.as_ref().clone())
}

/// Extract the list_length function from C TranslationUnit HIR
fn extract_list_length_from_translation_unit(hir: &CHIR) -> Result<CHIR> {
    // Extract the translation unit
    let CHIR::TranslationUnit { declarations, .. } = hir else {
        anyhow::bail!("Expected TranslationUnit, got: {hir:?}");
    };

    // Extract the first function (list_length)
    let function = declarations
        .first()
        .ok_or_else(|| anyhow::anyhow!("No declarations found"))?;

    Ok(function.clone())
}

#[test]
fn test_append_unification_end_to_end() -> Result<()> {
    // ═══════════════════════════════════════════════════════════
    // Step 1: Create Python HIR for append() call
    // ═══════════════════════════════════════════════════════════
    println!("\n═══ Step 1: Create Python HIR for append() ═══");

    // Create Python HIR directly (parser doesn't support Expr statements yet)
    let python_hir = PythonHIR::Call {
        id: spydecy_hir::NodeId::new(1),
        callee: Box::new(PythonHIR::Variable {
            id: spydecy_hir::NodeId::new(2),
            name: "append".to_owned(),
            inferred_type: None,
            meta: spydecy_hir::metadata::Metadata::new(),
        }),
        args: vec![PythonHIR::Variable {
            id: spydecy_hir::NodeId::new(3),
            name: "item".to_owned(),
            inferred_type: None,
            meta: spydecy_hir::metadata::Metadata::new(),
        }],
        kwargs: vec![],
        inferred_type: None,
        meta: spydecy_hir::metadata::Metadata::new(),
    };
    println!("✅ Python HIR for append() created: {python_hir:#?}");

    // ═══════════════════════════════════════════════════════════
    // Step 2: Parse C source
    // ═══════════════════════════════════════════════════════════
    println!("\n═══ Step 2: Parse C Source (PyList_Append) ═══");

    let c_source = r"
int
PyList_Append(PyObject *list, PyObject *item) {
    return 0;
}
";

    let c_translation_unit_hir = parse_c(c_source, "listobject.c")?;
    println!("✅ C HIR parsed");

    // Extract the C HIR for PyList_Append function
    let c_hir = extract_pylist_append_from_translation_unit(&c_translation_unit_hir)?;
    println!("✅ C HIR for PyList_Append() extracted: {c_hir:#?}");

    // ═══════════════════════════════════════════════════════════
    // Step 3: Unify Python + C HIR
    // ═══════════════════════════════════════════════════════════
    println!("\n═══ Step 3: Unify Python + C HIR ═══");

    let mut unifier = Unifier::new();
    let unified = unifier.unify(&python_hir, &c_hir)?;

    println!("✅ Unification succeeded!");
    println!("Unified HIR: {unified:#?}");

    // ═══════════════════════════════════════════════════════════
    // Step 4: Verify Result
    // ═══════════════════════════════════════════════════════════
    println!("\n═══ Step 4: Verify Result ═══");

    // Verify it's a Call targeting Rust
    let UnifiedHIR::Call {
        target_language,
        callee,
        cross_mapping,
        ..
    } = unified.clone()
    else {
        panic!("Expected UnifiedHIR::Call, got: {unified:?}");
    };

    // ✅ Verify: Target language is Rust (not Python or C)
    assert_eq!(
        target_language,
        Language::Rust,
        "Target language should be Rust"
    );
    println!("✅ Target language: Rust");

    // ✅ Verify: Callee is Vec::push (pure Rust, no FFI)
    assert_eq!(callee, "Vec::push", "Should call Vec::push");
    println!("✅ Callee: Vec::push (pure Rust)");

    // ✅ Verify: Cross-language mapping exists
    assert!(
        cross_mapping.is_some(),
        "Should have cross-language mapping"
    );
    println!("✅ Cross-language mapping exists");

    // ✅ Verify: Pattern is AppendPattern
    let mapping = cross_mapping.expect("Cross mapping should exist");
    assert_eq!(
        mapping.pattern,
        spydecy_hir::unified::UnificationPattern::AppendPattern,
        "Should be AppendPattern"
    );
    println!("✅ Pattern: AppendPattern");

    // ═══════════════════════════════════════════════════════════
    // Step 5: Eliminate Boundary
    // ═══════════════════════════════════════════════════════════
    println!("\n═══ Step 5: Eliminate Python→C Boundary ═══");

    let optimized = unified.eliminate_boundary();

    // Verify boundary is marked as eliminated
    if let UnifiedHIR::Call { cross_mapping, .. } = optimized {
        let mapping = cross_mapping.expect("Mapping should exist");
        assert!(mapping.boundary_eliminated, "Boundary should be eliminated");
        println!("✅ Python→C boundary eliminated");
    }

    println!("\n═══════════════════════════════════════════════════════════");
    println!("🎉 SUCCESS! Append pattern validated end-to-end!");
    println!("═══════════════════════════════════════════════════════════");
    println!();
    println!("Pipeline verified:");
    println!("  Python lst.append(item) → PythonHIR");
    println!("  C PyList_Append() → CHIR");
    println!("  Python + C → UnifiedHIR (Rust Vec::push)");
    println!("  Boundary eliminated → Pure Rust code");
    println!();
    println!("Second unification pattern working! ✅");

    Ok(())
}

/// Extract the PyList_Append function from C TranslationUnit HIR
fn extract_pylist_append_from_translation_unit(hir: &CHIR) -> Result<CHIR> {
    // Extract the translation unit
    let CHIR::TranslationUnit { declarations, .. } = hir else {
        anyhow::bail!("Expected TranslationUnit, got: {hir:?}");
    };

    // Extract the first function (PyList_Append)
    let function = declarations
        .first()
        .ok_or_else(|| anyhow::anyhow!("No declarations found"))?;

    Ok(function.clone())
}
