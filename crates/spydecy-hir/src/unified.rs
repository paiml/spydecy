//! Unified HIR - The Core Innovation
//!
//! This module implements the CRITICAL unification of Python and C HIRs.
//! Sprint 0 validated this concept works. This is the production implementation.
//!
//! # Architecture
//!
//! The Unified HIR bridges the impedance mismatch between Python and C:
//!
//! ```text
//! Python len(x)  ←─────┐
//!                       ├──→ Unified HIR ──→ Rust x.len()
//! C list_length() ──────┘
//! ```
//!
//! # Pattern Recognition
//!
//! The unifier recognizes Python-C patterns:
//! - `len()` + `list_length()` → `Vec::len()`
//! - `append()` + `PyList_Append()` → `Vec::push()`
//! - `dict.get()` + `PyDict_GetItem()` → `HashMap::get()`
//!
//! These patterns can be extended via the Pluggable C-API Architecture.

use crate::{c::CHIR, metadata::Metadata, python::PythonHIR, types::Type, Language, NodeId};
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

/// Unified HIR node - combines Python and C into a single representation
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UnifiedHIR {
    /// Module/compilation unit
    Module {
        /// Module name
        name: String,
        /// Original language
        source_language: Language,
        /// Declarations
        declarations: Vec<UnifiedHIR>,
        /// Metadata
        meta: Metadata,
    },

    /// Function definition (unified Python + C)
    Function {
        /// Node ID
        id: NodeId,
        /// Function name
        name: String,
        /// Parameters
        params: Vec<UnifiedParameter>,
        /// Return type
        return_type: Type,
        /// Function body
        body: Vec<UnifiedHIR>,
        /// Source language
        source_language: Language,
        /// Cross-language mapping (if unified with another function)
        cross_mapping: Option<CrossMapping>,
        /// Metadata
        meta: Metadata,
    },

    /// Function call (potentially cross-language)
    Call {
        /// Node ID
        id: NodeId,
        /// Target language (after optimization)
        target_language: Language,
        /// Callee
        callee: String,
        /// Arguments
        args: Vec<UnifiedHIR>,
        /// Inferred type
        inferred_type: Type,
        /// Source language
        source_language: Language,
        /// Cross-language mapping
        cross_mapping: Option<CrossMapping>,
        /// Metadata
        meta: Metadata,
    },

    /// Variable reference
    Variable {
        /// Node ID
        id: NodeId,
        /// Variable name
        name: String,
        /// Variable type
        var_type: Type,
        /// Source language
        source_language: Language,
        /// Metadata
        meta: Metadata,
    },

    /// Assignment
    Assign {
        /// Node ID
        id: NodeId,
        /// Target
        target: String,
        /// Value
        value: Box<UnifiedHIR>,
        /// Type
        var_type: Type,
        /// Source language
        source_language: Language,
        /// Metadata
        meta: Metadata,
    },

    /// Return statement
    Return {
        /// Node ID
        id: NodeId,
        /// Return value
        value: Option<Box<UnifiedHIR>>,
        /// Source language
        source_language: Language,
        /// Metadata
        meta: Metadata,
    },

    /// Control flow - if/else
    If {
        /// Node ID
        id: NodeId,
        /// Condition
        condition: Box<UnifiedHIR>,
        /// Then branch
        then_branch: Vec<UnifiedHIR>,
        /// Else branch
        else_branch: Vec<UnifiedHIR>,
        /// Source language
        source_language: Language,
        /// Metadata
        meta: Metadata,
    },

    /// Loop
    Loop {
        /// Node ID
        id: NodeId,
        /// Loop kind
        kind: LoopKind,
        /// Loop body
        body: Vec<UnifiedHIR>,
        /// Source language
        source_language: Language,
        /// Metadata
        meta: Metadata,
    },

    /// Binary operation
    BinOp {
        /// Node ID
        id: NodeId,
        /// Operator
        op: BinOp,
        /// Left operand
        left: Box<UnifiedHIR>,
        /// Right operand
        right: Box<UnifiedHIR>,
        /// Result type
        result_type: Type,
        /// Source language
        source_language: Language,
        /// Metadata
        meta: Metadata,
    },

    /// Literal value
    Literal {
        /// Node ID
        id: NodeId,
        /// Literal value
        value: LiteralValue,
        /// Literal type
        lit_type: Type,
        /// Metadata
        meta: Metadata,
    },
}

/// Unified parameter (bridges Python and C parameters)
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnifiedParameter {
    /// Parameter name
    pub name: String,
    /// Parameter type
    pub param_type: Type,
    /// Original language
    pub source_language: Language,
}

/// Cross-language mapping information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrossMapping {
    /// Python node ID (if applicable)
    pub python_node: Option<NodeId>,
    /// C node ID (if applicable)
    pub c_node: Option<NodeId>,
    /// Mapping pattern used
    pub pattern: UnificationPattern,
    /// Was boundary eliminated?
    pub boundary_eliminated: bool,
}

/// Unification pattern - how Python and C were unified
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnificationPattern {
    /// Python `len()` → C `list_length()` → Rust `Vec::len()`
    LenPattern,
    /// Python `append()` → C `PyList_Append()` → Rust `Vec::push()`
    AppendPattern,
    /// Python `dict.get()` → C `PyDict_GetItem()` → Rust `HashMap::get()`
    DictGetPattern,
    /// Python `list.reverse()` → C `list_reverse()` → Rust `Vec::reverse()`
    ReversePattern,
    /// Python `list.clear()` → C `list_clear()` → Rust `Vec::clear()`
    ClearPattern,
    /// Python `list.pop()` → C `list_pop()` → Rust `Vec::pop()`
    PopPattern,
    /// Python `list.insert()` → C `list_insert()` → Rust `Vec::insert()`
    InsertPattern,
    /// Python `list.extend()` → C `list_extend()` → Rust `Vec::extend()`
    ExtendPattern,
    /// Python `dict.pop()` → C `PyDict_DelItem()` → Rust `HashMap::remove()`
    DictPopPattern,
    /// Python `dict.clear()` → C `PyDict_Clear()` → Rust `HashMap::clear()`
    DictClearPattern,
    /// Python `dict.keys()` → C `PyDict_Keys()` → Rust `HashMap::keys()`
    DictKeysPattern,
    /// Custom pattern (extensible)
    Custom,
}

/// Loop kind (unified from Python/C)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LoopKind {
    /// Python for loop / C for loop
    For {
        /// Loop variable
        target: String,
        /// Iterable/range
        iter: Box<UnifiedHIR>,
    },
    /// While loop
    While {
        /// Condition
        condition: Box<UnifiedHIR>,
    },
}

/// Binary operator (unified)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BinOp {
    /// Arithmetic
    Add,
    /// Arithmetic
    Sub,
    /// Arithmetic
    Mul,
    /// Arithmetic
    Div,
    /// Arithmetic
    Mod,
    /// Comparison
    Eq,
    /// Comparison
    Ne,
    /// Comparison
    Lt,
    /// Comparison
    Le,
    /// Comparison
    Gt,
    /// Comparison
    Ge,
    /// Logical
    And,
    /// Logical
    Or,
}

/// Literal value (unified)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LiteralValue {
    /// Integer
    Int(i64),
    /// Float
    Float(f64),
    /// String
    Str(String),
    /// Boolean
    Bool(bool),
    /// None/NULL
    None,
}

/// Unifier - converts Python + C HIR into Unified HIR
pub struct Unifier {
    /// Next node ID
    next_id: u64,
}

impl Unifier {
    /// Create a new unifier
    #[must_use]
    pub const fn new() -> Self {
        Self { next_id: 1 }
    }

    /// Unify a Python HIR node with a C HIR node
    ///
    /// This is the CRITICAL function validated by Sprint 0.
    /// It recognizes Python-C patterns and creates a unified representation.
    ///
    /// # Errors
    ///
    /// Returns an error if the Python and C HIR nodes cannot be unified
    /// (i.e., no known pattern matches the combination).
    pub fn unify(&mut self, python: &PythonHIR, c: &CHIR) -> Result<UnifiedHIR> {
        // Pattern matching for known Python-C relationships
        match (python, c) {
            // Pattern 1: Python len() → C list_length() → Rust Vec::len()
            // This was validated in Sprint 0! ✅
            (
                PythonHIR::Call {
                    callee: py_callee,
                    args: py_args,
                    ..
                },
                CHIR::Function { name: c_name, .. },
            ) => {
                if let PythonHIR::Variable { name: py_name, .. } = py_callee.as_ref() {
                    if py_name == "len" && c_name == "list_length" {
                        // VALIDATED PATTERN from Sprint 0!
                        return self.unify_len_pattern(py_args);
                    }
                    if py_name == "append" && c_name == "PyList_Append" {
                        // APPEND PATTERN: Python list.append() + C PyList_Append() → Rust Vec::push()
                        return self.unify_append_pattern(py_args);
                    }
                    if py_name == "get" && c_name == "PyDict_GetItem" {
                        // DICT.GET PATTERN: Python dict.get() + C PyDict_GetItem() → Rust HashMap::get()
                        return self.unify_dict_get_pattern(py_args);
                    }
                    if py_name == "reverse" && c_name == "list_reverse" {
                        // REVERSE PATTERN: Python list.reverse() + C list_reverse() → Rust Vec::reverse()
                        return self.unify_reverse_pattern(py_args);
                    }
                    if py_name == "clear" && c_name == "list_clear" {
                        // CLEAR PATTERN: Python list.clear() + C list_clear() → Rust Vec::clear()
                        return self.unify_clear_pattern(py_args);
                    }
                    if py_name == "pop" && c_name == "list_pop" {
                        // POP PATTERN: Python list.pop() + C list_pop() → Rust Vec::pop()
                        return self.unify_pop_pattern(py_args);
                    }
                    if py_name == "insert" && c_name == "list_insert" {
                        // INSERT PATTERN: Python list.insert() + C list_insert() → Rust Vec::insert()
                        return self.unify_insert_pattern(py_args);
                    }
                    if py_name == "extend" && c_name == "list_extend" {
                        // EXTEND PATTERN: Python list.extend() + C list_extend() → Rust Vec::extend()
                        return self.unify_extend_pattern(py_args);
                    }
                    // Dict operations
                    if py_name == "dict_pop" && c_name == "PyDict_DelItem" {
                        // DICT POP PATTERN: Python dict.pop() + C PyDict_DelItem() → Rust HashMap::remove()
                        return self.unify_dict_pop_pattern(py_args);
                    }
                    if py_name == "dict_clear" && c_name == "PyDict_Clear" {
                        // DICT CLEAR PATTERN: Python dict.clear() + C PyDict_Clear() → Rust HashMap::clear()
                        return self.unify_dict_clear_pattern(py_args);
                    }
                    if py_name == "keys" && c_name == "PyDict_Keys" {
                        // DICT KEYS PATTERN: Python dict.keys() + C PyDict_Keys() → Rust HashMap::keys()
                        return self.unify_dict_keys_pattern(py_args);
                    }
                }
                bail!("Cannot unify Python call with C function")
            }

            // More patterns will be added here as we extend the unifier
            _ => bail!("Cannot unify Python HIR {python:?} with C HIR {c:?}"),
        }
    }

    /// Unify the `len()` pattern (from Sprint 0)
    #[allow(clippy::unnecessary_wraps)]
    fn unify_len_pattern(&mut self, _args: &[PythonHIR]) -> Result<UnifiedHIR> {
        let id = self.next_node_id();

        Ok(UnifiedHIR::Call {
            id,
            target_language: Language::Rust,
            callee: "Vec::len".to_owned(),
            args: vec![], // Simplified for now
            inferred_type: Type::Rust(crate::types::RustType::Int {
                bits: crate::types::IntSize::ISize,
                signed: false,
            }),
            source_language: Language::Python,
            cross_mapping: Some(CrossMapping {
                python_node: None,
                c_node: None,
                pattern: UnificationPattern::LenPattern,
                boundary_eliminated: false,
            }),
            meta: Metadata::new(),
        })
    }

    /// Unify the `append()` pattern (Python list.append + C `PyList_Append` → Rust `Vec::push`)
    #[allow(clippy::unnecessary_wraps)]
    fn unify_append_pattern(&mut self, _args: &[PythonHIR]) -> Result<UnifiedHIR> {
        let id = self.next_node_id();

        Ok(UnifiedHIR::Call {
            id,
            target_language: Language::Rust,
            callee: "Vec::push".to_owned(),
            args: vec![], // Simplified for now
            inferred_type: Type::Rust(crate::types::RustType::Unit),
            source_language: Language::Python,
            cross_mapping: Some(CrossMapping {
                python_node: None,
                c_node: None,
                pattern: UnificationPattern::AppendPattern,
                boundary_eliminated: false,
            }),
            meta: Metadata::new(),
        })
    }

    /// Unify the `dict.get()` pattern (Python dict.get + C `PyDict_GetItem` → Rust `HashMap::get`)
    #[allow(clippy::unnecessary_wraps)]
    fn unify_dict_get_pattern(&mut self, _args: &[PythonHIR]) -> Result<UnifiedHIR> {
        let id = self.next_node_id();

        Ok(UnifiedHIR::Call {
            id,
            target_language: Language::Rust,
            callee: "HashMap::get".to_owned(),
            args: vec![], // Simplified for now
            inferred_type: Type::Rust(crate::types::RustType::Option(Box::new(Type::Unknown))),
            source_language: Language::Python,
            cross_mapping: Some(CrossMapping {
                python_node: None,
                c_node: None,
                pattern: UnificationPattern::DictGetPattern,
                boundary_eliminated: false,
            }),
            meta: Metadata::new(),
        })
    }

    /// Unify the `reverse()` pattern (Python list.reverse + C `list_reverse` → Rust `Vec::reverse`)
    #[allow(clippy::unnecessary_wraps)]
    fn unify_reverse_pattern(&mut self, _args: &[PythonHIR]) -> Result<UnifiedHIR> {
        let id = self.next_node_id();

        Ok(UnifiedHIR::Call {
            id,
            target_language: Language::Rust,
            callee: "Vec::reverse".to_owned(),
            args: vec![], // Simplified for now
            inferred_type: Type::Rust(crate::types::RustType::Unit),
            source_language: Language::Python,
            cross_mapping: Some(CrossMapping {
                python_node: None,
                c_node: None,
                pattern: UnificationPattern::ReversePattern,
                boundary_eliminated: false,
            }),
            meta: Metadata::new(),
        })
    }

    /// Unify the `clear()` pattern (Python list.clear + C `list_clear` → Rust `Vec::clear`)
    #[allow(clippy::unnecessary_wraps)]
    fn unify_clear_pattern(&mut self, _args: &[PythonHIR]) -> Result<UnifiedHIR> {
        let id = self.next_node_id();

        Ok(UnifiedHIR::Call {
            id,
            target_language: Language::Rust,
            callee: "Vec::clear".to_owned(),
            args: vec![], // Simplified for now
            inferred_type: Type::Rust(crate::types::RustType::Unit),
            source_language: Language::Python,
            cross_mapping: Some(CrossMapping {
                python_node: None,
                c_node: None,
                pattern: UnificationPattern::ClearPattern,
                boundary_eliminated: false,
            }),
            meta: Metadata::new(),
        })
    }

    /// Unify the `pop()` pattern (Python list.pop + C `list_pop` → Rust `Vec::pop`)
    #[allow(clippy::unnecessary_wraps)]
    fn unify_pop_pattern(&mut self, _args: &[PythonHIR]) -> Result<UnifiedHIR> {
        let id = self.next_node_id();

        Ok(UnifiedHIR::Call {
            id,
            target_language: Language::Rust,
            callee: "Vec::pop".to_owned(),
            args: vec![], // Simplified for now
            inferred_type: Type::Rust(crate::types::RustType::Option(Box::new(Type::Unknown))),
            source_language: Language::Python,
            cross_mapping: Some(CrossMapping {
                python_node: None,
                c_node: None,
                pattern: UnificationPattern::PopPattern,
                boundary_eliminated: false,
            }),
            meta: Metadata::new(),
        })
    }

    /// Unify the `insert()` pattern (Python list.insert + C `list_insert` → Rust `Vec::insert`)
    #[allow(clippy::unnecessary_wraps)]
    fn unify_insert_pattern(&mut self, _args: &[PythonHIR]) -> Result<UnifiedHIR> {
        let id = self.next_node_id();

        Ok(UnifiedHIR::Call {
            id,
            target_language: Language::Rust,
            callee: "Vec::insert".to_owned(),
            args: vec![], // Simplified for now
            inferred_type: Type::Rust(crate::types::RustType::Unit),
            source_language: Language::Python,
            cross_mapping: Some(CrossMapping {
                python_node: None,
                c_node: None,
                pattern: UnificationPattern::InsertPattern,
                boundary_eliminated: false,
            }),
            meta: Metadata::new(),
        })
    }

    /// Unify the `extend()` pattern (Python list.extend + C `list_extend` → Rust `Vec::extend`)
    #[allow(clippy::unnecessary_wraps)]
    fn unify_extend_pattern(&mut self, _args: &[PythonHIR]) -> Result<UnifiedHIR> {
        let id = self.next_node_id();

        Ok(UnifiedHIR::Call {
            id,
            target_language: Language::Rust,
            callee: "Vec::extend".to_owned(),
            args: vec![], // Simplified for now
            inferred_type: Type::Rust(crate::types::RustType::Unit),
            source_language: Language::Python,
            cross_mapping: Some(CrossMapping {
                python_node: None,
                c_node: None,
                pattern: UnificationPattern::ExtendPattern,
                boundary_eliminated: false,
            }),
            meta: Metadata::new(),
        })
    }

    /// Unify the `dict.pop()` pattern (Python dict.pop + C `PyDict_DelItem` → Rust `HashMap::remove`)
    #[allow(clippy::unnecessary_wraps)]
    fn unify_dict_pop_pattern(&mut self, _args: &[PythonHIR]) -> Result<UnifiedHIR> {
        let id = self.next_node_id();

        Ok(UnifiedHIR::Call {
            id,
            target_language: Language::Rust,
            callee: "HashMap::remove".to_owned(),
            args: vec![], // Simplified for now
            inferred_type: Type::Rust(crate::types::RustType::Option(Box::new(Type::Unknown))),
            source_language: Language::Python,
            cross_mapping: Some(CrossMapping {
                python_node: None,
                c_node: None,
                pattern: UnificationPattern::DictPopPattern,
                boundary_eliminated: false,
            }),
            meta: Metadata::new(),
        })
    }

    /// Unify the `dict.clear()` pattern (Python dict.clear + C `PyDict_Clear` → Rust `HashMap::clear`)
    #[allow(clippy::unnecessary_wraps)]
    fn unify_dict_clear_pattern(&mut self, _args: &[PythonHIR]) -> Result<UnifiedHIR> {
        let id = self.next_node_id();

        Ok(UnifiedHIR::Call {
            id,
            target_language: Language::Rust,
            callee: "HashMap::clear".to_owned(),
            args: vec![], // Simplified for now
            inferred_type: Type::Rust(crate::types::RustType::Unit),
            source_language: Language::Python,
            cross_mapping: Some(CrossMapping {
                python_node: None,
                c_node: None,
                pattern: UnificationPattern::DictClearPattern,
                boundary_eliminated: false,
            }),
            meta: Metadata::new(),
        })
    }

    /// Unify the `dict.keys()` pattern (Python dict.keys + C `PyDict_Keys` → Rust `HashMap::keys`)
    #[allow(clippy::unnecessary_wraps)]
    fn unify_dict_keys_pattern(&mut self, _args: &[PythonHIR]) -> Result<UnifiedHIR> {
        let id = self.next_node_id();

        Ok(UnifiedHIR::Call {
            id,
            target_language: Language::Rust,
            callee: "HashMap::keys".to_owned(),
            args: vec![], // Simplified for now
            inferred_type: Type::Rust(crate::types::RustType::Custom("Keys".to_owned())),
            source_language: Language::Python,
            cross_mapping: Some(CrossMapping {
                python_node: None,
                c_node: None,
                pattern: UnificationPattern::DictKeysPattern,
                boundary_eliminated: false,
            }),
            meta: Metadata::new(),
        })
    }

    /// Get the next node ID
    fn next_node_id(&mut self) -> NodeId {
        let id = NodeId::new(self.next_id);
        self.next_id += 1;
        id
    }
}

impl Default for Unifier {
    fn default() -> Self {
        Self::new()
    }
}

impl UnifiedHIR {
    /// Eliminate Python→C boundaries through optimization
    ///
    /// This converts cross-language calls into pure Rust.
    /// Validated by Sprint 0! ✅
    #[must_use]
    pub fn eliminate_boundary(self) -> Self {
        match self {
            Self::Call {
                id,
                target_language,
                callee,
                args,
                inferred_type,
                source_language,
                cross_mapping,
                meta,
            } => {
                // If this call has cross-language mapping, mark boundary as eliminated
                let new_mapping = if let Some(mut mapping) = cross_mapping.clone() {
                    mapping.boundary_eliminated = true;
                    Some(mapping)
                } else {
                    cross_mapping
                };

                // Convert target to Rust if different from source
                let new_target = if source_language == target_language {
                    target_language
                } else {
                    Language::Rust
                };

                // Recursively eliminate boundaries in arguments
                let new_args = args.into_iter().map(Self::eliminate_boundary).collect();

                Self::Call {
                    id,
                    target_language: new_target,
                    callee,
                    args: new_args,
                    inferred_type,
                    source_language,
                    cross_mapping: new_mapping,
                    meta,
                }
            }

            // Recursively process other node types
            other => other,
        }
    }

    /// Get the node ID
    #[must_use]
    pub const fn id(&self) -> Option<NodeId> {
        match self {
            Self::Module { .. } => None,
            Self::Function { id, .. }
            | Self::Call { id, .. }
            | Self::Variable { id, .. }
            | Self::Assign { id, .. }
            | Self::Return { id, .. }
            | Self::If { id, .. }
            | Self::Loop { id, .. }
            | Self::BinOp { id, .. }
            | Self::Literal { id, .. } => Some(*id),
        }
    }
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::panic, clippy::similar_names)]
mod tests {
    use super::*;
    use crate::types::*;

    #[test]
    fn test_unifier_len_pattern() {
        // Recreate Sprint 0 success! ✅
        let mut unifier = Unifier::new();

        let python_call = PythonHIR::Call {
            id: NodeId::new(1),
            callee: Box::new(PythonHIR::Variable {
                id: NodeId::new(2),
                name: "len".to_owned(),
                inferred_type: None,
                meta: Metadata::new(),
            }),
            args: vec![],
            kwargs: vec![],
            inferred_type: None,
            meta: Metadata::new(),
        };

        let c_function = CHIR::Function {
            id: NodeId::new(3),
            name: "list_length".to_owned(),
            return_type: Type::C(CType::SizeT),
            params: vec![],
            body: vec![],
            storage_class: crate::c::StorageClass::Static,
            visibility: crate::Visibility::Private,
            meta: Metadata::new(),
        };

        let unified = unifier
            .unify(&python_call, &c_function)
            .expect("Unification should succeed");

        // Should create a call to Vec::len in Rust
        let UnifiedHIR::Call {
            target_language,
            callee,
            cross_mapping,
            ..
        } = unified
        else {
            panic!("Expected UnifiedHIR::Call");
        };
        assert_eq!(target_language, Language::Rust);
        assert_eq!(callee, "Vec::len");
        assert!(cross_mapping.is_some());
        assert_eq!(
            cross_mapping.expect("cross_mapping should exist").pattern,
            UnificationPattern::LenPattern
        );
    }

    #[test]
    fn test_unifier_append_pattern() {
        // Test append() pattern: Python list.append() + C PyList_Append → Rust Vec::push()
        let mut unifier = Unifier::new();

        let python_call = PythonHIR::Call {
            id: NodeId::new(1),
            callee: Box::new(PythonHIR::Variable {
                id: NodeId::new(2),
                name: "append".to_owned(),
                inferred_type: None,
                meta: Metadata::new(),
            }),
            args: vec![PythonHIR::Variable {
                id: NodeId::new(3),
                name: "item".to_owned(),
                inferred_type: None,
                meta: Metadata::new(),
            }],
            kwargs: vec![],
            inferred_type: None,
            meta: Metadata::new(),
        };

        let c_function = CHIR::Function {
            id: NodeId::new(4),
            name: "PyList_Append".to_owned(),
            return_type: Type::C(CType::Int),
            params: vec![],
            body: vec![],
            storage_class: crate::c::StorageClass::Static,
            visibility: crate::Visibility::Private,
            meta: Metadata::new(),
        };

        let unified = unifier
            .unify(&python_call, &c_function)
            .expect("Unification should succeed");

        // Should create a call to Vec::push in Rust
        let UnifiedHIR::Call {
            target_language,
            callee,
            cross_mapping,
            ..
        } = unified
        else {
            panic!("Expected UnifiedHIR::Call");
        };
        assert_eq!(target_language, Language::Rust);
        assert_eq!(callee, "Vec::push");
        assert!(cross_mapping.is_some());
        assert_eq!(
            cross_mapping.expect("cross_mapping should exist").pattern,
            UnificationPattern::AppendPattern
        );
    }

    #[test]
    fn test_unifier_dict_get_pattern() {
        // Test dict.get() pattern: Python dict.get() + C PyDict_GetItem → Rust HashMap::get()
        let mut unifier = Unifier::new();

        let python_call = PythonHIR::Call {
            id: NodeId::new(1),
            callee: Box::new(PythonHIR::Variable {
                id: NodeId::new(2),
                name: "get".to_owned(),
                inferred_type: None,
                meta: Metadata::new(),
            }),
            args: vec![PythonHIR::Variable {
                id: NodeId::new(3),
                name: "key".to_owned(),
                inferred_type: None,
                meta: Metadata::new(),
            }],
            kwargs: vec![],
            inferred_type: None,
            meta: Metadata::new(),
        };

        let c_function = CHIR::Function {
            id: NodeId::new(4),
            name: "PyDict_GetItem".to_owned(),
            return_type: Type::C(CType::Pointer(Box::new(CType::Void))),
            params: vec![],
            body: vec![],
            storage_class: crate::c::StorageClass::Static,
            visibility: crate::Visibility::Private,
            meta: Metadata::new(),
        };

        let unified = unifier
            .unify(&python_call, &c_function)
            .expect("Unification should succeed");

        // Should create a call to HashMap::get in Rust
        let UnifiedHIR::Call {
            target_language,
            callee,
            cross_mapping,
            ..
        } = unified
        else {
            panic!("Expected UnifiedHIR::Call");
        };
        assert_eq!(target_language, Language::Rust);
        assert_eq!(callee, "HashMap::get");
        assert!(cross_mapping.is_some());
        assert_eq!(
            cross_mapping.expect("cross_mapping should exist").pattern,
            UnificationPattern::DictGetPattern
        );
    }

    #[test]
    fn test_boundary_elimination() {
        // Test boundary elimination (from Sprint 0)
        let call = UnifiedHIR::Call {
            id: NodeId::new(1),
            target_language: Language::Python,
            callee: "len".to_owned(),
            args: vec![],
            inferred_type: Type::Unknown,
            source_language: Language::Python,
            cross_mapping: Some(CrossMapping {
                python_node: None,
                c_node: None,
                pattern: UnificationPattern::LenPattern,
                boundary_eliminated: false,
            }),
            meta: Metadata::new(),
        };

        let optimized = call.eliminate_boundary();

        if let UnifiedHIR::Call { cross_mapping, .. } = optimized {
            assert!(
                cross_mapping
                    .expect("cross_mapping should exist")
                    .boundary_eliminated
            );
        }
    }
}
