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

use crate::{
    c::CHIR,
    error::{extract_c_fn_name, extract_python_fn_name, find_similar_patterns, UnificationError},
    metadata::Metadata,
    python::PythonHIR,
    types::Type,
    Language, NodeId,
};
use anyhow::Result;
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
    /// Python `x in list` → C `list_contains()` → Rust `Vec::contains()`
    ListContainsPattern,
    /// Python `x in dict` → C `dict_contains()` → Rust `HashMap::contains_key()`
    DictContainsPattern,
    /// Python `dict.values()` → C `PyDict_Values()` → Rust `HashMap::values()`
    DictValuesPattern,
    /// Python `list.count(x)` → C `list_count()` → Rust `Vec::iter().filter().count()`
    ListCountPattern,
    /// Python `list.index(x)` → C `list_index()` → Rust `Vec::iter().position()`
    ListIndexPattern,
    /// Python `dict.items()` → C `PyDict_Items()` → Rust `HashMap::iter()`
    DictItemsPattern,
    /// Python `list.remove(x)` → C `list_remove()` → Rust `Vec::retain()`
    ListRemovePattern,
    /// Python `list.sort()` → C `list_sort()` → Rust `Vec::sort()`
    ListSortPattern,
    /// Python `list.copy()` → C `list_copy()` → Rust `Vec::clone()`
    ListCopyPattern,
    /// Python `dict.copy()` → C `dict_copy()` → Rust `HashMap::clone()`
    DictCopyPattern,
    /// Python `dict.setdefault()` → C `dict_setdefault()` → Rust `HashMap::entry().or_insert()`
    DictSetDefaultPattern,
    /// Python `dict.update()` → C `dict_update()` → Rust `HashMap::extend()`
    DictUpdatePattern,
    /// Python `min(list)` → C `list_min()` → Rust `Vec::iter().min()`
    ListMinPattern,
    /// Python `max(list)` → C `list_max()` → Rust `Vec::iter().max()`
    ListMaxPattern,
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
    #[allow(clippy::too_many_lines)]
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
                    if py_name == "contains" && c_name == "list_contains" {
                        // LIST CONTAINS PATTERN: Python 'x in list' + C list_contains() → Rust Vec::contains()
                        return self.unify_list_contains_pattern(py_args);
                    }
                    if py_name == "dict_contains" && c_name == "dict_contains" {
                        // DICT CONTAINS PATTERN: Python 'x in dict' + C dict_contains() → Rust HashMap::contains_key()
                        return self.unify_dict_contains_pattern(py_args);
                    }
                    if py_name == "dict_values" && c_name == "PyDict_Values" {
                        // DICT VALUES PATTERN: Python dict.values() + C PyDict_Values() → Rust HashMap::values()
                        return self.unify_dict_values_pattern(py_args);
                    }
                    if py_name == "count" && c_name == "list_count" {
                        // LIST COUNT PATTERN: Python list.count(x) + C list_count() → Rust Vec::iter().filter().count()
                        return self.unify_list_count_pattern(py_args);
                    }
                    if py_name == "index" && c_name == "list_index" {
                        // LIST INDEX PATTERN: Python list.index(x) + C list_index() → Rust Vec::iter().position()
                        return self.unify_list_index_pattern(py_args);
                    }
                    if py_name == "dict_items" && c_name == "PyDict_Items" {
                        // DICT ITEMS PATTERN: Python dict.items() + C PyDict_Items() → Rust HashMap::iter()
                        return self.unify_dict_items_pattern(py_args);
                    }
                    if py_name == "remove" && c_name == "list_remove" {
                        // LIST REMOVE PATTERN: Python list.remove(x) + C list_remove() → Rust Vec::retain()
                        return self.unify_list_remove_pattern(py_args);
                    }
                    if py_name == "sort" && c_name == "list_sort" {
                        // LIST SORT PATTERN: Python list.sort() + C list_sort() → Rust Vec::sort()
                        return self.unify_list_sort_pattern(py_args);
                    }
                    if py_name == "copy" && c_name == "list_copy" {
                        // LIST COPY PATTERN: Python list.copy() + C list_copy() → Rust Vec::clone()
                        return self.unify_list_copy_pattern(py_args);
                    }
                    if py_name == "dict_copy" && c_name == "dict_copy" {
                        // DICT COPY PATTERN: Python dict.copy() + C dict_copy() → Rust HashMap::clone()
                        return self.unify_dict_copy_pattern(py_args);
                    }
                    if py_name == "setdefault" && c_name == "dict_setdefault" {
                        // DICT SETDEFAULT PATTERN: Python dict.setdefault() + C dict_setdefault() → Rust HashMap::entry().or_insert()
                        return self.unify_dict_setdefault_pattern(py_args);
                    }
                    if py_name == "update" && c_name == "dict_update" {
                        // DICT UPDATE PATTERN: Python dict.update() + C dict_update() → Rust HashMap::extend()
                        return self.unify_dict_update_pattern(py_args);
                    }
                    if py_name == "min_value" && c_name == "list_min" {
                        // LIST MIN PATTERN: Python min(list) + C list_min() → Rust Vec::iter().min()
                        return self.unify_list_min_pattern(py_args);
                    }
                    if py_name == "max_value" && c_name == "list_max" {
                        // LIST MAX PATTERN: Python max(list) + C list_max() → Rust Vec::iter().max()
                        return self.unify_list_max_pattern(py_args);
                    }
                }

                // No pattern found - generate helpful error message
                let python_fn = if let PythonHIR::Variable { name, .. } = py_callee.as_ref() {
                    name.clone()
                } else {
                    extract_python_fn_name(python)
                };
                let c_fn = c_name.clone();
                let suggestions = find_similar_patterns(&python_fn, &c_fn);

                Err(UnificationError::NoPatternMatch {
                    python_fn,
                    c_fn,
                    suggestions,
                })?
            }

            // Incompatible node types
            _ => {
                let python_kind = extract_python_fn_name(python);
                let c_kind = extract_c_fn_name(c);

                Err(UnificationError::IncompatibleNodes {
                    python_kind,
                    c_kind,
                })?
            }
        }
    }

    /// Unify the `len()` pattern (from Sprint 0)
    #[allow(clippy::unnecessary_wraps)]
    fn unify_len_pattern(&mut self, args: &[PythonHIR]) -> Result<UnifiedHIR> {
        let id = self.next_node_id();

        Ok(UnifiedHIR::Call {
            id,
            target_language: Language::Rust,
            callee: "Vec::len".to_owned(),
            args: self.convert_args(args), // Phase 2.1: Real arguments!
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
    fn unify_append_pattern(&mut self, args: &[PythonHIR]) -> Result<UnifiedHIR> {
        let id = self.next_node_id();

        Ok(UnifiedHIR::Call {
            id,
            target_language: Language::Rust,
            callee: "Vec::push".to_owned(),
            args: self.convert_args(args),
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
    fn unify_dict_get_pattern(&mut self, args: &[PythonHIR]) -> Result<UnifiedHIR> {
        let id = self.next_node_id();

        Ok(UnifiedHIR::Call {
            id,
            target_language: Language::Rust,
            callee: "HashMap::get".to_owned(),
            args: self.convert_args(args),
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
    fn unify_reverse_pattern(&mut self, args: &[PythonHIR]) -> Result<UnifiedHIR> {
        let id = self.next_node_id();

        Ok(UnifiedHIR::Call {
            id,
            target_language: Language::Rust,
            callee: "Vec::reverse".to_owned(),
            args: self.convert_args(args),
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
    fn unify_clear_pattern(&mut self, args: &[PythonHIR]) -> Result<UnifiedHIR> {
        let id = self.next_node_id();

        Ok(UnifiedHIR::Call {
            id,
            target_language: Language::Rust,
            callee: "Vec::clear".to_owned(),
            args: self.convert_args(args),
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
    fn unify_pop_pattern(&mut self, args: &[PythonHIR]) -> Result<UnifiedHIR> {
        let id = self.next_node_id();

        Ok(UnifiedHIR::Call {
            id,
            target_language: Language::Rust,
            callee: "Vec::pop".to_owned(),
            args: self.convert_args(args),
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
    fn unify_insert_pattern(&mut self, args: &[PythonHIR]) -> Result<UnifiedHIR> {
        let id = self.next_node_id();

        Ok(UnifiedHIR::Call {
            id,
            target_language: Language::Rust,
            callee: "Vec::insert".to_owned(),
            args: self.convert_args(args),
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
    fn unify_extend_pattern(&mut self, args: &[PythonHIR]) -> Result<UnifiedHIR> {
        let id = self.next_node_id();

        Ok(UnifiedHIR::Call {
            id,
            target_language: Language::Rust,
            callee: "Vec::extend".to_owned(),
            args: self.convert_args(args),
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
    fn unify_dict_pop_pattern(&mut self, args: &[PythonHIR]) -> Result<UnifiedHIR> {
        let id = self.next_node_id();

        Ok(UnifiedHIR::Call {
            id,
            target_language: Language::Rust,
            callee: "HashMap::remove".to_owned(),
            args: self.convert_args(args),
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
    fn unify_dict_clear_pattern(&mut self, args: &[PythonHIR]) -> Result<UnifiedHIR> {
        let id = self.next_node_id();

        Ok(UnifiedHIR::Call {
            id,
            target_language: Language::Rust,
            callee: "HashMap::clear".to_owned(),
            args: self.convert_args(args),
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
    fn unify_dict_keys_pattern(&mut self, args: &[PythonHIR]) -> Result<UnifiedHIR> {
        let id = self.next_node_id();

        Ok(UnifiedHIR::Call {
            id,
            target_language: Language::Rust,
            callee: "HashMap::keys".to_owned(),
            args: self.convert_args(args),
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

    /// Unify the `list.contains()` pattern (Python 'x in list' + C `list_contains` → Rust `Vec::contains`)
    #[allow(clippy::unnecessary_wraps)]
    fn unify_list_contains_pattern(&mut self, args: &[PythonHIR]) -> Result<UnifiedHIR> {
        let id = self.next_node_id();

        Ok(UnifiedHIR::Call {
            id,
            target_language: Language::Rust,
            callee: "Vec::contains".to_owned(),
            args: self.convert_args(args),
            inferred_type: Type::Rust(crate::types::RustType::Bool),
            source_language: Language::Python,
            cross_mapping: Some(CrossMapping {
                python_node: None,
                c_node: None,
                pattern: UnificationPattern::ListContainsPattern,
                boundary_eliminated: false,
            }),
            meta: Metadata::new(),
        })
    }

    /// Unify the `dict.contains_key()` pattern (Python 'x in dict' + C `dict_contains` → Rust `HashMap::contains_key`)
    #[allow(clippy::unnecessary_wraps)]
    fn unify_dict_contains_pattern(&mut self, args: &[PythonHIR]) -> Result<UnifiedHIR> {
        let id = self.next_node_id();

        Ok(UnifiedHIR::Call {
            id,
            target_language: Language::Rust,
            callee: "HashMap::contains_key".to_owned(),
            args: self.convert_args(args),
            inferred_type: Type::Rust(crate::types::RustType::Bool),
            source_language: Language::Python,
            cross_mapping: Some(CrossMapping {
                python_node: None,
                c_node: None,
                pattern: UnificationPattern::DictContainsPattern,
                boundary_eliminated: false,
            }),
            meta: Metadata::new(),
        })
    }

    /// Unify the `dict.values()` pattern (Python dict.values + C `PyDict_Values` → Rust `HashMap::values`)
    #[allow(clippy::unnecessary_wraps)]
    fn unify_dict_values_pattern(&mut self, args: &[PythonHIR]) -> Result<UnifiedHIR> {
        let id = self.next_node_id();

        Ok(UnifiedHIR::Call {
            id,
            target_language: Language::Rust,
            callee: "HashMap::values".to_owned(),
            args: self.convert_args(args),
            inferred_type: Type::Rust(crate::types::RustType::Custom("Values".to_owned())),
            source_language: Language::Python,
            cross_mapping: Some(CrossMapping {
                python_node: None,
                c_node: None,
                pattern: UnificationPattern::DictValuesPattern,
                boundary_eliminated: false,
            }),
            meta: Metadata::new(),
        })
    }

    /// Unify the `list.count()` pattern (Python list.count + C `list_count` → Rust `Vec::iter().filter().count`)
    #[allow(clippy::unnecessary_wraps)]
    fn unify_list_count_pattern(&mut self, args: &[PythonHIR]) -> Result<UnifiedHIR> {
        let id = self.next_node_id();

        Ok(UnifiedHIR::Call {
            id,
            target_language: Language::Rust,
            callee: "Vec::count".to_owned(),
            args: self.convert_args(args),
            inferred_type: Type::Rust(crate::types::RustType::Int {
                bits: crate::types::IntSize::ISize,
                signed: false,
            }),
            source_language: Language::Python,
            cross_mapping: Some(CrossMapping {
                python_node: None,
                c_node: None,
                pattern: UnificationPattern::ListCountPattern,
                boundary_eliminated: false,
            }),
            meta: Metadata::new(),
        })
    }

    /// Unify the `list.index()` pattern (Python list.index + C `list_index` → Rust `Vec::iter().position`)
    #[allow(clippy::unnecessary_wraps)]
    fn unify_list_index_pattern(&mut self, args: &[PythonHIR]) -> Result<UnifiedHIR> {
        let id = self.next_node_id();

        Ok(UnifiedHIR::Call {
            id,
            target_language: Language::Rust,
            callee: "Vec::position".to_owned(),
            args: self.convert_args(args),
            inferred_type: Type::Rust(crate::types::RustType::Option(Box::new(Type::Rust(
                crate::types::RustType::Int {
                    bits: crate::types::IntSize::ISize,
                    signed: false,
                },
            )))),
            source_language: Language::Python,
            cross_mapping: Some(CrossMapping {
                python_node: None,
                c_node: None,
                pattern: UnificationPattern::ListIndexPattern,
                boundary_eliminated: false,
            }),
            meta: Metadata::new(),
        })
    }

    /// Unify the `dict.items()` pattern (Python dict.items + C `PyDict_Items` → Rust `HashMap::iter`)
    #[allow(clippy::unnecessary_wraps)]
    fn unify_dict_items_pattern(&mut self, args: &[PythonHIR]) -> Result<UnifiedHIR> {
        let id = self.next_node_id();

        Ok(UnifiedHIR::Call {
            id,
            target_language: Language::Rust,
            callee: "HashMap::iter".to_owned(),
            args: self.convert_args(args),
            inferred_type: Type::Rust(crate::types::RustType::Custom("Iter".to_owned())),
            source_language: Language::Python,
            cross_mapping: Some(CrossMapping {
                python_node: None,
                c_node: None,
                pattern: UnificationPattern::DictItemsPattern,
                boundary_eliminated: false,
            }),
            meta: Metadata::new(),
        })
    }

    /// Unify the `list.remove()` pattern (Python list.remove + C `list_remove` → Rust `Vec::retain`)
    #[allow(clippy::unnecessary_wraps)]
    fn unify_list_remove_pattern(&mut self, args: &[PythonHIR]) -> Result<UnifiedHIR> {
        let id = self.next_node_id();

        Ok(UnifiedHIR::Call {
            id,
            target_language: Language::Rust,
            callee: "Vec::retain".to_owned(),
            args: self.convert_args(args),
            inferred_type: Type::Rust(crate::types::RustType::Unit),
            source_language: Language::Python,
            cross_mapping: Some(CrossMapping {
                python_node: None,
                c_node: None,
                pattern: UnificationPattern::ListRemovePattern,
                boundary_eliminated: false,
            }),
            meta: Metadata::new(),
        })
    }

    /// Unify the `list.sort()` pattern (Python list.sort + C `list_sort` → Rust `Vec::sort`)
    #[allow(clippy::unnecessary_wraps)]
    fn unify_list_sort_pattern(&mut self, args: &[PythonHIR]) -> Result<UnifiedHIR> {
        let id = self.next_node_id();

        Ok(UnifiedHIR::Call {
            id,
            target_language: Language::Rust,
            callee: "Vec::sort".to_owned(),
            args: self.convert_args(args),
            inferred_type: Type::Rust(crate::types::RustType::Unit),
            source_language: Language::Python,
            cross_mapping: Some(CrossMapping {
                python_node: None,
                c_node: None,
                pattern: UnificationPattern::ListSortPattern,
                boundary_eliminated: false,
            }),
            meta: Metadata::new(),
        })
    }

    /// Unify the `list.copy()` pattern (Python list.copy + C `list_copy` → Rust `Vec::clone`)
    #[allow(clippy::unnecessary_wraps)]
    fn unify_list_copy_pattern(&mut self, args: &[PythonHIR]) -> Result<UnifiedHIR> {
        let id = self.next_node_id();

        Ok(UnifiedHIR::Call {
            id,
            target_language: Language::Rust,
            callee: "Vec::clone".to_owned(),
            args: self.convert_args(args),
            inferred_type: Type::Rust(crate::types::RustType::Vec(Box::new(Type::Unknown))),
            source_language: Language::Python,
            cross_mapping: Some(CrossMapping {
                python_node: None,
                c_node: None,
                pattern: UnificationPattern::ListCopyPattern,
                boundary_eliminated: false,
            }),
            meta: Metadata::new(),
        })
    }

    /// Unify the `dict.copy()` pattern (Python dict.copy + C `dict_copy` → Rust `HashMap::clone`)
    #[allow(clippy::unnecessary_wraps)]
    fn unify_dict_copy_pattern(&mut self, args: &[PythonHIR]) -> Result<UnifiedHIR> {
        let id = self.next_node_id();

        Ok(UnifiedHIR::Call {
            id,
            target_language: Language::Rust,
            callee: "HashMap::clone".to_owned(),
            args: self.convert_args(args),
            inferred_type: Type::Rust(crate::types::RustType::HashMap {
                key: Box::new(Type::Unknown),
                value: Box::new(Type::Unknown),
            }),
            source_language: Language::Python,
            cross_mapping: Some(CrossMapping {
                python_node: None,
                c_node: None,
                pattern: UnificationPattern::DictCopyPattern,
                boundary_eliminated: false,
            }),
            meta: Metadata::new(),
        })
    }

    /// Unify the `dict.setdefault()` pattern (Python dict.setdefault + C `dict_setdefault` → Rust `HashMap::entry().or_insert()`)
    #[allow(clippy::unnecessary_wraps)]
    fn unify_dict_setdefault_pattern(&mut self, args: &[PythonHIR]) -> Result<UnifiedHIR> {
        let id = self.next_node_id();

        Ok(UnifiedHIR::Call {
            id,
            target_language: Language::Rust,
            callee: "HashMap::entry_or_insert".to_owned(),
            args: self.convert_args(args),
            inferred_type: Type::Rust(crate::types::RustType::Reference {
                mutable: true,
                inner: Box::new(Type::Unknown),
            }),
            source_language: Language::Python,
            cross_mapping: Some(CrossMapping {
                python_node: None,
                c_node: None,
                pattern: UnificationPattern::DictSetDefaultPattern,
                boundary_eliminated: false,
            }),
            meta: Metadata::new(),
        })
    }

    /// Unify the `dict.update()` pattern (Python dict.update + C `dict_update` → Rust `HashMap::extend`)
    #[allow(clippy::unnecessary_wraps)]
    fn unify_dict_update_pattern(&mut self, args: &[PythonHIR]) -> Result<UnifiedHIR> {
        let id = self.next_node_id();

        Ok(UnifiedHIR::Call {
            id,
            target_language: Language::Rust,
            callee: "HashMap::extend".to_owned(),
            args: self.convert_args(args),
            inferred_type: Type::Rust(crate::types::RustType::Unit),
            source_language: Language::Python,
            cross_mapping: Some(CrossMapping {
                python_node: None,
                c_node: None,
                pattern: UnificationPattern::DictUpdatePattern,
                boundary_eliminated: false,
            }),
            meta: Metadata::new(),
        })
    }

    /// Unify the `list.min()` pattern (Python min(list) + C `list_min` → Rust `Vec::iter().min()`)
    #[allow(clippy::unnecessary_wraps)]
    fn unify_list_min_pattern(&mut self, args: &[PythonHIR]) -> Result<UnifiedHIR> {
        let id = self.next_node_id();

        Ok(UnifiedHIR::Call {
            id,
            target_language: Language::Rust,
            callee: "Vec::min".to_owned(),
            args: self.convert_args(args),
            inferred_type: Type::Rust(crate::types::RustType::Option(Box::new(Type::Unknown))),
            source_language: Language::Python,
            cross_mapping: Some(CrossMapping {
                python_node: None,
                c_node: None,
                pattern: UnificationPattern::ListMinPattern,
                boundary_eliminated: false,
            }),
            meta: Metadata::new(),
        })
    }

    /// Unify the `list.max()` pattern (Python max(list) + C `list_max` → Rust `Vec::iter().max()`)
    #[allow(clippy::unnecessary_wraps)]
    fn unify_list_max_pattern(&mut self, args: &[PythonHIR]) -> Result<UnifiedHIR> {
        let id = self.next_node_id();

        Ok(UnifiedHIR::Call {
            id,
            target_language: Language::Rust,
            callee: "Vec::max".to_owned(),
            args: self.convert_args(args),
            inferred_type: Type::Rust(crate::types::RustType::Option(Box::new(Type::Unknown))),
            source_language: Language::Python,
            cross_mapping: Some(CrossMapping {
                python_node: None,
                c_node: None,
                pattern: UnificationPattern::ListMaxPattern,
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

    /// Convert Python HIR arguments to Unified HIR
    /// This is a simplified conversion for Phase 2.1
    fn convert_args(&mut self, args: &[PythonHIR]) -> Vec<UnifiedHIR> {
        args.iter()
            .filter_map(|arg| self.convert_python_node(arg).ok())
            .collect()
    }

    /// Convert a single Python HIR node to Unified HIR
    /// Simplified for Phase 2.1 - handles common cases
    #[allow(clippy::unnecessary_wraps)]
    fn convert_python_node(&mut self, node: &PythonHIR) -> Result<UnifiedHIR> {
        match node {
            PythonHIR::Variable { name, .. } => {
                let id = self.next_node_id();
                Ok(UnifiedHIR::Variable {
                    id,
                    name: name.clone(),
                    var_type: Type::Unknown, // Type inference to be added later
                    source_language: Language::Python,
                    meta: Metadata::new(),
                })
            }
            // For now, just handle variables - expand later
            _ => Ok(UnifiedHIR::Variable {
                id: self.next_node_id(),
                name: "arg".to_owned(),
                var_type: Type::Unknown,
                source_language: Language::Python,
                meta: Metadata::new(),
            }),
        }
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
