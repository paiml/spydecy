//! Sprint 0: Tracer Bullet - Minimal HIR Validation
//!
//! This module implements the absolute minimum HIR needed to validate
//! the core assumption: Python HIR + C HIR can be unified.
//!
//! Micro-target: `len()` function
//! - Python: `def my_len(x): return len(x)`
//! - C: `list_length(PyListObject *self)`
//! - Result: Pure Rust `x.len()` with no FFI

#![warn(missing_docs, clippy::all, clippy::pedantic)]
#![deny(unsafe_code)]

use serde::{Deserialize, Serialize};

/// Minimal HIR to test unification concept
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MiniHIR {
    /// Python-level function
    PythonFunction {
        /// Function name
        name: String,
        /// Function body
        body: Vec<MiniHIR>,
    },

    /// Python function call
    PythonCall {
        /// Function being called (e.g., "len")
        callee: String,
        /// Arguments to the call
        args: Vec<MiniHIR>,
    },

    /// Python variable reference
    PythonVar(String),

    /// C-level function
    CFunction {
        /// Function name
        name: String,
        /// Function body
        body: Vec<MiniHIR>,
    },

    /// C field access (e.g., `Py_SIZE(obj)`)
    CFieldAccess {
        /// Object being accessed
        object: Box<MiniHIR>,
        /// Field name
        field: String,
    },

    /// C pointer type
    CPointer(Box<MiniHIR>),

    /// Unified cross-language call
    UnifiedCall {
        /// Target language after optimization
        target_language: Language,
        /// Function being called
        callee: String,
        /// Arguments
        args: Vec<MiniHIR>,
    },
}

/// Programming language
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    /// Python source
    Python,
    /// C source
    C,
    /// Rust target (after optimization)
    Rust,
}

impl MiniHIR {
    /// Unify Python and C HIRs into a single representation
    ///
    /// This is the CRITICAL function that validates the entire Spydecy architecture.
    /// If this works, the project is viable. If not, we need an architectural pivot.
    ///
    /// # Errors
    ///
    /// Returns an error if the Python and C HIRs cannot be unified
    pub fn unify(python_hir: &MiniHIR, c_hir: &MiniHIR) -> anyhow::Result<MiniHIR> {
        match (python_hir, c_hir) {
            // Key case: Python len() → C list_length()
            (
                MiniHIR::PythonCall {
                    callee,
                    args: py_args,
                },
                MiniHIR::CFunction { name, .. },
            ) if callee == "len" && name == "list_length" => {
                // This is the "magic" - recognize the relationship
                Ok(MiniHIR::UnifiedCall {
                    target_language: Language::Rust,
                    callee: "Vec::len".to_owned(),
                    args: py_args.clone(),
                })
            }

            // No unification possible
            _ => anyhow::bail!("Cannot unify Python HIR {python_hir:?} with C HIR {c_hir:?}"),
        }
    }

    /// Eliminate Python→C boundaries through optimization
    ///
    /// Converts cross-language calls into pure Rust.
    #[must_use]
    pub fn eliminate_boundary(self) -> MiniHIR {
        match self {
            MiniHIR::UnifiedCall {
                target_language: Language::Python,
                callee,
                args,
            } if callee == "len" => {
                // Eliminate boundary: Python len() → Rust Vec::len()
                MiniHIR::UnifiedCall {
                    target_language: Language::Rust,
                    callee: "Vec::len".to_owned(),
                    args,
                }
            }
            _ => self,
        }
    }

    /// Generate Rust code from HIR
    ///
    /// This is a simplified code generator for the tracer bullet.
    #[must_use]
    pub fn codegen(&self) -> String {
        match self {
            MiniHIR::PythonFunction { name, body } => {
                let body_code: Vec<String> = body.iter().map(Self::codegen).collect();
                format!(
                    "pub fn {}<T>(x: &Vec<T>) -> usize {{\n    {}\n}}",
                    name,
                    body_code.join("\n    ")
                )
            }

            MiniHIR::UnifiedCall {
                target_language: Language::Rust,
                callee,
                args: _,
            } if callee == "Vec::len" => "x.len()".to_owned(),

            MiniHIR::PythonVar(name) => name.clone(),

            _ => format!("/* Not implemented: {self:?} */"),
        }
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;

    #[test]
    fn test_unify_len_call() {
        // Python: len(x)
        let python_hir = MiniHIR::PythonCall {
            callee: "len".to_owned(),
            args: vec![MiniHIR::PythonVar("x".to_owned())],
        };

        // C: list_length(obj)
        let c_hir = MiniHIR::CFunction {
            name: "list_length".to_owned(),
            body: vec![MiniHIR::CFieldAccess {
                object: Box::new(MiniHIR::PythonVar("self".to_owned())),
                field: "Py_SIZE".to_owned(),
            }],
        };

        // Unify
        let unified = MiniHIR::unify(&python_hir, &c_hir).expect("Unification should succeed");

        // Should create UnifiedCall targeting Rust
        assert_eq!(
            unified,
            MiniHIR::UnifiedCall {
                target_language: Language::Rust,
                callee: "Vec::len".to_owned(),
                args: vec![MiniHIR::PythonVar("x".to_owned())],
            }
        );
    }

    #[test]
    fn test_eliminate_boundary() {
        let unified = MiniHIR::UnifiedCall {
            target_language: Language::Python,
            callee: "len".to_owned(),
            args: vec![MiniHIR::PythonVar("x".to_owned())],
        };

        let optimized = unified.eliminate_boundary();

        assert_eq!(
            optimized,
            MiniHIR::UnifiedCall {
                target_language: Language::Rust,
                callee: "Vec::len".to_owned(),
                args: vec![MiniHIR::PythonVar("x".to_owned())],
            }
        );
    }

    #[test]
    fn test_codegen_rust() {
        let python_func = MiniHIR::PythonFunction {
            name: "my_len".to_owned(),
            body: vec![MiniHIR::UnifiedCall {
                target_language: Language::Rust,
                callee: "Vec::len".to_owned(),
                args: vec![MiniHIR::PythonVar("x".to_owned())],
            }],
        };

        let rust_code = python_func.codegen();

        assert!(rust_code.contains("pub fn my_len"));
        assert!(rust_code.contains("x.len()"));
        assert!(!rust_code.contains("extern"));
        assert!(!rust_code.contains("PyObject"));
    }

    #[test]
    fn test_unify_fails_on_mismatch() {
        let python_hir = MiniHIR::PythonVar("x".to_owned());
        let c_hir = MiniHIR::PythonVar("y".to_owned());

        let result = MiniHIR::unify(&python_hir, &c_hir);
        assert!(result.is_err());
    }
}
