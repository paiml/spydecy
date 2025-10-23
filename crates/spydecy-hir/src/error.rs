//! Error types for Spydecy HIR operations
//!
//! Provides user-friendly, actionable error messages with helpful hints.

use crate::{c::CHIR, python::PythonHIR, unified::UnificationPattern};
use std::fmt;

/// Result type for HIR operations
pub type Result<T> = std::result::Result<T, UnificationError>;

/// Errors that can occur during Python-C unification
#[derive(Debug, Clone)]
pub enum UnificationError {
    /// No pattern found to unify Python and C code
    NoPatternMatch {
        /// Python function name
        python_fn: String,
        /// C function name
        c_fn: String,
        /// Suggested similar patterns
        suggestions: Vec<PatternSuggestion>,
    },

    /// Python and C nodes are incompatible types
    IncompatibleNodes {
        /// Python node description
        python_kind: String,
        /// C node description
        c_kind: String,
    },

    /// Unsupported Python HIR node
    UnsupportedPython {
        /// Node type description
        node_kind: String,
    },

    /// Unsupported C HIR node
    UnsupportedC {
        /// Node type description
        node_kind: String,
    },
}

/// Suggested pattern that might match the user's intent
#[derive(Debug, Clone)]
pub struct PatternSuggestion {
    /// Pattern name
    pub pattern: UnificationPattern,
    /// Python function name
    pub python_fn: &'static str,
    /// C function name
    pub c_fn: &'static str,
    /// Generated Rust code
    pub rust_output: &'static str,
}

impl PatternSuggestion {
    /// Create a new pattern suggestion
    pub const fn new(
        pattern: UnificationPattern,
        python_fn: &'static str,
        c_fn: &'static str,
        rust_output: &'static str,
    ) -> Self {
        Self {
            pattern,
            python_fn,
            c_fn,
            rust_output,
        }
    }
}

/// Get all supported pattern suggestions
pub fn all_patterns() -> Vec<PatternSuggestion> {
    vec![
        PatternSuggestion::new(
            UnificationPattern::LenPattern,
            "len()",
            "list_length()",
            "Vec::len()",
        ),
        PatternSuggestion::new(
            UnificationPattern::AppendPattern,
            "append()",
            "PyList_Append()",
            "Vec::push()",
        ),
        PatternSuggestion::new(
            UnificationPattern::DictGetPattern,
            "get()",
            "PyDict_GetItem()",
            "HashMap::get()",
        ),
        PatternSuggestion::new(
            UnificationPattern::ReversePattern,
            "reverse()",
            "list_reverse()",
            "Vec::reverse()",
        ),
        PatternSuggestion::new(
            UnificationPattern::ClearPattern,
            "clear()",
            "list_clear()",
            "Vec::clear()",
        ),
        PatternSuggestion::new(
            UnificationPattern::PopPattern,
            "pop()",
            "list_pop()",
            "Vec::pop()",
        ),
        PatternSuggestion::new(
            UnificationPattern::InsertPattern,
            "insert()",
            "list_insert()",
            "Vec::insert()",
        ),
        PatternSuggestion::new(
            UnificationPattern::ExtendPattern,
            "extend()",
            "list_extend()",
            "Vec::extend()",
        ),
        PatternSuggestion::new(
            UnificationPattern::DictPopPattern,
            "dict_pop()",
            "PyDict_DelItem()",
            "HashMap::remove()",
        ),
        PatternSuggestion::new(
            UnificationPattern::DictClearPattern,
            "dict_clear()",
            "PyDict_Clear()",
            "HashMap::clear()",
        ),
        PatternSuggestion::new(
            UnificationPattern::DictKeysPattern,
            "keys()",
            "PyDict_Keys()",
            "HashMap::keys()",
        ),
    ]
}

/// Find similar patterns based on function names
pub fn find_similar_patterns(python_fn: &str, c_fn: &str) -> Vec<PatternSuggestion> {
    let all = all_patterns();
    let mut suggestions = Vec::new();

    // Exact match on Python function
    for pattern in &all {
        if pattern.python_fn.contains(python_fn) || python_fn.contains(pattern.python_fn) {
            suggestions.push(pattern.clone());
        }
    }

    // Exact match on C function
    for pattern in &all {
        if pattern.c_fn.contains(c_fn) || c_fn.contains(pattern.c_fn) {
            suggestions.push(pattern.clone());
        }
    }

    // If no similar patterns found, return top 3 most common patterns
    if suggestions.is_empty() {
        suggestions.extend_from_slice(&all[0..3.min(all.len())]);
    }

    // Remove duplicates
    suggestions.sort_by_key(|s| s.python_fn);
    suggestions.dedup_by_key(|s| s.python_fn);

    suggestions
}

impl fmt::Display for UnificationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoPatternMatch {
                python_fn,
                c_fn,
                suggestions,
            } => {
                writeln!(
                    f,
                    "❌ Cannot match Python function '{}' with C function '{}'",
                    python_fn, c_fn
                )?;
                writeln!(f)?;
                writeln!(f, "Spydecy tried to unify:")?;
                writeln!(f, "  Python: {}()", python_fn)?;
                writeln!(f, "  C:      {}()", c_fn)?;
                writeln!(f)?;
                writeln!(f, "No known pattern matches this combination.")?;
                writeln!(f)?;

                if !suggestions.is_empty() {
                    writeln!(f, "💡 Supported patterns:")?;
                    for (i, suggestion) in suggestions.iter().enumerate().take(5) {
                        writeln!(
                            f,
                            "  {}. {} + {} → {}",
                            i + 1,
                            suggestion.python_fn,
                            suggestion.c_fn,
                            suggestion.rust_output
                        )?;
                    }
                    writeln!(f)?;
                }

                writeln!(f, "📖 For custom patterns, see:")?;
                writeln!(f, "   https://github.com/noahgift/spydecy#custom-patterns")?;
                Ok(())
            }

            Self::IncompatibleNodes {
                python_kind,
                c_kind,
            } => {
                writeln!(
                    f,
                    "❌ Cannot unify incompatible node types: Python {} with C {}",
                    python_kind, c_kind
                )?;
                writeln!(f)?;
                writeln!(f, "💡 Spydecy requires both nodes to be callable functions.")?;
                writeln!(f, "   Ensure your Python and C code represent the same operation.")?;
                Ok(())
            }

            Self::UnsupportedPython { node_kind } => {
                writeln!(f, "❌ Unsupported Python HIR node: {}", node_kind)?;
                writeln!(f)?;
                writeln!(f, "💡 This Python construct is not yet supported by Spydecy.")?;
                writeln!(f, "   Supported: function calls to known operations.")?;
                Ok(())
            }

            Self::UnsupportedC { node_kind } => {
                writeln!(f, "❌ Unsupported C HIR node: {}", node_kind)?;
                writeln!(f)?;
                writeln!(f, "💡 This C construct is not yet supported by Spydecy.")?;
                writeln!(f, "   Supported: function definitions.")?;
                Ok(())
            }
        }
    }
}

impl std::error::Error for UnificationError {}

/// Helper to extract function name from Python HIR
pub fn extract_python_fn_name(python: &PythonHIR) -> String {
    match python {
        PythonHIR::Call { callee, .. } => match callee.as_ref() {
            PythonHIR::Variable { name, .. } => name.clone(),
            _ => "<complex expression>".to_owned(),
        },
        PythonHIR::Variable { name, .. } => name.clone(),
        PythonHIR::Function { name, .. } => name.clone(),
        _ => format!("{:?}", python).split('{').next().unwrap_or("Unknown").to_owned(),
    }
}

/// Helper to extract function name from C HIR
pub fn extract_c_fn_name(c: &CHIR) -> String {
    match c {
        CHIR::Function { name, .. } => name.clone(),
        _ => format!("{:?}", c).split('{').next().unwrap_or("Unknown").to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_patterns_count() {
        let patterns = all_patterns();
        assert_eq!(patterns.len(), 11, "Should have 11 supported patterns");
    }

    #[test]
    fn test_find_similar_patterns_exact_match() {
        let suggestions = find_similar_patterns("len", "list_length");
        assert!(!suggestions.is_empty());
        assert_eq!(suggestions[0].python_fn, "len()");
    }

    #[test]
    fn test_find_similar_patterns_partial_match() {
        let suggestions = find_similar_patterns("append", "PyList");
        assert!(!suggestions.is_empty());
        // Should find append pattern
        assert!(suggestions
            .iter()
            .any(|s| s.python_fn.contains("append")));
    }

    #[test]
    fn test_find_similar_patterns_no_match() {
        let suggestions = find_similar_patterns("unknown_fn", "unknown_c_fn");
        // Should return top 3 common patterns as fallback
        assert!(!suggestions.is_empty());
        assert!(suggestions.len() <= 3);
    }

    #[test]
    fn test_error_display_no_pattern_match() {
        let error = UnificationError::NoPatternMatch {
            python_fn: "foo".to_owned(),
            c_fn: "bar".to_owned(),
            suggestions: all_patterns(),
        };

        let display = error.to_string();
        assert!(display.contains("foo"));
        assert!(display.contains("bar"));
        assert!(display.contains("Supported patterns"));
        assert!(display.contains("len()"));
    }

    #[test]
    fn test_error_display_incompatible_nodes() {
        let error = UnificationError::IncompatibleNodes {
            python_kind: "Module".to_owned(),
            c_kind: "Literal".to_owned(),
        };

        let display = error.to_string();
        assert!(display.contains("Module"));
        assert!(display.contains("Literal"));
        assert!(display.contains("incompatible"));
    }
}
