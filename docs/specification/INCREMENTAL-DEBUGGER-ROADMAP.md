# Incremental Debugger Development Roadmap
## Building Developer Tools Alongside the Transpiler

**Principle**: Jidoka (Automation with a Human Touch)
**Philosophy**: "Eat Your Own Dog Food"
**Timeline**: Sprints 2-15 (parallel with transpiler development)

---

## Executive Summary

**Problem**: The original roadmap defers the Introspective Debugger to Phase 3 (Sprints 11-15), meaning the team will spend **20 weeks building a complex transpiler with no specialized debugging tools**.

**Solution**: Build the debugger **incrementally, alongside the transpiler**, starting in Sprint 2. The debugger becomes the team's primary development tool, accelerating transpiler development while creating a battle-tested end-user product.

**Benefits**:
1. **Faster Development**: Team can debug transpiler issues 10x faster
2. **Better Debugger**: Tool will be intuitive because the team uses it daily
3. **Continuous Validation**: Each transpiler feature is immediately testable
4. **Eliminates Mura** (unevenness): No long period of difficult development followed by separate tool-building

---

## Core Principle: Developer-First Design

> "The best tools are created by developers who use them daily." - The Pragmatic Programmer

### Traditional Approach (Original Roadmap)
```
Sprints 1-10: Build transpiler (20 weeks)
  └─> Debug with generic tools (slow, painful)

Sprints 11-15: Build debugger (10 weeks)
  └─> Finally have good tools (too late)
```

**Problems**:
- Team suffers 20 weeks of productivity loss
- Debugger built without real-world usage experience
- High risk of building the wrong features

### Incremental Approach (Revised Roadmap)
```
Sprint 2: Build Python parser + visualize-python-ast debugger command
Sprint 3: Build C parser + visualize-c-ast debugger command
Sprint 4: Build Unified HIR + step-through debugger
Sprint 6: Build optimizer + breakpoint-on-optimization
...
```

**Benefits**:
- Team has appropriate tools for each development phase
- Debugger features validated immediately through daily use
- Natural prioritization (build what you need when you need it)

---

## Sprint-by-Sprint Debugger Features

### Sprint 2: Python Transpiler + Basic Visualization

**Transpiler Work**:
- DECY-005: Python AST parser
- DECY-006: Type hint extraction
- DECY-007: Gradual type checking
- DECY-008: Python HIR generation

**Debugger Work (Parallel)**:
- DECY-005-DEBUG: `spydecy debug visualize python-ast <file.py>`
- DECY-006-DEBUG: `spydecy debug show-type-hints <file.py>`
- DECY-008-DEBUG: `spydecy debug visualize python-hir <file.py>`

**Implementation**:
```rust
// spydecy-debugger/src/visualize/python.rs

pub struct PythonASTVisualizer;

impl PythonASTVisualizer {
    pub fn visualize(&self, ast: &PythonAST) -> String {
        // ASCII tree visualization
        self.render_tree(ast, 0)
    }

    fn render_tree(&self, node: &ASTNode, depth: usize) -> String {
        let indent = "  ".repeat(depth);
        let mut output = format!("{}├─ {}\n", indent, node.kind());

        for child in &node.children {
            output.push_str(&self.render_tree(child, depth + 1));
        }

        output
    }
}
```

**User Story**:
```bash
# Developer debugging Python parser issue
$ spydecy debug visualize python-ast test.py

Function(my_len)
  ├─ Parameters
  │   └─ x: inferred type List[T]
  └─ Body
      └─ Return
          └─ Call(len)
              └─ Var(x)
```

**Value**: Team can immediately verify Python parsing is working correctly.

---

### Sprint 3: C Transpiler + C Visualization

**Transpiler Work**:
- DECY-009: C parser (clang-sys)
- DECY-010: CPython API identification
- DECY-011: PyObject tracking
- DECY-012: C HIR generation

**Debugger Work (Parallel)**:
- DECY-009-DEBUG: `spydecy debug visualize c-ast <file.c>`
- DECY-010-DEBUG: `spydecy debug show-cpython-api-calls <file.c>`
- DECY-011-DEBUG: `spydecy debug track-pyobject <file.c>`

**Implementation**:
```rust
// spydecy-debugger/src/visualize/c.rs

pub struct CASTVisualizer;

impl CASTVisualizer {
    pub fn visualize_with_cpython_annotations(
        &self,
        ast: &CAST,
        cpython_info: &CPythonInfo,
    ) -> String {
        let mut output = String::new();

        for func in &ast.functions {
            output.push_str(&format!("Function: {}\n", func.name));

            // Annotate with CPython API usage
            for call in &cpython_info.api_calls {
                output.push_str(&format!(
                    "  ⚡ CPython API: {} (line {})\n",
                    call.name, call.location.line
                ));
            }

            // Show PyObject* tracking
            for pyobj in &cpython_info.pyobject_usage {
                output.push_str(&format!(
                    "  🐍 PyObject*: {} (refcount ops: {})\n",
                    pyobj.name, pyobj.refcount_ops.len()
                ));
            }
        }

        output
    }
}
```

**User Story**:
```bash
$ spydecy debug show-cpython-api-calls Objects/listobject.c

Function: list_length
  ⚡ CPython API: Py_SIZE (line 42)
  🐍 PyObject*: self (refcount ops: 0)

Function: PyList_Size
  ⚡ CPython API: PyList_Check (line 58)
  ⚡ CPython API: Py_SIZE (line 61)
  🐍 PyObject*: op (refcount ops: 1)
```

**Value**: Team can verify CPython API detection is working.

---

### Sprint 4: Unified HIR + Step-Through Debugger

**Transpiler Work**:
- DECY-013: Unified type system
- DECY-014: Cross-language references
- DECY-015: Metadata tracking
- DECY-016: HIR validation

**Debugger Work (Parallel)** - **MOST CRITICAL SPRINT**:
- DECY-013-DEBUG: `spydecy debug step --from python --to hir`
- DECY-014-DEBUG: `spydecy debug visualize unified-hir`
- DECY-016-DEBUG: `spydecy debug validate hir`

**Implementation**:
```rust
// spydecy-debugger/src/interactive/stepper.rs

pub struct TranspilationStepper {
    state: TranspilationState,
    breakpoints: Vec<Breakpoint>,
}

impl TranspilationStepper {
    pub fn start_session(&mut self, source_file: &Path) -> Result<()> {
        println!("Spydecy Interactive Debugger");
        println!("File: {}", source_file.display());
        println!("\nCommands: step, continue, break, inspect, visualize, quit");

        loop {
            print!("\n(spydecy-debug) ");
            io::stdout().flush()?;

            let command = self.read_command()?;

            match command {
                Command::Step => self.step()?,
                Command::Visualize => self.visualize_current_state()?,
                Command::Inspect(target) => self.inspect(target)?,
                Command::Break(breakpoint) => self.add_breakpoint(breakpoint),
                Command::Continue => self.continue_until_breakpoint()?,
                Command::Quit => break,
            }
        }

        Ok(())
    }

    fn step(&mut self) -> Result<()> {
        let next = self.state.next_transformation()?;

        println!("\n═══ Step {} ═══", self.state.step_count);
        println!("Transformation: {}", next.name());

        // Apply transformation
        self.state = next.apply(&self.state)?;
        self.state.step_count += 1;

        // Show what changed
        self.show_diff()?;

        Ok(())
    }

    fn visualize_current_state(&self) -> Result<()> {
        println!("\n═══ Current State (Step {}) ═══", self.state.step_count);

        // Show Python HIR
        if let Some(py_hir) = &self.state.python_hir {
            println!("\n📄 Python HIR:");
            println!("{}", PythonHIRVisualizer.visualize(py_hir));
        }

        // Show C HIR
        if let Some(c_hir) = &self.state.c_hir {
            println!("\n🔧 C HIR:");
            println!("{}", CHIRVisualizer.visualize(c_hir));
        }

        // Show Unified HIR
        if let Some(unified) = &self.state.unified_hir {
            println!("\n🔗 Unified HIR:");
            println!("{}", UnifiedHIRVisualizer.visualize(unified));
        }

        Ok(())
    }
}
```

**User Story**:
```bash
$ spydecy debug step --from python --to rust test_len.py

Spydecy Interactive Debugger
File: test_len.py

(spydecy-debug) step

═══ Step 1 ═══
Transformation: Parse Python to AST

📄 Python AST:
  Function(my_len)
    └─ Call(len)

(spydecy-debug) step

═══ Step 2 ═══
Transformation: Lower Python AST to Python HIR

📄 Python HIR:
  Function { name: "my_len", ... }

(spydecy-debug) step

═══ Step 3 ═══
Transformation: Identify CPython dependency (len → list_length)

🔧 Detected C dependency:
  Python: len() → C: list_length() in Objects/listobject.c

(spydecy-debug) step

═══ Step 4 ═══
Transformation: Parse CPython C implementation

🔧 C HIR:
  CFunction { name: "list_length", ... }

(spydecy-debug) step

═══ Step 5 ═══
Transformation: Unify Python HIR + C HIR

🔗 Unified HIR:
  CrossLanguageCall {
    python_caller: my_len,
    c_implementation: list_length,
    boundary: ForeignFunctionInterface
  }

(spydecy-debug) visualize

═══ Current State (Step 5) ═══

📄 Python HIR:
  my_len(x) → len(x)

🔧 C HIR:
  list_length(self) → Py_SIZE(self)

🔗 Unified HIR:
  my_len(x: Vec<T>) → x.len()  ✨ (boundary eliminated)
```

**Value**: This is the **killer feature**. Team can step through the entire transpilation process, seeing exactly how Python + C unify into Rust.

---

### Sprint 6: Optimizer + Breakpoint System

**Transpiler Work**:
- DECY-021: Boundary elimination
- DECY-022: Cross-language inlining
- DECY-023: Ownership unification
- DECY-024: Refcount optimization

**Debugger Work (Parallel)**:
- DECY-021-DEBUG: `spydecy debug break --on boundary-elimination`
- DECY-022-DEBUG: `spydecy debug diff --before-after-optimization`
- DECY-024-DEBUG: `spydecy debug show-refcount-ops`

**Implementation**:
```rust
pub enum Breakpoint {
    // Break when optimizer eliminates a boundary
    BoundaryElimination,

    // Break when inlining a cross-language call
    CrossLanguageInlining { function: String },

    // Break when converting PyObject* to safe Rust
    UnsafeToSafe,

    // Break on refcount optimization
    RefCountOptimization,
}
```

**User Story**:
```bash
$ spydecy debug test_len.py --break-on boundary-elimination

🛑 Breakpoint hit: BoundaryElimination

Before:
  my_len(x) → FFI_call(list_length, x)

After:
  my_len(x) → x.len()  // Inlined!

Eliminated: Foreign function interface overhead
Unsafe blocks removed: 2
Performance improvement: ~100x
```

**Value**: Team can verify optimizer is working correctly.

---

### Sprints 7-10: Progressive Feature Addition

As the transpiler gains more features, add corresponding debugger commands:

**Sprint 7 (CPython Optimization)**:
- `spydecy debug show-gil-operations`
- `spydecy debug suggest-gil-elimination`

**Sprint 8 (Memory Optimization)**:
- `spydecy debug show-allocations`
- `spydecy debug suggest-stack-allocation`

**Sprint 9 (Performance Validation)**:
- `spydecy debug benchmark --compare python`
- `spydecy debug profile`

**Sprint 10 (Safety Verification)**:
- `spydecy debug count-unsafe-blocks`
- `spydecy debug suggest-safe-alternatives`

---

### Sprints 11-15: Advanced Debugger Features

By this point, the basic debugger is mature and battle-tested. Now add advanced features:

**Sprint 11**:
- Graphical visualization (web UI)
- Export to DOT/Graphviz

**Sprint 12**:
- Record/replay transpilation sessions
- Regression testing

**Sprint 13**:
- AI-powered issue detection
- Pattern-based recommendations

**Sprint 14**:
- Integration with VS Code
- Language server protocol (LSP)

**Sprint 15**:
- MCP server mode for Claude Code
- Interactive REPL

---

## Debugger Architecture

### Core Abstractions

```rust
// spydecy-debugger/src/lib.rs

pub struct Debugger {
    // Transpilation state
    state: TranspilationState,

    // Visualization engines
    visualizers: VisualizerRegistry,

    // Breakpoint system
    breakpoints: BreakpointManager,

    // Issue detection
    issue_detector: IssueDetector,

    // Command processor
    repl: REPL,
}

pub struct TranspilationState {
    pub source: SourceCode,
    pub python_ast: Option<PythonAST>,
    pub python_hir: Option<PythonHIR>,
    pub c_ast: Option<CAST>,
    pub c_hir: Option<CHIR>,
    pub unified_hir: Option<UnifiedHIR>,
    pub optimized_hir: Option<UnifiedHIR>,
    pub rust_code: Option<RustCode>,
    pub step_count: usize,
    pub history: Vec<Transformation>,
}

pub struct VisualizerRegistry {
    visualizers: HashMap<String, Box<dyn Visualizer>>,
}

pub trait Visualizer {
    fn visualize(&self, data: &dyn Any) -> String;
}
```

### Plugin Architecture

Allow third-party debugger extensions:

```rust
#[plugin]
pub struct NumPyDebugger;

impl DebuggerPlugin for NumPyDebugger {
    fn commands(&self) -> Vec<Command> {
        vec![
            Command::new("show-numpy-arrays", show_numpy_arrays),
            Command::new("visualize-ndarray", visualize_ndarray),
        ]
    }

    fn visualizers(&self) -> Vec<Box<dyn Visualizer>> {
        vec![
            Box::new(NumpyArrayVisualizer),
            Box::new(NDArrayVisualizer),
        ]
    }
}
```

---

## Success Metrics

### Sprint 2-4 (Early Validation)
- ✅ Team uses debugger daily for transpiler development
- ✅ Debugger catches ≥1 bug per sprint
- ✅ Visualizations are clear and helpful

### Sprint 5-10 (Maturity)
- ✅ Debugger accelerates development by ≥50%
- ✅ All major transpiler features have corresponding debug commands
- ✅ Debugger has ≥80% test coverage

### Sprint 11-15 (Production Readiness)
- ✅ Debugger used by external beta testers
- ✅ Documentation complete
- ✅ Integration with external tools (VS Code, Claude Code)

---

## Key Benefits

### For the Spydecy Team
1. **Faster Development**: Debug transpiler issues 10x faster
2. **Higher Quality**: Catch bugs immediately, not weeks later
3. **Better Understanding**: Visualizations clarify complex transformations
4. **Natural Prioritization**: Build what you need when you need it

### For End Users
1. **Battle-Tested**: Tool has been used extensively by the dev team
2. **Intuitive**: Designed by developers who understand the workflow
3. **Complete**: All features validated through real-world use
4. **Well-Documented**: Documentation written while features were fresh

---

## Comparison: Original vs. Incremental Approach

| Aspect | Original (Sprints 11-15) | Incremental (Sprints 2-15) |
|--------|--------------------------|----------------------------|
| **Debugger Available** | Week 22+ | Week 4+ |
| **Team Productivity** | Low (weeks 1-20) | High (all sprints) |
| **Debugger Quality** | Unproven | Battle-tested |
| **User Experience** | Theoretical | Validated daily |
| **Risk** | High | Low |
| **Development Speed** | Slow | Fast |

---

## Conclusion

Building the debugger incrementally, alongside the transpiler, embodies the Toyota Way principle of **Jidoka** (automation with a human touch). It eliminates *mura* (unevenness) and *muri* (overburden), creating a smooth, continuous development process.

The debugger becomes both:
1. **A development tool** - Accelerating transpiler development
2. **A user-facing product** - Battle-tested and intuitive

This is the essence of "eating your own dog food" - building tools that serve both the developers and the users, ensuring quality through continuous, real-world validation.

---

**Status**: ROADMAP COMPLETE
**Next Step**: Begin Sprint 2 with parallel debugger development
**Expected Impact**: 50-100% increase in development velocity
