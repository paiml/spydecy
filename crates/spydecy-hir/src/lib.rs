//! Spydecy Unified High-Level Intermediate Representation (HIR)
//!
//! This module defines the core data structures for Spydecy's Unified HIR.
//! The HIR is the central innovation that enables cross-layer optimization
//! by unifying Python and C representations.
//!
//! # Architecture
//!
//! ```text
//! ┌──────────┐       ┌──────────┐
//! │ Python   │       │   C      │
//! │  AST     │       │  AST     │
//! └────┬─────┘       └────┬─────┘
//!      │                  │
//!      ▼                  ▼
//! ┌──────────┐       ┌──────────┐
//! │ Python   │       │   C      │
//! │  HIR     │       │  HIR     │
//! └────┬─────┘       └────┬─────┘
//!      │                  │
//!      └────────┬─────────┘
//!               ▼
//!       ┌──────────────┐
//!       │ Unified HIR  │ ⭐ Core Innovation
//!       └──────┬───────┘
//!              ▼
//!       ┌──────────────┐
//!       │  Optimizer   │
//!       └──────┬───────┘
//!              ▼
//!       ┌──────────────┐
//!       │   Codegen    │
//!       └──────┬───────┘
//!              ▼
//!         Rust Code
//! ```
//!
//! # Sprint 0 Validation
//!
//! Sprint 0 validated this architecture with a minimal implementation:
//! - Python `len()` + C `list_length()` → Unified HIR → Rust `Vec::len()`
//! - 8/8 tests passing ✅
//! - Zero FFI, zero unsafe ✅
//!
//! This production version extends that success to handle real code.

#![warn(missing_docs, clippy::all, clippy::pedantic)]
#![deny(unsafe_code)]

pub mod c;
pub mod metadata;
pub mod python;
pub mod types;
pub mod unified;

use serde::{Deserialize, Serialize};
use std::fmt;

/// Programming language source
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Language {
    /// Python source code
    Python,
    /// C source code (including `CPython` implementation)
    C,
    /// Rust target code (output)
    Rust,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Python => write!(f, "Python"),
            Self::C => write!(f, "C"),
            Self::Rust => write!(f, "Rust"),
        }
    }
}

/// Source location for error reporting and debugging
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SourceLocation {
    /// Source file path
    pub file: String,
    /// Line number (1-indexed)
    pub line: usize,
    /// Column number (1-indexed)
    pub column: usize,
    /// Source language
    pub language: Language,
}

impl SourceLocation {
    /// Create a new source location
    #[must_use]
    pub const fn new(file: String, line: usize, column: usize, language: Language) -> Self {
        Self {
            file,
            line,
            column,
            language,
        }
    }
}

/// Unique identifier for HIR nodes (for cross-referencing)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(pub u64);

impl NodeId {
    /// Create a new node ID
    #[must_use]
    pub const fn new(id: u64) -> Self {
        Self(id)
    }
}

/// Visibility modifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Visibility {
    /// Public (exported)
    Public,
    /// Private (internal)
    Private,
    /// Module-level (Python) or static (C)
    Module,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_display() {
        assert_eq!(Language::Python.to_string(), "Python");
        assert_eq!(Language::C.to_string(), "C");
        assert_eq!(Language::Rust.to_string(), "Rust");
    }

    #[test]
    fn test_source_location_creation() {
        let loc = SourceLocation::new("test.py".to_owned(), 10, 5, Language::Python);
        assert_eq!(loc.line, 10);
        assert_eq!(loc.column, 5);
        assert_eq!(loc.language, Language::Python);
    }

    #[test]
    fn test_node_id_creation() {
        let id = NodeId::new(42);
        assert_eq!(id.0, 42);
    }
}
