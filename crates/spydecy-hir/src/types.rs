//! Type system for Unified HIR
//!
//! This module defines the type representation that bridges Python's dynamic typing,
//! C's static typing, and Rust's ownership system.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Type representation in the Unified HIR
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Type {
    /// Python built-in types
    Python(PythonType),
    /// C types
    C(CType),
    /// Rust types (target)
    Rust(RustType),
    /// Generic type parameter
    Generic {
        /// Type parameter name
        name: String,
        /// Bounds/constraints
        bounds: Vec<String>,
    },
    /// Function type
    Function {
        /// Parameter types
        params: Vec<Type>,
        /// Return type
        return_type: Box<Type>,
    },
    /// Unknown/inferred type
    Unknown,
}

/// Python type
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PythonType {
    /// int
    Int,
    /// float
    Float,
    /// str
    Str,
    /// bool
    Bool,
    /// list[T]
    List(Box<Type>),
    /// dict[K, V]
    Dict {
        /// Key type
        key: Box<Type>,
        /// Value type
        value: Box<Type>,
    },
    /// tuple[T1, T2, ...]
    Tuple(Vec<Type>),
    /// set[T]
    Set(Box<Type>),
    /// None
    None,
    /// Any (dynamic)
    Any,
    /// Custom class
    Class(String),
}

/// C type
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CType {
    /// void
    Void,
    /// char
    Char,
    /// int
    Int,
    /// long
    Long,
    /// `size_t`
    SizeT,
    /// float
    Float,
    /// double
    Double,
    /// Pointer to type
    Pointer(Box<CType>),
    /// Array of type with size
    Array {
        /// Element type
        element: Box<CType>,
        /// Array size (None for flexible array)
        size: Option<usize>,
    },
    /// Struct
    Struct(String),
    /// Union
    Union(String),
    /// Typedef
    Typedef(String),
    /// `CPython` API types
    CPython(CPythonType),
}

/// `CPython` API types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CPythonType {
    /// `PyObject`*
    PyObject,
    /// `PyListObject`*
    PyListObject,
    /// `PyDictObject`*
    PyDictObject,
    /// `PyTupleObject`*
    PyTupleObject,
    /// `PyTypeObject`*
    PyTypeObject,
    /// `Py_ssize_t`
    PySsizeT,
}

/// Rust type (target)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RustType {
    /// i8, i16, i32, i64, i128, isize
    Int {
        /// Bits (8, 16, 32, 64, 128, size)
        bits: IntSize,
        /// Signed or unsigned
        signed: bool,
    },
    /// f32, f64
    Float {
        /// Bits (32 or 64)
        bits: u8,
    },
    /// bool
    Bool,
    /// String
    String,
    /// &str
    Str,
    /// Vec<T>
    Vec(Box<Type>),
    /// `HashMap`<K, V>
    HashMap {
        /// Key type
        key: Box<Type>,
        /// Value type
        value: Box<Type>,
    },
    /// (T1, T2, ...)
    Tuple(Vec<Type>),
    /// Option<T>
    Option(Box<Type>),
    /// Result<T, E>
    Result {
        /// Ok type
        ok: Box<Type>,
        /// Error type
        err: Box<Type>,
    },
    /// &T
    Reference {
        /// Mutable reference
        mutable: bool,
        /// Referenced type
        inner: Box<Type>,
    },
    /// Custom type
    Custom(String),
    /// Unit type ()
    Unit,
}

/// Integer size
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IntSize {
    /// 8 bits
    I8,
    /// 16 bits
    I16,
    /// 32 bits
    I32,
    /// 64 bits
    I64,
    /// 128 bits
    I128,
    /// Pointer size
    ISize,
}

impl Type {
    /// Check if type is compatible with another type (for unification)
    #[must_use]
    pub fn is_compatible(&self, other: &Self) -> bool {
        match (self, other) {
            // Python list → Rust Vec
            (Type::Python(PythonType::List(_)), Type::Rust(RustType::Vec(_))) => true,
            // Python dict → Rust HashMap
            (Type::Python(PythonType::Dict { .. }), Type::Rust(RustType::HashMap { .. })) => true,
            // C PyListObject → Rust Vec
            (Type::C(CType::CPython(CPythonType::PyListObject)), Type::Rust(RustType::Vec(_))) => {
                true
            }
            // Same types are compatible
            (a, b) if a == b => true,
            // Unknown types are always compatible
            (Type::Unknown, _) | (_, Type::Unknown) => true,
            _ => false,
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Python(py_type) => write!(f, "{py_type}"),
            Self::C(c_type) => write!(f, "{c_type}"),
            Self::Rust(rust_type) => write!(f, "{rust_type}"),
            Self::Generic { name, .. } => write!(f, "{name}"),
            Self::Function { params, return_type } => {
                write!(f, "fn(")?;
                for (i, param) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{param}")?;
                }
                write!(f, ") -> {return_type}")
            }
            Self::Unknown => write!(f, "?"),
        }
    }
}

impl fmt::Display for PythonType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int => write!(f, "int"),
            Self::Float => write!(f, "float"),
            Self::Str => write!(f, "str"),
            Self::Bool => write!(f, "bool"),
            Self::List(inner) => write!(f, "list[{inner}]"),
            Self::Dict { key, value } => write!(f, "dict[{key}, {value}]"),
            Self::Tuple(types) => {
                write!(f, "tuple[")?;
                for (i, t) in types.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{t}")?;
                }
                write!(f, "]")
            }
            Self::Set(inner) => write!(f, "set[{inner}]"),
            Self::None => write!(f, "None"),
            Self::Any => write!(f, "Any"),
            Self::Class(name) => write!(f, "{name}"),
        }
    }
}

impl fmt::Display for CType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Void => write!(f, "void"),
            Self::Char => write!(f, "char"),
            Self::Int => write!(f, "int"),
            Self::Long => write!(f, "long"),
            Self::SizeT => write!(f, "size_t"),
            Self::Float => write!(f, "float"),
            Self::Double => write!(f, "double"),
            Self::Pointer(inner) => write!(f, "{inner}*"),
            Self::Array { element, size } => {
                if let Some(s) = size {
                    write!(f, "{element}[{s}]")
                } else {
                    write!(f, "{element}[]")
                }
            }
            Self::Struct(name) => write!(f, "struct {name}"),
            Self::Union(name) => write!(f, "union {name}"),
            Self::Typedef(name) => write!(f, "{name}"),
            Self::CPython(cpy) => write!(f, "{cpy}"),
        }
    }
}

impl fmt::Display for CPythonType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PyObject => write!(f, "PyObject*"),
            Self::PyListObject => write!(f, "PyListObject*"),
            Self::PyDictObject => write!(f, "PyDictObject*"),
            Self::PyTupleObject => write!(f, "PyTupleObject*"),
            Self::PyTypeObject => write!(f, "PyTypeObject*"),
            Self::PySsizeT => write!(f, "Py_ssize_t"),
        }
    }
}

impl fmt::Display for RustType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int { bits, signed } => {
                let prefix = if *signed { "i" } else { "u" };
                let size = match bits {
                    IntSize::I8 => "8",
                    IntSize::I16 => "16",
                    IntSize::I32 => "32",
                    IntSize::I64 => "64",
                    IntSize::I128 => "128",
                    IntSize::ISize => "size",
                };
                write!(f, "{prefix}{size}")
            }
            Self::Float { bits } => write!(f, "f{bits}"),
            Self::Bool => write!(f, "bool"),
            Self::String => write!(f, "String"),
            Self::Str => write!(f, "&str"),
            Self::Vec(inner) => write!(f, "Vec<{inner}>"),
            Self::HashMap { key, value } => write!(f, "HashMap<{key}, {value}>"),
            Self::Tuple(types) => {
                write!(f, "(")?;
                for (i, t) in types.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{t}")?;
                }
                write!(f, ")")
            }
            Self::Option(inner) => write!(f, "Option<{inner}>"),
            Self::Result { ok, err } => write!(f, "Result<{ok}, {err}>"),
            Self::Reference { mutable, inner } => {
                if *mutable {
                    write!(f, "&mut {inner}")
                } else {
                    write!(f, "&{inner}")
                }
            }
            Self::Custom(name) => write!(f, "{name}"),
            Self::Unit => write!(f, "()"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_python_list_to_rust_vec_compatibility() {
        let py_list = Type::Python(PythonType::List(Box::new(Type::Python(PythonType::Int))));
        let rust_vec = Type::Rust(RustType::Vec(Box::new(Type::Rust(RustType::Int {
            bits: IntSize::I32,
            signed: true,
        }))));

        assert!(py_list.is_compatible(&rust_vec));
    }

    #[test]
    fn test_c_pylistobject_to_rust_vec_compatibility() {
        let c_list = Type::C(CType::CPython(CPythonType::PyListObject));
        let rust_vec = Type::Rust(RustType::Vec(Box::new(Type::Unknown)));

        assert!(c_list.is_compatible(&rust_vec));
    }

    #[test]
    fn test_type_display() {
        let py_list = Type::Python(PythonType::List(Box::new(Type::Python(PythonType::Int))));
        assert_eq!(py_list.to_string(), "list[int]");

        let rust_vec = Type::Rust(RustType::Vec(Box::new(Type::Rust(RustType::Int {
            bits: IntSize::I32,
            signed: true,
        }))));
        assert_eq!(rust_vec.to_string(), "Vec<i32>");
    }
}
