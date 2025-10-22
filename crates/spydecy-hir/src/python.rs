//! Python-specific HIR nodes
//!
//! This module defines HIR nodes for Python constructs.
//! These will be unified with C HIR nodes during cross-layer optimization.

use crate::{metadata::Metadata, types::Type, NodeId, Visibility};
use serde::{Deserialize, Serialize};

/// Python HIR node
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PythonHIR {
    /// Module (top-level)
    Module {
        /// Module name
        name: String,
        /// Module body
        body: Vec<PythonHIR>,
        /// Metadata
        meta: Metadata,
    },

    /// Function definition
    Function {
        /// Node ID
        id: NodeId,
        /// Function name
        name: String,
        /// Parameters
        params: Vec<Parameter>,
        /// Return type annotation
        return_type: Option<Type>,
        /// Function body
        body: Vec<PythonHIR>,
        /// Decorators
        decorators: Vec<String>,
        /// Visibility
        visibility: Visibility,
        /// Metadata
        meta: Metadata,
    },

    /// Class definition
    Class {
        /// Node ID
        id: NodeId,
        /// Class name
        name: String,
        /// Base classes
        bases: Vec<String>,
        /// Class body
        body: Vec<PythonHIR>,
        /// Decorators
        decorators: Vec<String>,
        /// Metadata
        meta: Metadata,
    },

    /// Function call
    Call {
        /// Node ID
        id: NodeId,
        /// Function being called
        callee: Box<PythonHIR>,
        /// Arguments
        args: Vec<PythonHIR>,
        /// Keyword arguments
        kwargs: Vec<(String, PythonHIR)>,
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
        /// Inferred type
        inferred_type: Option<Type>,
        /// Metadata
        meta: Metadata,
    },

    /// Assignment
    Assign {
        /// Node ID
        id: NodeId,
        /// Target variable
        target: String,
        /// Value being assigned
        value: Box<PythonHIR>,
        /// Type annotation
        type_annotation: Option<Type>,
        /// Metadata
        meta: Metadata,
    },

    /// Return statement
    Return {
        /// Node ID
        id: NodeId,
        /// Return value
        value: Option<Box<PythonHIR>>,
        /// Metadata
        meta: Metadata,
    },

    /// If statement
    If {
        /// Node ID
        id: NodeId,
        /// Condition
        condition: Box<PythonHIR>,
        /// Then branch
        then_branch: Vec<PythonHIR>,
        /// Else branch
        else_branch: Vec<PythonHIR>,
        /// Metadata
        meta: Metadata,
    },

    /// For loop
    For {
        /// Node ID
        id: NodeId,
        /// Loop variable
        target: String,
        /// Iterable
        iter: Box<PythonHIR>,
        /// Loop body
        body: Vec<PythonHIR>,
        /// Else clause
        orelse: Vec<PythonHIR>,
        /// Metadata
        meta: Metadata,
    },

    /// While loop
    While {
        /// Node ID
        id: NodeId,
        /// Condition
        condition: Box<PythonHIR>,
        /// Loop body
        body: Vec<PythonHIR>,
        /// Else clause
        orelse: Vec<PythonHIR>,
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
        left: Box<PythonHIR>,
        /// Right operand
        right: Box<PythonHIR>,
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
        operand: Box<PythonHIR>,
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

    /// List comprehension
    ListComp {
        /// Node ID
        id: NodeId,
        /// Element expression
        element: Box<PythonHIR>,
        /// Generators
        generators: Vec<Comprehension>,
        /// Metadata
        meta: Metadata,
    },

    /// Attribute access (obj.attr)
    Attribute {
        /// Node ID
        id: NodeId,
        /// Object
        object: Box<PythonHIR>,
        /// Attribute name
        attr: String,
        /// Inferred type
        inferred_type: Option<Type>,
        /// Metadata
        meta: Metadata,
    },

    /// Subscript (obj[index])
    Subscript {
        /// Node ID
        id: NodeId,
        /// Object
        object: Box<PythonHIR>,
        /// Index
        index: Box<PythonHIR>,
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
    /// Type annotation
    pub type_annotation: Option<Type>,
    /// Default value
    pub default: Option<String>,
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
    /// //
    FloorDiv,
    /// %
    Mod,
    /// **
    Pow,
    /// ==
    Eq,
    /// !=
    NotEq,
    /// <
    Lt,
    /// <=
    Le,
    /// >
    Gt,
    /// >=
    Ge,
    /// and
    And,
    /// or
    Or,
}

/// Unary operator
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnaryOp {
    /// not
    Not,
    /// -
    Neg,
    /// +
    Pos,
}

/// Literal value
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Literal {
    /// Integer
    Int(i64),
    /// Float
    Float(f64),
    /// String
    Str(String),
    /// Boolean
    Bool(bool),
    /// None
    None,
}

/// List comprehension generator
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Comprehension {
    /// Target variable
    pub target: String,
    /// Iterable
    pub iter: Box<PythonHIR>,
    /// Filters
    pub ifs: Vec<PythonHIR>,
}

impl PythonHIR {
    /// Get the node ID if present
    #[must_use]
    pub const fn id(&self) -> Option<NodeId> {
        match self {
            Self::Module { .. } => None,
            Self::Function { id, .. }
            | Self::Class { id, .. }
            | Self::Call { id, .. }
            | Self::Variable { id, .. }
            | Self::Assign { id, .. }
            | Self::Return { id, .. }
            | Self::If { id, .. }
            | Self::For { id, .. }
            | Self::While { id, .. }
            | Self::BinOp { id, .. }
            | Self::UnaryOp { id, .. }
            | Self::Literal { id, .. }
            | Self::ListComp { id, .. }
            | Self::Attribute { id, .. }
            | Self::Subscript { id, .. } => Some(*id),
        }
    }

    /// Get the metadata
    #[must_use]
    pub const fn metadata(&self) -> &Metadata {
        match self {
            Self::Module { meta, .. }
            | Self::Function { meta, .. }
            | Self::Class { meta, .. }
            | Self::Call { meta, .. }
            | Self::Variable { meta, .. }
            | Self::Assign { meta, .. }
            | Self::Return { meta, .. }
            | Self::If { meta, .. }
            | Self::For { meta, .. }
            | Self::While { meta, .. }
            | Self::BinOp { meta, .. }
            | Self::UnaryOp { meta, .. }
            | Self::Literal { meta, .. }
            | Self::ListComp { meta, .. }
            | Self::Attribute { meta, .. }
            | Self::Subscript { meta, .. } => meta,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_python_function_creation() {
        let func = PythonHIR::Function {
            id: NodeId::new(1),
            name: "test_func".to_owned(),
            params: vec![],
            return_type: None,
            body: vec![],
            decorators: vec![],
            visibility: Visibility::Public,
            meta: Metadata::new(),
        };

        assert_eq!(func.id(), Some(NodeId::new(1)));
    }

    #[test]
    fn test_python_call_creation() {
        let call = PythonHIR::Call {
            id: NodeId::new(2),
            callee: Box::new(PythonHIR::Variable {
                id: NodeId::new(3),
                name: "len".to_owned(),
                inferred_type: None,
                meta: Metadata::new(),
            }),
            args: vec![],
            kwargs: vec![],
            inferred_type: None,
            meta: Metadata::new(),
        };

        assert_eq!(call.id(), Some(NodeId::new(2)));
    }
}
