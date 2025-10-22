//! C parser using clang-sys
//!
//! This module provides C parsing functionality using LLVM/Clang bindings.
//! Following decy's approach for production-grade C parsing.

use anyhow::{Context, Result};
use clang_sys::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::ptr;

/// Simplified C AST representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CAST {
    /// Node type (e.g., "TranslationUnit", "FunctionDecl", "CallExpr")
    pub node_type: String,
    /// Node name (for functions, variables, etc.)
    pub name: Option<String>,
    /// Return type (for functions)
    pub return_type: Option<String>,
    /// Parameters (for functions)
    pub params: Vec<CParam>,
    /// Child nodes
    pub children: Vec<CAST>,
    /// Attributes
    pub attributes: HashMap<String, String>,
    /// Is this a CPython API node?
    pub is_cpython_api: bool,
}

/// C function parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CParam {
    /// Parameter name
    pub name: String,
    /// Parameter type
    pub param_type: String,
}

impl CAST {
    /// Create a new AST node
    #[must_use]
    pub fn new(node_type: String) -> Self {
        Self {
            node_type,
            name: None,
            return_type: None,
            params: Vec::new(),
            children: Vec::new(),
            attributes: HashMap::new(),
            is_cpython_api: false,
        }
    }
}

/// C parser using clang-sys
pub struct CParser {
    index: CXIndex,
}

impl CParser {
    /// Create a new C parser
    ///
    /// # Errors
    ///
    /// Returns an error if the clang index cannot be created
    pub fn new() -> Result<Self> {
        // SAFETY: clang_createIndex is safe with these parameters
        let index = unsafe { clang_createIndex(0, 0) };
        if index.is_null() {
            anyhow::bail!("Failed to create clang index");
        }
        Ok(Self { index })
    }

    /// Parse C source code
    ///
    /// # Errors
    ///
    /// Returns an error if parsing fails
    pub fn parse(&self, source: &str, filename: &str) -> Result<CAST> {
        if source.trim().is_empty() {
            return Ok(CAST::new("TranslationUnit".to_string()));
        }

        let filename_cstr = CString::new(filename).context("Failed to create filename CString")?;
        let source_cstr = CString::new(source).context("Failed to create source CString")?;

        // SAFETY: Creating unsaved file with valid C strings
        let unsaved_file = CXUnsavedFile {
            Filename: filename_cstr.as_ptr(),
            Contents: source_cstr.as_ptr(),
            Length: source.len() as std::os::raw::c_ulong,
        };

        // SAFETY: Parsing with clang
        let mut tu = ptr::null_mut();
        let result = unsafe {
            clang_parseTranslationUnit2(
                self.index,
                filename_cstr.as_ptr(),
                ptr::null(),
                0,
                &unsaved_file as *const CXUnsavedFile as *mut CXUnsavedFile,
                1,
                CXTranslationUnit_DetailedPreprocessingRecord,
                &mut tu,
            )
        };

        if result != CXError_Success {
            anyhow::bail!("Failed to parse C source: error code {result}");
        }

        if tu.is_null() {
            anyhow::bail!("Translation unit is null");
        }

        // Get the root cursor
        let cursor = unsafe { clang_getTranslationUnitCursor(tu) };

        // Visit the AST
        let mut root = CAST::new("TranslationUnit".to_string());
        unsafe {
            clang_visitChildren(cursor, visit_node, &mut root as *mut CAST as CXClientData);
        }

        // Cleanup
        unsafe {
            clang_disposeTranslationUnit(tu);
        }

        Ok(root)
    }
}

impl Drop for CParser {
    fn drop(&mut self) {
        if !self.index.is_null() {
            unsafe {
                clang_disposeIndex(self.index);
            }
        }
    }
}

/// Visitor function for AST traversal
///
/// # Safety
///
/// This function is called by clang and must handle C FFI correctly
extern "C" fn visit_node(
    cursor: CXCursor,
    _parent: CXCursor,
    client_data: CXClientData,
) -> CXChildVisitResult {
    unsafe {
        let parent_ast = &mut *(client_data as *mut CAST);

        let kind = clang_getCursorKind(cursor);
        let kind_spelling = clang_getCursorKindSpelling(kind);
        let node_type = to_rust_string(kind_spelling);

        let mut node = CAST::new(node_type);

        // Get node name if available
        let cursor_spelling = clang_getCursorSpelling(cursor);
        if !is_empty_string(&cursor_spelling) {
            node.name = Some(to_rust_string(cursor_spelling));

            // Check if this is a CPython API name
            if let Some(ref name) = node.name {
                node.is_cpython_api = is_cpython_api_name(name);
            }
        }

        // For function declarations, get return type and parameters
        if kind == CXCursor_FunctionDecl {
            let func_type = clang_getCursorType(cursor);
            let return_type = clang_getResultType(func_type);
            let return_type_spelling = clang_getTypeSpelling(return_type);
            node.return_type = Some(to_rust_string(return_type_spelling));

            // Get parameters
            let num_args = clang_Cursor_getNumArguments(cursor);
            for i in 0..num_args {
                let arg = clang_Cursor_getArgument(cursor, i as u32);
                let arg_name = clang_getCursorSpelling(arg);
                let arg_type = clang_getCursorType(arg);
                let arg_type_spelling = clang_getTypeSpelling(arg_type);

                node.params.push(CParam {
                    name: to_rust_string(arg_name),
                    param_type: to_rust_string(arg_type_spelling),
                });
            }
        }

        // Recursively visit children
        clang_visitChildren(cursor, visit_node, &mut node as *mut CAST as CXClientData);

        parent_ast.children.push(node);

        CXChildVisit_Continue
    }
}

/// Convert CXString to Rust String
///
/// # Safety
///
/// Must be called with a valid CXString
unsafe fn to_rust_string(cx_string: CXString) -> String {
    if cx_string.data.is_null() {
        return String::new();
    }
    let c_str = clang_getCString(cx_string);
    let rust_str = if c_str.is_null() {
        String::new()
    } else {
        CStr::from_ptr(c_str).to_string_lossy().into_owned()
    };
    clang_disposeString(cx_string);
    rust_str
}

/// Check if CXString is empty
///
/// # Safety
///
/// Must be called with a valid CXString reference
unsafe fn is_empty_string(cx_string: &CXString) -> bool {
    if cx_string.data.is_null() {
        return true;
    }
    let c_str = clang_getCString(*cx_string);
    if c_str.is_null() {
        return true;
    }
    let len = CStr::from_ptr(c_str).to_bytes().len();
    len == 0
}

/// Check if a name is a CPython API identifier
fn is_cpython_api_name(name: &str) -> bool {
    name.starts_with("Py")
        || name.starts_with("_Py")
        || name.starts_with("PyList_")
        || name.starts_with("PyDict_")
        || name.starts_with("PyObject_")
}

/// Parse C source code (convenience function)
///
/// # Errors
///
/// Returns an error if parsing fails
pub fn parse(source: &str, filename: &str) -> Result<CAST> {
    let parser = CParser::new()?;
    parser.parse(source, filename)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty() {
        let result = parse("", "empty.c");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_simple_function() {
        let source = r"
int add(int a, int b) {
    return a + b;
}
";
        let result = parse(source, "test.c");
        assert!(result.is_ok());

        let ast = result.unwrap();
        assert_eq!(ast.node_type, "TranslationUnit");

        // Should have at least one child (the function)
        assert!(!ast.children.is_empty());
    }

    #[test]
    fn test_parse_cpython_function() {
        let source = r"
static Py_ssize_t
list_length(PyListObject *self) {
    return Py_SIZE(self);
}
";
        let result = parse(source, "listobject.c");
        assert!(result.is_ok());

        let ast = result.unwrap();
        assert_eq!(ast.node_type, "TranslationUnit");
    }

    #[test]
    fn test_cpython_api_detection() {
        assert!(is_cpython_api_name("PyList_Append"));
        assert!(is_cpython_api_name("Py_SIZE"));
        assert!(is_cpython_api_name("PyObject_Call"));
        assert!(is_cpython_api_name("_PyObject_New"));
        assert!(!is_cpython_api_name("list_length"));
        assert!(!is_cpython_api_name("add"));
    }
}
