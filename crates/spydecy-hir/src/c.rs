//! C-specific HIR nodes
//!
//! This module defines HIR nodes for C constructs, with special support
//! for `CPython` API patterns.

use crate::{metadata::Metadata, types::Type, NodeId, Visibility};
use serde::{Deserialize, Serialize};

/// C HIR node
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CHIR {
    /// Translation unit (file)
    TranslationUnit {
        /// File name
        name: String,
        /// Declarations
        declarations: Vec<CHIR>,
        /// Metadata
        meta: Metadata,
    },

    /// Function definition
    Function {
        /// Node ID
        id: NodeId,
        /// Function name
        name: String,
        /// Return type
        return_type: Type,
        /// Parameters
        params: Vec<Parameter>,
        /// Function body
        body: Vec<CHIR>,
        /// Storage class (static, extern, etc.)
        storage_class: StorageClass,
        /// Visibility
        visibility: Visibility,
        /// Metadata
        meta: Metadata,
    },

    /// Struct definition
    Struct {
        /// Node ID
        id: NodeId,
        /// Struct name
        name: String,
        /// Fields
        fields: Vec<Field>,
        /// Metadata
        meta: Metadata,
    },

    /// Function call
    Call {
        /// Node ID
        id: NodeId,
        /// Function being called
        callee: Box<CHIR>,
        /// Arguments
        args: Vec<CHIR>,
        /// Inferred type
        inferred_type: Option<Type>,
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
        var_type: Option<Type>,
        /// Metadata
        meta: Metadata,
    },

    /// Variable declaration
    VarDecl {
        /// Node ID
        id: NodeId,
        /// Variable name
        name: String,
        /// Variable type
        var_type: Type,
        /// Initializer
        init: Option<Box<CHIR>>,
        /// Storage class
        storage_class: StorageClass,
        /// Metadata
        meta: Metadata,
    },

    /// Assignment
    Assign {
        /// Node ID
        id: NodeId,
        /// Left-hand side
        lhs: Box<CHIR>,
        /// Right-hand side
        rhs: Box<CHIR>,
        /// Metadata
        meta: Metadata,
    },

    /// Return statement
    Return {
        /// Node ID
        id: NodeId,
        /// Return value
        value: Option<Box<CHIR>>,
        /// Metadata
        meta: Metadata,
    },

    /// If statement
    If {
        /// Node ID
        id: NodeId,
        /// Condition
        condition: Box<CHIR>,
        /// Then branch
        then_branch: Vec<CHIR>,
        /// Else branch
        else_branch: Vec<CHIR>,
        /// Metadata
        meta: Metadata,
    },

    /// For loop
    For {
        /// Node ID
        id: NodeId,
        /// Initialization
        init: Option<Box<CHIR>>,
        /// Condition
        condition: Option<Box<CHIR>>,
        /// Increment
        increment: Option<Box<CHIR>>,
        /// Loop body
        body: Vec<CHIR>,
        /// Metadata
        meta: Metadata,
    },

    /// While loop
    While {
        /// Node ID
        id: NodeId,
        /// Condition
        condition: Box<CHIR>,
        /// Loop body
        body: Vec<CHIR>,
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
        left: Box<CHIR>,
        /// Right operand
        right: Box<CHIR>,
        /// Inferred type
        inferred_type: Option<Type>,
        /// Metadata
        meta: Metadata,
    },

    /// Unary operation
    UnaryOp {
        /// Node ID
        id: NodeId,
        /// Operator
        op: UnaryOp,
        /// Operand
        operand: Box<CHIR>,
        /// Inferred type
        inferred_type: Option<Type>,
        /// Metadata
        meta: Metadata,
    },

    /// Literal value
    Literal {
        /// Node ID
        id: NodeId,
        /// Literal value
        value: Literal,
        /// Metadata
        meta: Metadata,
    },

    /// Field access (obj.field or obj->field)
    FieldAccess {
        /// Node ID
        id: NodeId,
        /// Object
        object: Box<CHIR>,
        /// Field name
        field: String,
        /// Pointer access (->)
        is_pointer: bool,
        /// Inferred type
        inferred_type: Option<Type>,
        /// Metadata
        meta: Metadata,
    },

    /// Array subscript (arr[index])
    ArraySubscript {
        /// Node ID
        id: NodeId,
        /// Array
        array: Box<CHIR>,
        /// Index
        index: Box<CHIR>,
        /// Inferred type
        inferred_type: Option<Type>,
        /// Metadata
        meta: Metadata,
    },

    /// Cast
    Cast {
        /// Node ID
        id: NodeId,
        /// Target type
        target_type: Type,
        /// Expression being cast
        expr: Box<CHIR>,
        /// Metadata
        meta: Metadata,
    },

    /// Pointer dereference (*ptr)
    Deref {
        /// Node ID
        id: NodeId,
        /// Pointer
        pointer: Box<CHIR>,
        /// Inferred type
        inferred_type: Option<Type>,
        /// Metadata
        meta: Metadata,
    },

    /// Address-of (&var)
    AddrOf {
        /// Node ID
        id: NodeId,
        /// Variable
        var: Box<CHIR>,
        /// Metadata
        meta: Metadata,
    },

    /// `CPython` API macro (e.g., `Py_SIZE`, `PyList_GET_SIZE`)
    CPythonMacro {
        /// Node ID
        id: NodeId,
        /// Macro name
        name: String,
        /// Arguments
        args: Vec<CHIR>,
        /// Inferred type
        inferred_type: Option<Type>,
        /// Metadata
        meta: Metadata,
    },
}

/// Function parameter
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Parameter {
    /// Parameter name
    pub name: String,
    /// Parameter type
    pub param_type: Type,
}

/// Struct field
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Field {
    /// Field name
    pub name: String,
    /// Field type
    pub field_type: Type,
}

/// Storage class
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StorageClass {
    /// No storage class
    None,
    /// static
    Static,
    /// extern
    Extern,
    /// typedef
    Typedef,
    /// auto
    Auto,
    /// register
    Register,
}

/// Binary operator
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BinOp {
    /// +
    Add,
    /// -
    Sub,
    /// *
    Mul,
    /// /
    Div,
    /// %
    Mod,
    /// ==
    Eq,
    /// !=
    Ne,
    /// <
    Lt,
    /// <=
    Le,
    /// >
    Gt,
    /// >=
    Ge,
    /// &&
    And,
    /// ||
    Or,
    /// &
    BitAnd,
    /// |
    BitOr,
    /// ^
    BitXor,
    /// <<
    Shl,
    /// >>
    Shr,
}

/// Unary operator
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnaryOp {
    /// !
    Not,
    /// -
    Neg,
    /// +
    Pos,
    /// ~
    BitNot,
}

/// Literal value
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Literal {
    /// Integer
    Int(i64),
    /// Unsigned integer
    UInt(u64),
    /// Float
    Float(f64),
    /// String
    Str(String),
    /// Character
    Char(char),
    /// NULL
    Null,
}

impl CHIR {
    /// Get the node ID if present
    #[must_use]
    pub const fn id(&self) -> Option<NodeId> {
        match self {
            Self::TranslationUnit { .. } => None,
            Self::Function { id, .. }
            | Self::Struct { id, .. }
            | Self::Call { id, .. }
            | Self::Variable { id, .. }
            | Self::VarDecl { id, .. }
            | Self::Assign { id, .. }
            | Self::Return { id, .. }
            | Self::If { id, .. }
            | Self::For { id, .. }
            | Self::While { id, .. }
            | Self::BinOp { id, .. }
            | Self::UnaryOp { id, .. }
            | Self::Literal { id, .. }
            | Self::FieldAccess { id, .. }
            | Self::ArraySubscript { id, .. }
            | Self::Cast { id, .. }
            | Self::Deref { id, .. }
            | Self::AddrOf { id, .. }
            | Self::CPythonMacro { id, .. } => Some(*id),
        }
    }

    /// Get the metadata
    #[must_use]
    pub const fn metadata(&self) -> &Metadata {
        match self {
            Self::TranslationUnit { meta, .. }
            | Self::Function { meta, .. }
            | Self::Struct { meta, .. }
            | Self::Call { meta, .. }
            | Self::Variable { meta, .. }
            | Self::VarDecl { meta, .. }
            | Self::Assign { meta, .. }
            | Self::Return { meta, .. }
            | Self::If { meta, .. }
            | Self::For { meta, .. }
            | Self::While { meta, .. }
            | Self::BinOp { meta, .. }
            | Self::UnaryOp { meta, .. }
            | Self::Literal { meta, .. }
            | Self::FieldAccess { meta, .. }
            | Self::ArraySubscript { meta, .. }
            | Self::Cast { meta, .. }
            | Self::Deref { meta, .. }
            | Self::AddrOf { meta, .. }
            | Self::CPythonMacro { meta, .. } => meta,
        }
    }

    /// Check if this is a `CPython` API call
    #[must_use]
    pub fn is_cpython_api(&self) -> bool {
        match self {
            Self::Call { callee, .. } => {
                if let Self::Variable { name, .. } = callee.as_ref() {
                    name.starts_with("Py") || name.starts_with("_Py")
                } else {
                    false
                }
            }
            Self::CPythonMacro { .. } => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c_function_creation() {
        let func = CHIR::Function {
            id: NodeId::new(1),
            name: "list_length".to_owned(),
            return_type: Type::C(crate::types::CType::SizeT),
            params: vec![],
            body: vec![],
            storage_class: StorageClass::Static,
            visibility: Visibility::Private,
            meta: Metadata::new(),
        };

        assert_eq!(func.id(), Some(NodeId::new(1)));
    }

    #[test]
    fn test_cpython_api_detection() {
        let py_call = CHIR::Call {
            id: NodeId::new(2),
            callee: Box::new(CHIR::Variable {
                id: NodeId::new(3),
                name: "PyList_Append".to_owned(),
                var_type: None,
                meta: Metadata::new(),
            }),
            args: vec![],
            inferred_type: None,
            meta: Metadata::new(),
        };

        assert!(py_call.is_cpython_api());

        let normal_call = CHIR::Call {
            id: NodeId::new(4),
            callee: Box::new(CHIR::Variable {
                id: NodeId::new(5),
                name: "strlen".to_owned(),
                var_type: None,
                meta: Metadata::new(),
            }),
            args: vec![],
            inferred_type: None,
            meta: Metadata::new(),
        };

        assert!(!normal_call.is_cpython_api());
    }

    #[test]
    fn test_cpython_macro() {
        let macro_call = CHIR::CPythonMacro {
            id: NodeId::new(6),
            name: "Py_SIZE".to_owned(),
            args: vec![],
            inferred_type: None,
            meta: Metadata::new(),
        };

        assert!(macro_call.is_cpython_api());
    }
}
