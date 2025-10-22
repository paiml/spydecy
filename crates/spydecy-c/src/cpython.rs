//! CPython API pattern recognition
//!
//! This module identifies CPython API patterns in C code for unification.

use crate::parser::CAST;

/// Check if a C AST node is a CPython API call
#[must_use]
pub fn is_cpython_api(ast: &CAST) -> bool {
    ast.is_cpython_api
}

/// Identify CPython API pattern
#[must_use]
pub fn identify_pattern(ast: &CAST) -> Option<CPythonPattern> {
    if let Some(ref name) = ast.name {
        match name.as_str() {
            "list_length" | "PyList_Size" => Some(CPythonPattern::ListLength),
            "PyList_Append" => Some(CPythonPattern::ListAppend),
            "PyDict_GetItem" => Some(CPythonPattern::DictGet),
            "Py_SIZE" => Some(CPythonPattern::ObjectSize),
            _ => None,
        }
    } else {
        None
    }
}

/// CPython API patterns for unification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CPythonPattern {
    /// List length (maps to len())
    ListLength,
    /// List append (maps to append())
    ListAppend,
    /// Dict get (maps to dict.get())
    DictGet,
    /// Object size macro
    ObjectSize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identify_list_length_pattern() {
        let mut ast = CAST::new("FunctionDecl".to_string());
        ast.name = Some("list_length".to_string());

        assert_eq!(identify_pattern(&ast), Some(CPythonPattern::ListLength));
    }

    #[test]
    fn test_identify_pylist_size() {
        let mut ast = CAST::new("FunctionDecl".to_string());
        ast.name = Some("PyList_Size".to_string());

        assert_eq!(identify_pattern(&ast), Some(CPythonPattern::ListLength));
    }
}
