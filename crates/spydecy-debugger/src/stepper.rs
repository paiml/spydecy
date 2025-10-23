//! Transpilation Stepper
//!
//! Core logic for stepping through transpilation phases.

use crate::commands::Breakpoint;
use crate::state::{TranspilationPhase, TranspilationState};
use anyhow::{Context, Result};
use spydecy_c::parse_c;
use spydecy_codegen::generate_rust;
use spydecy_hir::unified::Unifier;
use spydecy_optimizer::OptimizationPipeline;
use spydecy_python::parse_python;
use std::fs;

/// Transpilation stepper - manages stepping through phases
pub struct Stepper {
    state: TranspilationState,
    breakpoints: Vec<Breakpoint>,
}

impl Stepper {
    /// Create new stepper
    #[must_use]
    pub fn new(state: TranspilationState) -> Self {
        Self {
            state,
            breakpoints: Vec::new(),
        }
    }

    /// Get current state
    #[must_use]
    pub const fn state(&self) -> &TranspilationState {
        &self.state
    }

    /// Add breakpoint
    pub fn add_breakpoint(&mut self, bp: Breakpoint) {
        self.breakpoints.push(bp);
    }

    /// List breakpoints
    #[must_use]
    pub fn breakpoints(&self) -> &[Breakpoint] {
        &self.breakpoints
    }

    /// Clear breakpoint by index
    pub fn clear_breakpoint(&mut self, index: usize) -> bool {
        if index < self.breakpoints.len() {
            self.breakpoints.remove(index);
            true
        } else {
            false
        }
    }

    /// Check if breakpoint should trigger
    fn check_breakpoint(&self) -> bool {
        for bp in &self.breakpoints {
            match bp {
                Breakpoint::BoundaryElimination => {
                    if matches!(self.state.phase, TranspilationPhase::Optimized) {
                        return true;
                    }
                }
                Breakpoint::Phase(phase_name) => {
                    if self.state.phase.name().eq_ignore_ascii_case(phase_name) {
                        return true;
                    }
                }
                Breakpoint::Function(_) => {
                    // Function breakpoints NYI
                }
            }
        }
        false
    }

    /// Step to next phase
    ///
    /// # Errors
    ///
    /// Returns error if phase transition fails
    pub fn step(&mut self) -> Result<TranspilationPhase> {
        let next_phase = self.state.advance()?;

        match next_phase {
            TranspilationPhase::PythonParsed => self.parse_python()?,
            TranspilationPhase::CParsed => self.parse_c()?,
            TranspilationPhase::UnifiedHIR => self.unify()?,
            TranspilationPhase::Optimized => self.optimize()?,
            TranspilationPhase::RustGenerated => self.generate_rust()?,
            // These phases don't require additional processing
            TranspilationPhase::PythonHIR
            | TranspilationPhase::CHIR
            | TranspilationPhase::Complete
            | TranspilationPhase::Start => {}
        }

        Ok(next_phase)
    }

    fn parse_python(&mut self) -> Result<()> {
        let python_file = self
            .state
            .python_file
            .as_ref()
            .context("No Python file set")?;

        let source = fs::read_to_string(python_file)?;
        self.state.python_source = Some(source.clone());

        let hir = parse_python(&source, python_file.to_str().unwrap_or("input.py"))?;
        self.state.python_hir = Some(hir);

        Ok(())
    }

    fn parse_c(&mut self) -> Result<()> {
        let c_file = self.state.c_file.as_ref().context("No C file set")?;

        let source = fs::read_to_string(c_file)?;
        self.state.c_source = Some(source.clone());

        let hir = parse_c(&source, c_file.to_str().unwrap_or("input.c"))?;
        self.state.c_hir = Some(hir);

        Ok(())
    }

    fn unify(&mut self) -> Result<()> {
        let python_hir = self
            .state
            .python_hir
            .as_ref()
            .context("No Python HIR")?
            .clone();
        let c_hir = self.state.c_hir.as_ref().context("No C HIR")?.clone();

        // Extract callable from Python
        let python_call = extract_python_call(python_hir)?;
        let c_function = extract_c_function(c_hir)?;

        let mut unifier = Unifier::new();
        let unified_hir = unifier.unify(&python_call, &c_function)?;

        self.state.unified_hir = Some(unified_hir);
        Ok(())
    }

    fn optimize(&mut self) -> Result<()> {
        let unified = self
            .state
            .unified_hir
            .as_ref()
            .context("No Unified HIR")?
            .clone();

        let pipeline = OptimizationPipeline::standard();
        let optimized = pipeline.run(unified)?;

        self.state.optimized_hir = Some(optimized);
        Ok(())
    }

    fn generate_rust(&mut self) -> Result<()> {
        let optimized = self
            .state
            .optimized_hir
            .as_ref()
            .context("No optimized HIR")?;

        let rust_code = generate_rust(optimized)?;
        self.state.rust_code = Some(rust_code);

        Ok(())
    }

    /// Continue until breakpoint or completion
    ///
    /// # Errors
    ///
    /// Returns error if any phase fails
    pub fn continue_execution(&mut self) -> Result<()> {
        while !self.state.is_complete() {
            self.step()?;

            if self.check_breakpoint() {
                break;
            }
        }
        Ok(())
    }
}

fn extract_python_call(
    python_hir: spydecy_hir::python::PythonHIR,
) -> Result<spydecy_hir::python::PythonHIR> {
    use spydecy_hir::python::PythonHIR;

    if let PythonHIR::Module { body, .. } = python_hir {
        if let Some(PythonHIR::Function {
            body: func_body, ..
        }) = body.first()
        {
            if let Some(PythonHIR::Return {
                value: Some(call), ..
            }) = func_body.first()
            {
                return Ok(call.as_ref().clone());
            }
        }
    }
    anyhow::bail!("Could not extract Python call");
}

fn extract_c_function(c_hir: spydecy_hir::c::CHIR) -> Result<spydecy_hir::c::CHIR> {
    use spydecy_hir::c::CHIR;

    if let CHIR::TranslationUnit { declarations, .. } = c_hir {
        declarations.first().cloned().context("No C declarations")
    } else {
        anyhow::bail!("Expected C TranslationUnit")
    }
}
