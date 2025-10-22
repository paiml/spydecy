//! Metadata tracking for HIR nodes
//!
//! This module tracks additional information about HIR nodes for debugging,
//! error reporting, and the interactive debugger.

use crate::{Language, SourceLocation};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Metadata attached to HIR nodes
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Metadata {
    /// Source location where this node originated
    pub source: Option<SourceLocation>,
    /// Documentation/comments
    pub docs: Option<String>,
    /// Custom attributes/decorators
    pub attributes: Vec<Attribute>,
    /// Cross-references to other nodes
    pub cross_refs: Vec<CrossRef>,
    /// Optimization hints
    pub hints: HashMap<String, String>,
}

impl Metadata {
    /// Create empty metadata
    #[must_use]
    pub fn new() -> Self {
        Self {
            source: None,
            docs: None,
            attributes: Vec::new(),
            cross_refs: Vec::new(),
            hints: HashMap::new(),
        }
    }

    /// Create metadata with source location
    #[must_use]
    pub fn with_source(source: SourceLocation) -> Self {
        Self {
            source: Some(source),
            docs: None,
            attributes: Vec::new(),
            cross_refs: Vec::new(),
            hints: HashMap::new(),
        }
    }

    /// Add documentation
    #[must_use]
    pub fn with_docs(mut self, docs: String) -> Self {
        self.docs = Some(docs);
        self
    }

    /// Add an attribute
    pub fn add_attribute(&mut self, attr: Attribute) {
        self.attributes.push(attr);
    }

    /// Add a cross-reference
    pub fn add_cross_ref(&mut self, cross_ref: CrossRef) {
        self.cross_refs.push(cross_ref);
    }

    /// Add an optimization hint
    pub fn add_hint(&mut self, key: String, value: String) {
        self.hints.insert(key, value);
    }
}

impl Default for Metadata {
    fn default() -> Self {
        Self::new()
    }
}

/// Attribute/decorator on a node
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Attribute {
    /// Attribute name (e.g., "@staticmethod", "__attribute__((packed))")
    pub name: String,
    /// Attribute arguments
    pub args: Vec<String>,
}

impl Attribute {
    /// Create a new attribute
    #[must_use]
    pub fn new(name: String) -> Self {
        Self {
            name,
            args: Vec::new(),
        }
    }

    /// Create an attribute with arguments
    #[must_use]
    pub fn with_args(name: String, args: Vec<String>) -> Self {
        Self { name, args }
    }
}

/// Cross-reference to another node
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrossRef {
    /// Type of relationship
    pub kind: CrossRefKind,
    /// Target node ID
    pub target: crate::NodeId,
    /// Source language where this reference originated
    pub source_language: Language,
    /// Description
    pub description: Option<String>,
}

/// Kind of cross-reference
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CrossRefKind {
    /// Python function calls C function
    PythonToC,
    /// C function implements Python builtin
    CImplementsPython,
    /// Unified call (after optimization)
    Unified,
    /// Type equivalence
    TypeEquiv,
    /// Control flow
    ControlFlow,
}

impl CrossRef {
    /// Create a new cross-reference
    #[must_use]
    pub const fn new(kind: CrossRefKind, target: crate::NodeId, source_language: Language) -> Self {
        Self {
            kind,
            target,
            source_language,
            description: None,
        }
    }

    /// Add a description
    #[must_use]
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata_creation() {
        let meta = Metadata::new();
        assert!(meta.source.is_none());
        assert!(meta.docs.is_none());
        assert!(meta.attributes.is_empty());
    }

    #[test]
    fn test_metadata_with_docs() {
        let meta = Metadata::new().with_docs("Test documentation".to_string());
        assert_eq!(meta.docs, Some("Test documentation".to_string()));
    }

    #[test]
    fn test_attribute_creation() {
        let attr = Attribute::new("staticmethod".to_string());
        assert_eq!(attr.name, "staticmethod");
        assert!(attr.args.is_empty());
    }

    #[test]
    fn test_cross_ref_creation() {
        let cross_ref = CrossRef::new(
            CrossRefKind::PythonToC,
            crate::NodeId::new(42),
            Language::Python,
        );
        assert_eq!(cross_ref.kind, CrossRefKind::PythonToC);
        assert_eq!(cross_ref.target.0, 42);
    }
}
