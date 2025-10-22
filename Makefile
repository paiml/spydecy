# Spydecy Makefile - EXTREME TDD Quality Gates
# Inspired by Toyota Way, bashrs, and ruchy
# Zero Tolerance for Technical Debt

SHELL := /bin/bash
.DEFAULT_GOAL := help

# Colors for output
BLUE := \033[0;34m
GREEN := \033[0;32m
YELLOW := \033[0;33m
RED := \033[0;31m
NC := \033[0m# No Color

# Project Configuration
PROJECT_NAME := spydecy
RUST_VERSION := 1.75.0
MIN_COVERAGE := 80

# Tool Check Helpers
CARGO := $(shell command -v cargo 2>/dev/null)
PMAT := $(shell command -v pmat 2>/dev/null)
MUTANTS := $(shell command -v cargo-mutants 2>/dev/null)
LLVM_COV := $(shell command -v cargo-llvm-cov 2>/dev/null)

.PHONY: help
help: ## Show this help message
	@echo "$(BLUE)Spydecy - EXTREME TDD Quality Gates$(NC)"
	@echo "$(BLUE)=====================================$(NC)"
	@echo ""
	@echo "$(GREEN)Quality Gates:$(NC)"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?## "}; /^## Quality/ {print "\n$(YELLOW)" $$2 "$(NC)"} /^[^#]/ {printf "  $(BLUE)%-25s$(NC) %s\n", $$1, $$2}'

## Installation & Setup

.PHONY: install-tools
install-tools: ## Install all required development tools
	@echo "$(BLUE)Installing development tools...$(NC)"
	@if ! command -v rustup >/dev/null 2>&1; then \
		echo "$(YELLOW)Installing Rust via rustup...$(NC)"; \
		curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y; \
	fi
	@echo "$(GREEN)Installing Rust components...$(NC)"
	rustup component add rustfmt clippy llvm-tools-preview
	@echo "$(GREEN)Installing cargo tools...$(NC)"
	cargo install cargo-llvm-cov cargo-mutants cargo-watch cargo-outdated cargo-audit
	@echo "$(GREEN)Installing PMAT (if not installed)...$(NC)"
	cargo install pmat || echo "$(YELLOW)PMAT installation failed - install manually$(NC)"
	@echo "$(GREEN)Installing mdbook (if not installed)...$(NC)"
	cargo install mdbook --vers "^0.4" --locked || echo "$(YELLOW)mdbook already installed$(NC)"
	@echo "$(GREEN)✅ All tools installed!$(NC)"

.PHONY: setup
setup: install-tools ## Complete project setup
	@echo "$(BLUE)Setting up Spydecy development environment...$(NC)"
	@mkdir -p .github/workflows docs/reports coverage
	@echo "$(GREEN)✅ Project setup complete!$(NC)"

## Build Targets

.PHONY: build
build: ## Build all crates in workspace
	@echo "$(BLUE)Building Spydecy...$(NC)"
	cargo build --workspace --all-features

.PHONY: build-release
build-release: ## Build optimized release binary
	@echo "$(BLUE)Building release binary...$(NC)"
	cargo build --release --workspace

.PHONY: clean
clean: ## Clean build artifacts
	@echo "$(BLUE)Cleaning build artifacts...$(NC)"
	cargo clean
	rm -rf coverage/ docs/reports/ .kaizen/ book/build/

## Book Documentation

.PHONY: book
book: ## Build the documentation book
	@echo "$(BLUE)Building documentation book...$(NC)"
	@if ! command -v mdbook >/dev/null 2>&1; then \
		echo "$(YELLOW)mdbook not found. Installing...$(NC)"; \
		cargo install mdbook --vers "^0.4" --locked; \
	fi
	mdbook build book
	@echo "$(GREEN)✅ Book built successfully!$(NC)"
	@echo "$(BLUE)Open book/build/html/index.html to view$(NC)"

.PHONY: book-test
book-test: ## Test code examples in the book (TDD-enforced)
	@echo "$(BLUE)Testing book code examples...$(NC)"
	@if ! command -v mdbook >/dev/null 2>&1; then \
		echo "$(YELLOW)mdbook not found. Installing...$(NC)"; \
		cargo install mdbook --vers "^0.4" --locked; \
	fi
	mdbook test book
	@echo "$(GREEN)✅ All book examples tested!$(NC)"

.PHONY: book-serve
book-serve: ## Serve the book locally for development
	@echo "$(BLUE)Serving book at http://localhost:3000$(NC)"
	mdbook serve book --open

.PHONY: book-watch
book-watch: ## Watch and rebuild book on changes
	@echo "$(BLUE)Watching book for changes...$(NC)"
	mdbook watch book

## Quality Gates

.PHONY: format
format: ## Format code with rustfmt
	@echo "$(BLUE)Formatting code...$(NC)"
	cargo fmt --all

.PHONY: format-check
format-check: ## Check code formatting
	@echo "$(BLUE)Checking code formatting...$(NC)"
	cargo fmt --all -- --check

.PHONY: lint
lint: ## Run Clippy lints
	@echo "$(BLUE)Running Clippy...$(NC)"
	cargo clippy --workspace --all-features --all-targets -- -D warnings

.PHONY: lint-fix
lint-fix: ## Auto-fix Clippy issues
	@echo "$(BLUE)Auto-fixing Clippy issues...$(NC)"
	cargo clippy --workspace --all-features --all-targets --fix --allow-dirty -- -D warnings

## Testing

.PHONY: test
test: ## Run all tests
	@echo "$(BLUE)Running tests...$(NC)"
	cargo test --workspace --all-features

.PHONY: test-fast
test-fast: ## Run tests without expensive property tests
	@echo "$(BLUE)Running fast tests...$(NC)"
	cargo test --workspace --lib --bins

.PHONY: test-property
test-property: ## Run property-based tests
	@echo "$(BLUE)Running property tests...$(NC)"
	PROPTEST_CASES=1000 cargo test --workspace --all-features -- --ignored proptest

.PHONY: test-doc
test-doc: ## Run documentation tests
	@echo "$(BLUE)Running doc tests...$(NC)"
	cargo test --workspace --doc

.PHONY: test-all
test-all: test test-property test-doc ## Run all test suites

## Benchmarking

.PHONY: bench
bench: ## Run performance benchmarks
	@echo "$(BLUE)Running benchmarks...$(NC)"
	cargo bench --workspace --no-fail-fast
	@echo "$(GREEN)✅ Benchmarks complete!$(NC)"
	@echo "$(BLUE)View results: target/criterion/report/index.html$(NC)"

.PHONY: bench-optimizer
bench-optimizer: ## Run optimizer benchmarks only
	@echo "$(BLUE)Running optimizer benchmarks...$(NC)"
	cargo bench --bench optimizer_benchmarks
	@echo "$(GREEN)✅ Optimizer benchmarks complete!$(NC)"

.PHONY: bench-unification
bench-unification: ## Run unification benchmarks only
	@echo "$(BLUE)Running unification benchmarks...$(NC)"
	cargo bench --bench unification_benchmarks
	@echo "$(GREEN)✅ Unification benchmarks complete!$(NC)"

.PHONY: bench-baseline
bench-baseline: ## Save current benchmarks as baseline
	@echo "$(BLUE)Saving baseline benchmarks...$(NC)"
	cargo bench --workspace --no-fail-fast -- --save-baseline main
	@echo "$(GREEN)✅ Baseline saved!$(NC)"

.PHONY: bench-compare
bench-compare: ## Compare benchmarks against baseline
	@echo "$(BLUE)Comparing against baseline...$(NC)"
	cargo bench --workspace --no-fail-fast -- --baseline main
	@echo "$(GREEN)✅ Comparison complete!$(NC)"

## Coverage

.PHONY: coverage
coverage: ## Generate test coverage report
	@echo "$(BLUE)Generating coverage report...$(NC)"
ifndef LLVM_COV
	@echo "$(RED)Error: cargo-llvm-cov not installed$(NC)"
	@echo "Run: cargo install cargo-llvm-cov"
	@exit 1
endif
	cargo llvm-cov --workspace --all-features --html --output-dir coverage
	cargo llvm-cov --workspace --all-features --summary-only
	@echo "$(GREEN)Coverage report: coverage/index.html$(NC)"

.PHONY: coverage-ci
coverage-ci: ## Generate coverage for CI (with JSON output)
	@echo "$(BLUE)Generating CI coverage...$(NC)"
	cargo llvm-cov --workspace --all-features --lcov --output-path coverage/lcov.info
	cargo llvm-cov --workspace --all-features --json --output-path coverage/coverage.json

.PHONY: coverage-check
coverage-check: coverage ## Check coverage meets minimum threshold
	@COVERAGE=$$(cargo llvm-cov --workspace --all-features --summary-only 2>/dev/null | \
		grep -oP 'TOTAL.*\K[0-9]+\.[0-9]+' | head -1); \
	echo "Current coverage: $$COVERAGE%"; \
	if [ $$(echo "$$COVERAGE < $(MIN_COVERAGE)" | bc) -eq 1 ]; then \
		echo "$(RED)❌ Coverage $$COVERAGE% is below threshold $(MIN_COVERAGE)%$(NC)"; \
		exit 1; \
	else \
		echo "$(GREEN)✅ Coverage $$COVERAGE% meets threshold$(NC)"; \
	fi

## Mutation Testing

.PHONY: mutants
mutants: ## Run mutation testing
	@echo "$(BLUE)Running mutation tests...$(NC)"
ifndef MUTANTS
	@echo "$(RED)Error: cargo-mutants not installed$(NC)"
	@echo "Run: cargo install cargo-mutants"
	@exit 1
endif
	cargo mutants --workspace --timeout 300 --in-diff || true
	@echo "$(YELLOW)Check mutants.out/ for detailed results$(NC)"

.PHONY: mutants-fast
mutants-fast: ## Run mutation tests on changed files only
	@echo "$(BLUE)Running fast mutation tests...$(NC)"
	cargo mutants --workspace --timeout 120 --in-diff --jobs 4

.PHONY: mutants-baseline
mutants-baseline: ## Establish mutation testing baseline
	@echo "$(BLUE)Establishing mutation testing baseline...$(NC)"
	cargo mutants --workspace --timeout 300 --baseline mutants-baseline.json

## PMAT Quality Analysis

.PHONY: pmat-check
pmat-check: ## Run PMAT quality analysis
	@echo "$(BLUE)Running PMAT quality analysis...$(NC)"
ifndef PMAT
	@echo "$(RED)Error: PMAT not installed$(NC)"
	@echo "Run: cargo install pmat"
	@exit 1
endif
	pmat analyze complexity --path . --fail-on-violation --max-cyclomatic 10
	pmat analyze satd --path . --fail-on-violation
	@echo "$(GREEN)✅ PMAT quality checks passed!$(NC)"

.PHONY: pmat-report
pmat-report: ## Generate PMAT quality report
	@echo "$(BLUE)Generating PMAT quality report...$(NC)"
	mkdir -p docs/reports
	pmat analyze complexity --path . --output docs/reports/complexity.json || true
	pmat analyze satd --path . --output docs/reports/satd.json || true
	@echo "$(GREEN)Reports generated in docs/reports/$(NC)"

## Quality Gate - Full Suite

.PHONY: quality-gate
quality-gate: format-check lint pmat-check test coverage-check ## Run full quality gate suite
	@echo "$(GREEN)╔═══════════════════════════════════════╗$(NC)"
	@echo "$(GREEN)║  ✅ ALL QUALITY GATES PASSED ✅      ║$(NC)"
	@echo "$(GREEN)╚═══════════════════════════════════════╝$(NC)"

.PHONY: quality-fast
quality-fast: format-check lint test-fast ## Run fast quality checks (no coverage/mutation)
	@echo "$(GREEN)✅ Fast quality checks passed!$(NC)"

.PHONY: pre-commit
pre-commit: format lint test-fast pmat-check book-test ## Pre-commit quality checks
	@echo "$(GREEN)✅ Pre-commit checks passed!$(NC)"

## Continuous Integration

.PHONY: ci
ci: quality-gate mutants-fast ## Full CI pipeline
	@echo "$(GREEN)✅ CI pipeline complete!$(NC)"

.PHONY: ci-fast
ci-fast: quality-fast ## Fast CI for rapid feedback
	@echo "$(GREEN)✅ Fast CI complete!$(NC)"

## Development

.PHONY: dev
dev: ## Start development mode with auto-reload
	@echo "$(BLUE)Starting development mode...$(NC)"
	cargo watch -x 'build --workspace' -x 'test --workspace --lib' -x 'clippy --workspace'

.PHONY: check
check: ## Quick sanity check (build + test)
	@echo "$(BLUE)Running sanity check...$(NC)"
	cargo check --workspace --all-features
	cargo test --workspace --lib

## Documentation

.PHONY: docs
docs: ## Build documentation
	@echo "$(BLUE)Building documentation...$(NC)"
	cargo doc --workspace --all-features --no-deps --document-private-items

.PHONY: docs-open
docs-open: docs ## Build and open documentation
	cargo doc --workspace --all-features --no-deps --open

## Benchmarks

.PHONY: bench
bench: ## Run benchmarks
	@echo "$(BLUE)Running benchmarks...$(NC)"
	cargo bench --workspace

.PHONY: bench-baseline
bench-baseline: ## Establish benchmark baseline
	@echo "$(BLUE)Establishing benchmark baseline...$(NC)"
	cargo bench --workspace -- --save-baseline baseline

## Security

.PHONY: audit
audit: ## Run security audit
	@echo "$(BLUE)Running security audit...$(NC)"
	cargo audit

.PHONY: outdated
outdated: ## Check for outdated dependencies
	@echo "$(BLUE)Checking for outdated dependencies...$(NC)"
	cargo outdated --workspace

## Kaizen - Continuous Improvement

.PHONY: kaizen
kaizen: ## Run Kaizen continuous improvement analysis
	@echo "$(BLUE)═══════════════════════════════════════════════════$(NC)"
	@echo "$(BLUE)    KAIZEN (改善) - Continuous Improvement          $(NC)"
	@echo "$(BLUE)═══════════════════════════════════════════════════$(NC)"
	@mkdir -p .kaizen
	@echo ""
	@echo "$(YELLOW)Step 1: Quality Metrics$(NC)"
	@make pmat-report || true
	@echo ""
	@echo "$(YELLOW)Step 2: Test Coverage$(NC)"
	@make coverage-ci || true
	@echo ""
	@echo "$(YELLOW)Step 3: Code Complexity$(NC)"
	@pmat analyze complexity . || true
	@echo ""
	@echo "$(YELLOW)Step 4: Technical Debt$(NC)"
	@pmat analyze satd . || true
	@echo ""
	@echo "$(YELLOW)Step 5: Security Audit$(NC)"
	@make audit || true
	@echo ""
	@echo "$(YELLOW)Step 6: Dependency Health$(NC)"
	@make outdated || true
	@echo ""
	@date '+%Y-%m-%d %H:%M:%S' > .kaizen/last-kaizen.txt
	@echo "$(GREEN)✅ Kaizen analysis complete$(NC)"
	@echo "$(GREEN)継続的改善 - Continuous Improvement$(NC)"

## Sprint 0 - Tracer Bullet

.PHONY: sprint0-tracer-bullet
sprint0-tracer-bullet: ## Run Sprint 0 tracer bullet validation
	@echo "$(BLUE)═══════════════════════════════════════════════════$(NC)"
	@echo "$(BLUE)    Sprint 0: Tracer Bullet Validation             $(NC)"
	@echo "$(BLUE)═══════════════════════════════════════════════════$(NC)"
	@echo ""
	@echo "$(YELLOW)This will validate the Unified HIR concept$(NC)"
	@echo "$(YELLOW)Target: Python len() → C list_length() → Rust Vec::len()$(NC)"
	@echo ""
	@echo "$(RED)Not yet implemented - requires Sprint 0 code$(NC)"

## Utility Targets

.PHONY: watch
watch: ## Watch for changes and run tests
	cargo watch -x 'test --workspace'

.PHONY: watch-test
watch-test: ## Watch and run specific test
	@read -p "Test name: " test; \
	cargo watch -x "test $$test"

.PHONY: install
install: build-release ## Install spydecy binary
	@echo "$(BLUE)Installing spydecy...$(NC)"
	cargo install --path .

.PHONY: uninstall
uninstall: ## Uninstall spydecy binary
	cargo uninstall spydecy

.PHONY: version
version: ## Show version information
	@echo "Spydecy version: $$(cargo pkgid | cut -d# -f2)"
	@echo "Rust version: $$(rustc --version)"
	@cargo --version

.PHONY: tree
tree: ## Show dependency tree
	cargo tree --workspace

## Validation

.PHONY: validate
validate: quality-gate ## Full validation suite (alias for quality-gate)
	@echo "$(GREEN)✅ Validation complete!$(NC)"

.PHONY: validate-fast
validate-fast: quality-fast ## Fast validation (alias for quality-fast)
	@echo "$(GREEN)✅ Fast validation complete!$(NC)"

.PHONY: all
all: clean build test quality-gate ## Clean, build, test, and validate
	@echo "$(GREEN)✅ Full build cycle complete!$(NC)"
