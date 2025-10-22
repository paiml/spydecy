//! HIR Bridge between Decy and Spydecy
//!
//! This crate provides conversion between Decy's HIR (High-level Intermediate Representation)
//! and Spydecy's types, enabling deep integration between the two transpiler projects.
//!
//! # Integration Vision
//!
//! ```text
//! C Source → decy-parser → Decy HIR
//!                              ↓
//!                      spydecy-decy-bridge
//!                              ↓
//!                         Spydecy Types
//!                              ↓
//!                  Python HIR + Spydecy Types → UnifiedHIR → Rust
//! ```
//!
//! # Status
//!
//! **Phase 1**: Type conversion layer (Current)
//! - Convert between Decy and Spydecy type systems
//! - Foundation for full HIR bridge
//!
//! **Phase 2**: Full HIR conversion (Planned)
//! - Convert decy HIR functions to spydecy CHIR
//! - Parse C with decy-parser, output spydecy-compatible HIR
//!
//! # Example
//!
//! ```
//! use spydecy_decy_bridge::DecyTypeConverter;
//!
//! // Convert Decy type to Spydecy type
//! let decy_type = decy_hir::HirType::Int;
//! let spydecy_type = DecyTypeConverter::convert(&decy_type)
//!     .expect("Should convert");
//! ```

#![warn(missing_docs, clippy::all, clippy::pedantic)]
#![deny(unsafe_code)]
#![allow(clippy::module_name_repetitions)]

use anyhow::Result;

/// Type converter between Decy and Spydecy
///
/// This struct provides methods to convert types between Decy's type system
/// and Spydecy's type system. This is Phase 1 of the integration plan.
pub struct DecyTypeConverter;

/// Function converter between Decy and Spydecy
///
/// This struct provides methods to convert complete HIR functions from Decy
/// to Spydecy CHIR. This is Phase 2 of the integration plan.
pub struct DecyFunctionConverter;

impl DecyTypeConverter {
    /// Convert Decy type to Spydecy type
    ///
    /// # Errors
    ///
    /// Returns an error if the type cannot be converted
    pub fn convert(decy_type: &decy_hir::HirType) -> Result<spydecy_hir::types::Type> {
        use spydecy_hir::types::{CType, RustType, Type};

        match decy_type {
            // Basic C types
            decy_hir::HirType::Void => Ok(Type::C(CType::Void)),
            decy_hir::HirType::Int => Ok(Type::C(CType::Int)),
            decy_hir::HirType::Float => Ok(Type::C(CType::Float)),
            decy_hir::HirType::Double => Ok(Type::C(CType::Double)),
            decy_hir::HirType::Char => Ok(Type::C(CType::Char)),

            // Pointer types
            decy_hir::HirType::Pointer(inner) => {
                let inner_type = Self::convert(inner)?;
                match inner_type {
                    Type::C(c) => Ok(Type::C(CType::Pointer(Box::new(c)))),
                    // For non-C types, wrap in a pointer fallback
                    _ => Ok(Type::C(CType::Pointer(Box::new(CType::Void)))),
                }
            }

            // Rust-oriented types from Decy
            decy_hir::HirType::Vec(inner) => {
                let inner_type = Self::convert(inner)?;
                Ok(Type::Rust(RustType::Vec(Box::new(inner_type))))
            }
            decy_hir::HirType::Option(inner) => {
                let inner_type = Self::convert(inner)?;
                Ok(Type::Rust(RustType::Option(Box::new(inner_type))))
            }
            decy_hir::HirType::Reference { inner, mutable } => {
                let inner_type = Self::convert(inner)?;
                Ok(Type::Rust(RustType::Reference {
                    mutable: *mutable,
                    inner: Box::new(inner_type),
                }))
            }

            // Struct/Enum types
            decy_hir::HirType::Struct(name) => Ok(Type::C(CType::Struct(name.clone()))),
            decy_hir::HirType::Enum(_name) => {
                // Spydecy doesn't have Enum in CType, use Typedef
                Ok(Type::C(CType::Int)) // Fallback to int for now
            }

            // Array types
            decy_hir::HirType::Array { element_type, .. } => {
                let elem_type = Self::convert(element_type)?;
                // Convert to pointer (C arrays decay to pointers)
                match elem_type {
                    Type::C(c) => Ok(Type::C(CType::Pointer(Box::new(c)))),
                    _ => Ok(Type::C(CType::Pointer(Box::new(CType::Int)))),
                }
            }

            // String types
            decy_hir::HirType::StringLiteral | decy_hir::HirType::StringReference => {
                Ok(Type::Rust(RustType::Str))
            }
            decy_hir::HirType::OwnedString => Ok(Type::Rust(RustType::String)),

            // Function pointers and Box
            decy_hir::HirType::FunctionPointer { .. } => {
                // Map to void pointer
                Ok(Type::C(CType::Pointer(Box::new(CType::Void))))
            }
            decy_hir::HirType::Box(inner) => {
                // Decy's Box maps to Spydecy pointer + ownership annotation
                let inner_type = Self::convert(inner)?;
                match inner_type {
                    Type::C(c) => Ok(Type::C(CType::Pointer(Box::new(c)))),
                    Type::Rust(r) => Ok(Type::Rust(RustType::Vec(Box::new(Type::Rust(r))))), // Use Vec as proxy
                    other => Ok(Type::Rust(RustType::Vec(Box::new(other)))),
                }
            }
        }
    }
}

impl DecyFunctionConverter {
    /// Convert Decy `HirFunction` to Spydecy CHIR Function
    ///
    /// # Errors
    ///
    /// Returns an error if the conversion fails
    pub fn convert(decy_func: &decy_hir::HirFunction) -> Result<spydecy_hir::c::CHIR> {
        use spydecy_hir::{
            c::{Parameter, StorageClass, CHIR},
            metadata::Metadata,
            NodeId, Visibility,
        };

        // Convert return type
        let return_type = DecyTypeConverter::convert(decy_func.return_type())?;

        // Convert parameters
        let params: Result<Vec<Parameter>> = decy_func
            .parameters()
            .iter()
            .map(|p| {
                Ok(Parameter {
                    name: p.name().to_owned(),
                    param_type: DecyTypeConverter::convert(p.param_type())?,
                })
            })
            .collect();

        // Create CHIR function
        // Note: Body conversion would be more complex and is left for Phase 2b
        Ok(CHIR::Function {
            id: NodeId::new(1),
            name: decy_func.name().to_owned(),
            return_type,
            params: params?,
            body: vec![], // Body statements would be converted here
            storage_class: StorageClass::Static, // Default to static
            visibility: Visibility::Private, // Default to private
            meta: Metadata::new(),
        })
    }

    /// Parse C source with decy-parser and convert to Spydecy CHIR
    ///
    /// This is a convenience method combining parsing and conversion.
    ///
    /// # Errors
    ///
    /// Returns an error if parsing or conversion fails
    ///
    /// # Example
    ///
    /// ```no_run
    /// use spydecy_decy_bridge::DecyFunctionConverter;
    ///
    /// let c_source = "int add(int a, int b) { return a + b; }";
    /// let spydecy_hir = DecyFunctionConverter::parse_and_convert(c_source, "add.c")?;
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn parse_and_convert(c_source: &str, filename: &str) -> Result<spydecy_hir::c::CHIR> {
        use anyhow::Context;

        // Parse with decy's comprehensive C parser
        let parser = decy_parser::CParser::new()?;
        let ast = parser
            .parse(c_source)
            .with_context(|| format!("Failed to parse C source: {filename}"))?;

        // Get first function from AST
        let func = ast
            .functions()
            .first()
            .with_context(|| format!("No functions found in {filename}"))?;

        // Convert AST function to decy HIR
        let decy_hir = decy_hir::HirFunction::from_ast_function(func);

        // Convert decy HIR to spydecy CHIR
        Self::convert(&decy_hir).context("Failed to convert Decy HIR to Spydecy CHIR")
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_basic_c_types() {
        // Test basic C type conversions
        let void_type = decy_hir::HirType::Void;
        let converted = DecyTypeConverter::convert(&void_type).unwrap();
        assert!(matches!(
            converted,
            spydecy_hir::types::Type::C(spydecy_hir::types::CType::Void)
        ));

        let int_type = decy_hir::HirType::Int;
        let converted = DecyTypeConverter::convert(&int_type).unwrap();
        assert!(matches!(
            converted,
            spydecy_hir::types::Type::C(spydecy_hir::types::CType::Int)
        ));

        let float_type = decy_hir::HirType::Float;
        let converted = DecyTypeConverter::convert(&float_type).unwrap();
        assert!(matches!(
            converted,
            spydecy_hir::types::Type::C(spydecy_hir::types::CType::Float)
        ));
    }

    #[test]
    fn test_convert_rust_oriented_types() {
        // Test Decy's Rust-oriented types
        let vec_type = decy_hir::HirType::Vec(Box::new(decy_hir::HirType::Int));
        let converted = DecyTypeConverter::convert(&vec_type).unwrap();
        assert!(matches!(
            converted,
            spydecy_hir::types::Type::Rust(spydecy_hir::types::RustType::Vec(_))
        ));

        let option_type = decy_hir::HirType::Option(Box::new(decy_hir::HirType::Int));
        let converted = DecyTypeConverter::convert(&option_type).unwrap();
        assert!(matches!(
            converted,
            spydecy_hir::types::Type::Rust(spydecy_hir::types::RustType::Option(_))
        ));
    }

    #[test]
    fn test_convert_pointer_types() {
        // Test pointer conversion
        let ptr_type = decy_hir::HirType::Pointer(Box::new(decy_hir::HirType::Int));
        let converted = DecyTypeConverter::convert(&ptr_type).unwrap();
        assert!(matches!(
            converted,
            spydecy_hir::types::Type::C(spydecy_hir::types::CType::Pointer(_))
        ));
    }

    #[test]
    fn test_convert_struct_types() {
        // Test struct conversion
        let struct_type = decy_hir::HirType::Struct("MyStruct".to_owned());
        let converted = DecyTypeConverter::convert(&struct_type).unwrap();
        assert!(matches!(
            converted,
            spydecy_hir::types::Type::C(spydecy_hir::types::CType::Struct(_))
        ));
    }

    #[test]
    fn test_convert_function_simple() {
        // Create a simple Decy function: int add(int a, int b)
        let decy_func = decy_hir::HirFunction::new(
            "add".to_owned(),
            decy_hir::HirType::Int,
            vec![
                decy_hir::HirParameter::new("a".to_owned(), decy_hir::HirType::Int),
                decy_hir::HirParameter::new("b".to_owned(), decy_hir::HirType::Int),
            ],
        );

        // Convert to Spydecy CHIR
        let spydecy_func = DecyFunctionConverter::convert(&decy_func).unwrap();

        // Verify it's a function
        if let spydecy_hir::c::CHIR::Function {
            name,
            return_type,
            params,
            ..
        } = spydecy_func
        {
            assert_eq!(name, "add");
            assert!(matches!(
                return_type,
                spydecy_hir::types::Type::C(spydecy_hir::types::CType::Int)
            ));
            assert_eq!(params.len(), 2);
            assert_eq!(params[0].name, "a");
            assert_eq!(params[1].name, "b");
        } else {
            panic!("Expected CHIR::Function");
        }
    }

    #[test]
    fn test_convert_function_no_params() {
        // Create function: int main()
        let decy_func =
            decy_hir::HirFunction::new("main".to_owned(), decy_hir::HirType::Int, vec![]);

        let spydecy_func = DecyFunctionConverter::convert(&decy_func).unwrap();

        if let spydecy_hir::c::CHIR::Function { name, params, .. } = spydecy_func {
            assert_eq!(name, "main");
            assert_eq!(params.len(), 0);
        } else {
            panic!("Expected CHIR::Function");
        }
    }

    #[test]
    fn test_parse_and_convert_integration() {
        // Test parsing C code with decy-parser and converting to Spydecy
        let c_source = r"
            static int add(int x, int y) {
                return x + y;
            }
        ";

        let result = DecyFunctionConverter::parse_and_convert(c_source, "test.c");
        assert!(result.is_ok(), "Should parse and convert C code");

        let spydecy_func = result.unwrap();
        if let spydecy_hir::c::CHIR::Function { name, params, .. } = spydecy_func {
            assert_eq!(name, "add");
            assert_eq!(params.len(), 2);
            assert_eq!(params[0].name, "x");
            assert_eq!(params[1].name, "y");
        } else {
            panic!("Expected CHIR::Function");
        }
    }
}
