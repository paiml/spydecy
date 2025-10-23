//! Transpilation State Tracking
//!
//! Tracks the state of transpilation through all phases for step-through debugging.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use spydecy_hir::{c::CHIR, python::PythonHIR, unified::UnifiedHIR};
use std::path::PathBuf;

/// Current phase of transpilation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TranspilationPhase {
    /// Initial state
    Start,
    /// Python source parsed to AST
    PythonParsed,
    /// Python AST converted to HIR
    PythonHIR,
    /// C source parsed to AST
    CParsed,
    /// C AST converted to HIR
    CHIR,
    /// Python + C unified
    UnifiedHIR,
    /// HIR optimized
    Optimized,
    /// Rust code generated
    RustGenerated,
    /// Complete
    Complete,
}

impl TranspilationPhase {
    /// Get human-readable name
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Start => "Start",
            Self::PythonParsed => "Python Parsed",
            Self::PythonHIR => "Python HIR",
            Self::CParsed => "C Parsed",
            Self::CHIR => "C HIR",
            Self::UnifiedHIR => "Unified HIR",
            Self::Optimized => "Optimized",
            Self::RustGenerated => "Rust Generated",
            Self::Complete => "Complete",
        }
    }

    /// Get the next phase
    #[must_use]
    pub const fn next(self) -> Option<Self> {
        match self {
            Self::Start => Some(Self::PythonParsed),
            Self::PythonParsed => Some(Self::PythonHIR),
            Self::PythonHIR => Some(Self::CParsed),
            Self::CParsed => Some(Self::CHIR),
            Self::CHIR => Some(Self::UnifiedHIR),
            Self::UnifiedHIR => Some(Self::Optimized),
            Self::Optimized => Some(Self::RustGenerated),
            Self::RustGenerated => Some(Self::Complete),
            Self::Complete => None,
        }
    }
}

/// Transpilation state snapshot
#[derive(Debug, Clone)]
pub struct TranspilationState {
    /// Current phase
    pub phase: TranspilationPhase,
    /// Step count
    pub step_count: usize,
    /// Python source file
    pub python_file: Option<PathBuf>,
    /// C source file
    pub c_file: Option<PathBuf>,
    /// Python source code
    pub python_source: Option<String>,
    /// C source code
    pub c_source: Option<String>,
    /// Python HIR
    pub python_hir: Option<PythonHIR>,
    /// C HIR
    pub c_hir: Option<CHIR>,
    /// Unified HIR
    pub unified_hir: Option<UnifiedHIR>,
    /// Optimized HIR
    pub optimized_hir: Option<UnifiedHIR>,
    /// Generated Rust code
    pub rust_code: Option<String>,
}

impl TranspilationState {
    /// Create new state
    #[must_use]
    pub fn new(python_file: PathBuf, c_file: PathBuf) -> Self {
        Self {
            phase: TranspilationPhase::Start,
            step_count: 0,
            python_file: Some(python_file),
            c_file: Some(c_file),
            python_source: None,
            c_source: None,
            python_hir: None,
            c_hir: None,
            unified_hir: None,
            optimized_hir: None,
            rust_code: None,
        }
    }

    /// Advance to next phase
    ///
    /// # Errors
    ///
    /// Returns error if already at final phase
    pub fn advance(&mut self) -> Result<TranspilationPhase> {
        if let Some(next) = self.phase.next() {
            self.phase = next;
            self.step_count += 1;
            Ok(next)
        } else {
            anyhow::bail!("Already at final phase");
        }
    }

    /// Check if transpilation is complete
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        matches!(self.phase, TranspilationPhase::Complete)
    }
}
