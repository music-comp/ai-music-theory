# Makefile for the mcp-server Project

# ANSI color codes
BLUE := \033[1;34m
GREEN := \033[1;32m
YELLOW := \033[1;33m
RED := \033[1;31m
CYAN := \033[1;36m
RESET := \033[0m

# Variables
PROJECT_NAME := mcp-server
BIN_DIR := ./bin
MODE := debug
MODES := debug release
TARGET := ./target/$(MODE)
GIT_COMMIT := $(shell git rev-parse --short HEAD 2>/dev/null || echo "unknown")
GIT_BRANCH := $(shell git rev-parse --abbrev-ref HEAD 2>/dev/null || echo "unknown")
BUILD_TIME := $(shell date -u '+%Y-%m-%dT%H:%M:%SZ')
RUST_VERSION := $(shell rustc --version 2>/dev/null || echo "unknown")

# List of binaries to build and install
BINARIES := music-theory-mcp

# Default target
.DEFAULT_GOAL := help

# Help target
.PHONY: help
help:
	@echo ""
	@echo "$(CYAN)╔══════════════════════════════════════════════════════════╗$(RESET)"
	@echo "$(CYAN)║$(RESET) $(BLUE)$(PROJECT_NAME) Build System$(RESET)                                  $(CYAN)║$(RESET)"
	@echo "$(CYAN)╚══════════════════════════════════════════════════════════╝$(RESET)"
	@echo ""
	@echo "$(GREEN)Building:$(RESET)"
	@echo "  $(YELLOW)make build$(RESET)            - Build all binaries (simple search)"
	@echo "  $(YELLOW)make build-fts$(RESET)        - Build with full-text search feature"
	@echo "  $(YELLOW)make build-graph$(RESET)      - Build with graph database feature"
	@echo "  $(YELLOW)make build-fts-graph$(RESET)  - Build with FTS + graph"
	@echo "  $(YELLOW)make build-full$(RESET)       - Build with all features (FTS + graph + vector + http)"
	@echo "  $(YELLOW)make build-release$(RESET)    - Build optimized release binaries (simple)"
	@echo "  $(YELLOW)make build-release-fts$(RESET) - Build optimized release binaries (with FTS)"
	@echo "  $(YELLOW)make build-release-graph$(RESET) - Build optimized release (with graph)"
	@echo "  $(YELLOW)make build-release-fts-graph$(RESET) - Build optimized (FTS + graph)"
	@echo "  $(YELLOW)make build-release-full$(RESET) - Build optimized (all features, recommended)"
	@echo "  $(YELLOW)make build-all$(RESET)        - Build all configurations in release mode"
	@echo "  $(YELLOW)make build MODE=release$(RESET) - Build with custom mode"
	@echo ""
	@echo "$(GREEN)Testing & Quality:$(RESET)"
	@echo "  $(YELLOW)make test$(RESET)             - Run all tests (without features)"
	@echo "  $(YELLOW)make test-fts$(RESET)         - Run all tests with FTS feature"
	@echo "  $(YELLOW)make test-graph$(RESET)       - Run all tests with graph feature"
	@echo "  $(YELLOW)make test-fts-graph$(RESET)   - Run all tests with FTS + graph"
	@echo "  $(YELLOW)make test-full$(RESET)        - Run all tests with all features"
	@echo "  $(YELLOW)make test-all$(RESET)         - Run tests in all configurations"
	@echo "  $(YELLOW)make lint$(RESET)             - Run clippy and format check (all configs)"
	@echo "  $(YELLOW)make format$(RESET)           - Format all code with rustfmt"
	@echo "  $(YELLOW)make coverage$(RESET)         - Generate test coverage report"
	@echo "  $(YELLOW)make check$(RESET)            - Build + lint + test (simple)"
	@echo "  $(YELLOW)make check-fts$(RESET)        - Build + lint + test (FTS)"
	@echo "  $(YELLOW)make check-graph$(RESET)      - Build + lint + test (graph)"
	@echo "  $(YELLOW)make check-fts-graph$(RESET)  - Build + lint + test (FTS + graph)"
	@echo "  $(YELLOW)make check-all$(RESET)        - Build + lint + test (all configs)"
	@echo ""
	@echo "$(GREEN)Cleaning:$(RESET)"
	@echo "  $(YELLOW)make clean$(RESET)            - Clean bin directory"
	@echo "  $(YELLOW)make clean-all$(RESET)        - Full clean (cargo clean)"
	@echo ""
	@echo "$(GREEN)Utilities:$(RESET)"
	@echo "  $(YELLOW)make push$(RESET)             - Pushes to Codeberg and Github"
	@echo "  $(YELLOW)make publish$(RESET)          - WIP: Publishes all crates to crates.io"
	@echo "  $(YELLOW)make tracked-files$(RESET)    - Save list of tracked files"
	@echo "  $(YELLOW)make size-compare$(RESET)     - Compare binary sizes (all configs)"
	@echo ""
	@echo "$(GREEN)CLI Tools - FTS (requires --features fts):$(RESET)"
	@echo "  $(YELLOW)make index$(RESET)            - Build FTS index"
	@echo "  $(YELLOW)make index-force$(RESET)      - Force rebuild FTS index"
	@echo "  $(YELLOW)make fts-status$(RESET)       - Show FTS index status"
	@echo ""
	@echo "$(GREEN)CLI Tools - Graph (requires --features graph):$(RESET)"
	@echo "  $(YELLOW)make graph-build$(RESET)      - Build concept graph database"
	@echo "  $(YELLOW)make graph-validate$(RESET)   - Validate graph integrity"
	@echo "  $(YELLOW)make graph-stats$(RESET)      - Show graph statistics"
	@echo ""
	@echo "$(GREEN)Information:$(RESET)"
	@echo "  $(YELLOW)make info$(RESET)             - Show build information"
	@echo "  $(YELLOW)make check-tools$(RESET)      - Verify required tools are installed"
	@echo ""
	@echo "$(CYAN)Current status:$(RESET) Branch: $(GIT_BRANCH) | Commit: $(GIT_COMMIT)"
	@echo ""

# Info target
.PHONY: info
info:
	@echo ""
	@echo "$(CYAN)╔══════════════════════════════════════════════════════════╗$(RESET)"
	@echo "$(CYAN)║$(RESET)  $(BLUE)Build Information$(RESET)                                       $(CYAN)║$(RESET)"
	@echo "$(CYAN)╚══════════════════════════════════════════════════════════╝$(RESET)"
	@echo ""
	@echo "$(GREEN)Project:$(RESET)"
	@echo "  Name:           $(PROJECT_NAME)"
	@echo "  Build Mode:     $(MODE)"
	@echo "  Build Time:     $(BUILD_TIME)"
	@echo ""
	@echo "$(GREEN)Paths:$(RESET)"
	@echo "  Binary Dir:     $(BIN_DIR)/"
	@echo "  Target Dir:     $(TARGET)/"
	@echo "  Workspace:      $$(pwd)"
	@echo ""
	@echo "$(GREEN)Git:$(RESET)"
	@echo "  Branch:         $(GIT_BRANCH)"
	@echo "  Commit:         $(GIT_COMMIT)"
	@echo ""
	@echo "$(GREEN)Tools:$(RESET)"
	@echo "  Rust:           $(RUST_VERSION)"
	@echo "  Cargo:          $$(cargo --version 2>/dev/null || echo 'not found')"
	@echo "  Rustfmt:        $$(rustfmt --version 2>/dev/null || echo 'not found')"
	@echo "  Clippy:         $$(cargo clippy --version 2>/dev/null || echo 'not found')"
	@echo ""
	@echo "$(GREEN)Binaries:$(RESET)"
	@for bin in $(BINARIES); do \
		if [ -f $(BIN_DIR)/$$bin ]; then \
			echo "  $$bin:          $(GREEN)✓ installed$(RESET)"; \
		else \
			echo "  $$bin:          $(RED)✗ not built$(RESET)"; \
		fi; \
	done
	@echo ""

# Check tools target
.PHONY: check-tools
check-tools:
	@echo "$(BLUE)Checking for required tools...$(RESET)"
	@command -v rustc >/dev/null 2>&1 && echo "$(GREEN)✓ rustc found (version: $$(rustc --version))$(RESET)" || echo "$(RED)✗ rustc not found$(RESET)"
	@command -v cargo >/dev/null 2>&1 && echo "$(GREEN)✓ cargo found (version: $$(cargo --version))$(RESET)" || echo "$(RED)✗ cargo not found$(RESET)"
	@command -v rustfmt >/dev/null 2>&1 && echo "$(GREEN)✓ rustfmt found$(RESET)" || echo "$(RED)✗ rustfmt not found (install: rustup component add rustfmt)$(RESET)"
	@cargo clippy --version >/dev/null 2>&1 && echo "$(GREEN)✓ clippy found$(RESET)" || echo "$(RED)✗ clippy not found (install: rustup component add clippy)$(RESET)"
	@cargo llvm-cov --version >/dev/null 2>&1 && echo "$(GREEN)✓ llvm-cov found$(RESET)" || echo "$(RED)✗ llvm-cov not found (install: cargo install cargo-llvm-cov)$(RESET)"
	@command -v git >/dev/null 2>&1 && echo "$(GREEN)✓ git found$(RESET)" || echo "$(RED)✗ git not found$(RESET)"
	@test -f Cargo.toml && echo "$(GREEN)✓ Cargo.toml found$(RESET)" || echo "$(RED)✗ Cargo.toml not found$(RESET)"

# Build directory creation
$(BIN_DIR):
	@echo "$(BLUE)Creating bin directory...$(RESET)"
	@mkdir -p $(BIN_DIR)
	@echo "$(GREEN)✓ Directory created$(RESET)"

# Build targets
.PHONY: build
build: clean $(BIN_DIR)
	@echo "$(BLUE)Building $(PROJECT_NAME) in $(MODE) mode with all features...$(RESET)"
	@echo "$(CYAN)• Compiling workspace with --features full (fts,graph,vector,http)...$(RESET)"
	@if [ "$(MODE)" = "release" ]; then \
		cargo build --release --features full; \
	else \
		cargo build --features full; \
	fi
	@echo "$(CYAN)• Copying binaries to $(BIN_DIR)/$(RESET)"
	@for bin in $(BINARIES); do \
		if [ -f $(TARGET)/$$bin ]; then \
			cp $(TARGET)/$$bin $(BIN_DIR)/$$bin; \
			echo "  $(GREEN)✓$(RESET) $$bin"; \
		else \
			echo "  $(YELLOW)⚠$(RESET) $$bin not found, skipping"; \
		fi; \
	done
	@echo "$(GREEN)✓ Build complete$(RESET)"
	@echo "$(CYAN)→ Binaries available in $(BIN_DIR)/$(RESET)"

.PHONY: build-fts
build-fts: clean $(BIN_DIR)
	@echo "$(BLUE)Building $(PROJECT_NAME) with FTS feature in $(MODE) mode...$(RESET)"
	@echo "$(CYAN)• Compiling workspace with --features fts...$(RESET)"
	@if [ "$(MODE)" = "release" ]; then \
		cargo build --release --features fts; \
	else \
		cargo build --features fts; \
	fi
	@echo "$(CYAN)• Copying binaries to $(BIN_DIR)/$(RESET)"
	@for bin in $(BINARIES); do \
		if [ -f $(TARGET)/$$bin ]; then \
			cp $(TARGET)/$$bin $(BIN_DIR)/$$bin; \
			echo "  $(GREEN)✓$(RESET) $$bin (with FTS)"; \
		else \
			echo "  $(YELLOW)⚠$(RESET) $$bin not found, skipping"; \
		fi; \
	done
	@echo "$(GREEN)✓ Build complete (FTS enabled)$(RESET)"
	@echo "$(CYAN)→ Binaries available in $(BIN_DIR)/$(RESET)"

.PHONY: build-release
build-release: MODE = release
build-release: TARGET = ./target/$(MODE)
build-release: ODM_TARGET = $(OXUR_WORKSPACE)/target/$(MODE)
build-release: clean $(BIN_DIR)
	@echo "$(BLUE)Building $(PROJECT_NAME) in release mode with all features...$(RESET)"
	@echo "$(CYAN)• Compiling optimized workspace with --features full (fts,graph,vector,http)...$(RESET)"
	@cargo build --release --features full
	@echo "$(CYAN)• Copying binaries to $(BIN_DIR)/$(RESET)"
	@for bin in $(BINARIES); do \
		if [ -f $(TARGET)/$$bin ]; then \
			cp $(TARGET)/$$bin $(BIN_DIR)/$$bin; \
			echo "  $(GREEN)✓$(RESET) $$bin (size: $$(du -h $(BIN_DIR)/$$bin | cut -f1))"; \
		else \
			echo "  $(YELLOW)⚠$(RESET) $$bin not found, skipping"; \
		fi; \
	done
	@echo "$(GREEN)✓ Release build complete$(RESET)"
	@echo "$(CYAN)→ Optimized binaries in $(BIN_DIR)/$(RESET)"

.PHONY: build-release-fts
build-release-fts: MODE = release
build-release-fts: TARGET = ./target/$(MODE)
build-release-fts: clean $(BIN_DIR)
	@echo "$(BLUE)Building $(PROJECT_NAME) in release mode with FTS...$(RESET)"
	@echo "$(CYAN)• Compiling optimized workspace with --features fts...$(RESET)"
	@cargo build --release --features fts
	@echo "$(CYAN)• Copying binaries to $(BIN_DIR)/$(RESET)"
	@for bin in $(BINARIES); do \
		if [ -f $(TARGET)/$$bin ]; then \
			cp $(TARGET)/$$bin $(BIN_DIR)/$$bin; \
			echo "  $(GREEN)✓$(RESET) $$bin (with FTS, size: $$(du -h $(BIN_DIR)/$$bin | cut -f1))"; \
		else \
			echo "  $(YELLOW)⚠$(RESET) $$bin not found, skipping"; \
		fi; \
	done
	@echo "$(GREEN)✓ Release build complete (FTS enabled)$(RESET)"
	@echo "$(CYAN)→ Optimized binaries in $(BIN_DIR)/$(RESET)"

.PHONY: build-graph
build-graph: clean $(BIN_DIR)
	@echo "$(BLUE)Building $(PROJECT_NAME) with graph feature in $(MODE) mode...$(RESET)"
	@echo "$(CYAN)• Compiling workspace with --features graph...$(RESET)"
	@if [ "$(MODE)" = "release" ]; then \
		cargo build --release --features graph; \
	else \
		cargo build --features graph; \
	fi
	@echo "$(CYAN)• Copying binaries to $(BIN_DIR)/$(RESET)"
	@for bin in $(BINARIES); do \
		if [ -f $(TARGET)/$$bin ]; then \
			cp $(TARGET)/$$bin $(BIN_DIR)/$$bin; \
			echo "  $(GREEN)✓$(RESET) $$bin (with graph)"; \
		else \
			echo "  $(YELLOW)⚠$(RESET) $$bin not found, skipping"; \
		fi; \
	done
	@echo "$(GREEN)✓ Build complete (graph enabled)$(RESET)"
	@echo "$(CYAN)→ Binaries available in $(BIN_DIR)/$(RESET)"

.PHONY: build-release-graph
build-release-graph: MODE = release
build-release-graph: TARGET = ./target/$(MODE)
build-release-graph: clean $(BIN_DIR)
	@echo "$(BLUE)Building $(PROJECT_NAME) in release mode with graph...$(RESET)"
	@echo "$(CYAN)• Compiling optimized workspace with --features graph...$(RESET)"
	@cargo build --release --features graph
	@echo "$(CYAN)• Copying binaries to $(BIN_DIR)/$(RESET)"
	@for bin in $(BINARIES); do \
		if [ -f $(TARGET)/$$bin ]; then \
			cp $(TARGET)/$$bin $(BIN_DIR)/$$bin; \
			echo "  $(GREEN)✓$(RESET) $$bin (with graph, size: $$(du -h $(BIN_DIR)/$$bin | cut -f1))"; \
		else \
			echo "  $(YELLOW)⚠$(RESET) $$bin not found, skipping"; \
		fi; \
	done
	@echo "$(GREEN)✓ Release build complete (graph enabled)$(RESET)"
	@echo "$(CYAN)→ Optimized binaries in $(BIN_DIR)/$(RESET)"

.PHONY: build-fts-graph
build-fts-graph: clean $(BIN_DIR)
	@echo "$(BLUE)Building $(PROJECT_NAME) with FTS + graph features in $(MODE) mode...$(RESET)"
	@echo "$(CYAN)• Compiling workspace with --features fts,graph...$(RESET)"
	@if [ "$(MODE)" = "release" ]; then \
		cargo build --release --features fts,graph; \
	else \
		cargo build --features fts,graph; \
	fi
	@echo "$(CYAN)• Copying binaries to $(BIN_DIR)/$(RESET)"
	@for bin in $(BINARIES); do \
		if [ -f $(TARGET)/$$bin ]; then \
			cp $(TARGET)/$$bin $(BIN_DIR)/$$bin; \
			echo "  $(GREEN)✓$(RESET) $$bin (with FTS + graph)"; \
		else \
			echo "  $(YELLOW)⚠$(RESET) $$bin not found, skipping"; \
		fi; \
	done
	@echo "$(GREEN)✓ Build complete (FTS + graph enabled)$(RESET)"
	@echo "$(CYAN)→ Binaries available in $(BIN_DIR)/$(RESET)"

.PHONY: build-release-fts-graph
build-release-fts-graph: MODE = release
build-release-fts-graph: TARGET = ./target/$(MODE)
build-release-fts-graph: clean $(BIN_DIR)
	@echo "$(BLUE)Building $(PROJECT_NAME) in release mode with FTS + graph...$(RESET)"
	@echo "$(CYAN)• Compiling optimized workspace with --features fts,graph...$(RESET)"
	@cargo build --release --features fts,graph
	@echo "$(CYAN)• Copying binaries to $(BIN_DIR)/$(RESET)"
	@for bin in $(BINARIES); do \
		if [ -f $(TARGET)/$$bin ]; then \
			cp $(TARGET)/$$bin $(BIN_DIR)/$$bin; \
			echo "  $(GREEN)✓$(RESET) $$bin (with FTS + graph, size: $$(du -h $(BIN_DIR)/$$bin | cut -f1))"; \
		else \
			echo "  $(YELLOW)⚠$(RESET) $$bin not found, skipping"; \
		fi; \
	done
	@echo "$(GREEN)✓ Release build complete (FTS + graph enabled)$(RESET)"
	@echo "$(CYAN)→ Optimized binaries in $(BIN_DIR)/$(RESET)"

.PHONY: build-full
build-full: clean $(BIN_DIR)
	@echo "$(BLUE)Building $(PROJECT_NAME) with all features in $(MODE) mode...$(RESET)"
	@echo "$(CYAN)• Compiling workspace with --features full (fts,graph,vector,http)...$(RESET)"
	@if [ "$(MODE)" = "release" ]; then \
		cargo build --release --features full; \
	else \
		cargo build --features full; \
	fi
	@echo "$(CYAN)• Copying binaries to $(BIN_DIR)/$(RESET)"
	@for bin in $(BINARIES); do \
		if [ -f $(TARGET)/$$bin ]; then \
			cp $(TARGET)/$$bin $(BIN_DIR)/$$bin; \
			echo "  $(GREEN)✓$(RESET) $$bin (full: FTS + graph + vector + http)"; \
		else \
			echo "  $(YELLOW)⚠$(RESET) $$bin not found, skipping"; \
		fi; \
	done
	@echo "$(GREEN)✓ Build complete (all features enabled)$(RESET)"
	@echo "$(CYAN)→ Binaries available in $(BIN_DIR)/$(RESET)"

.PHONY: build-release-full
build-release-full: MODE = release
build-release-full: TARGET = ./target/$(MODE)
build-release-full: clean $(BIN_DIR)
	@echo "$(BLUE)Building $(PROJECT_NAME) in release mode with all features...$(RESET)"
	@echo "$(CYAN)• Compiling optimized workspace with --features full (fts,graph,vector,http)...$(RESET)"
	@cargo build --release --features full
	@echo "$(CYAN)• Copying binaries to $(BIN_DIR)/$(RESET)"
	@for bin in $(BINARIES); do \
		if [ -f $(TARGET)/$$bin ]; then \
			cp $(TARGET)/$$bin $(BIN_DIR)/$$bin; \
			echo "  $(GREEN)✓$(RESET) $$bin (full, size: $$(du -h $(BIN_DIR)/$$bin | cut -f1))"; \
		else \
			echo "  $(YELLOW)⚠$(RESET) $$bin not found, skipping"; \
		fi; \
	done
	@echo "$(GREEN)✓ Release build complete (all features enabled)$(RESET)"
	@echo "$(CYAN)→ Optimized binaries in $(BIN_DIR)/$(RESET)"

docs: DOCS_PATH = target/doc/$(PROJECT_NAME)
docs:
	@echo "$(BLUE)Building $(PROJECT_NAME) docs...$(RESET)"
	@cargo doc --all-features --no-deps --workspace
	@echo
	@echo "$(CYAN)• Docs are available here:$(RESET)"
	@echo "   • $(DOCS_PATH)"
	@echo "   • file://$(shell pwd)/$(DOCS_PATH)/index.html"
	@echo

open-docs:
	@cargo doc --all-features --no-deps --workspace --open

# Common checks
.PHONY: common-checks
common-checks: check-deps lint build

# Combined check targets
.PHONY: check
check: common-checks test
	@echo ""
	@echo "$(GREEN)✓ All checks passed (build + lint + test + cli-validation)$(RESET)"
	@echo ""

# Ensure cargo-binstall is available for fast tool installation
.PHONY: ensure-binstall
ensure-binstall:
	@command -v cargo-binstall >/dev/null 2>&1 || { \
		echo "$(YELLOW)→ Installing cargo-binstall...$(RESET)"; \
		curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash; \
	}

.PHONY: check-deps
check-deps: ensure-binstall
	@echo "$(BLUE)Checking for outdated dependencies...$(RESET)"
	@command -v cargo-outdated >/dev/null 2>&1 || { \
		echo "$(YELLOW)→ Installing cargo-outdated...$(RESET)"; \
		cargo binstall -y cargo-outdated; \
	}
	@OUTPUT=$$(cargo outdated --root-deps-only 2>&1); EXITCODE=$$?; \
	if [ $$EXITCODE -ne 0 ]; then \
		if echo "$$OUTPUT" | grep -q -e "failed to load" -e "no entry found"; then \
			echo "$(YELLOW)⚠ Skipping outdated check (cargo-outdated cannot resolve git dependencies)$(RESET)"; \
			exit 0; \
		else \
			echo "$$OUTPUT"; \
			exit 1; \
		fi; \
	fi; \
	echo "$$OUTPUT"; \
	echo ""; \
	if echo "$$OUTPUT" | grep -E "^[a-z0-9_-]+\s+" | grep -v "^----" | awk '{print $$3}' | grep -v "^---$$" | grep -v "^Compat$$" | grep -E "^[0-9]" | grep -q .; then \
		echo "$(RED)✗ Compatible dependency updates available$(RESET)"; \
		echo "$(YELLOW)→ Run 'make deps' to update and commit the updated Cargo.lock$(RESET)"; \
		exit 1; \
	else \
		echo "$(GREEN)✓ All dependencies up to date$(RESET)"; \
	fi


# Cleaning targets
.PHONY: clean
clean:
	@echo "$(BLUE)Cleaning bin directory...$(RESET)"
	@rm -rf $(BIN_DIR)
	@echo "$(GREEN)✓ Clean complete$(RESET)"

.PHONY: clean-all
clean-all: clean
	@echo "$(BLUE)Performing full cargo clean...$(RESET)"
	@cargo clean
	@echo "$(GREEN)✓ Full clean complete$(RESET)"

# Testing & Quality targets
.PHONY: test
test:
	@echo "$(BLUE)Running tests (without FTS)...$(RESET)"
	@echo "$(CYAN)• Running all workspace tests...$(RESET)"
	@cargo test --workspace
	@echo "$(GREEN)✓ All tests passed$(RESET)"

.PHONY: test-fts
test-fts:
	@echo "$(BLUE)Running tests with FTS feature...$(RESET)"
	@echo "$(CYAN)• Running all workspace tests with --features fts...$(RESET)"
	@cargo test --features fts --workspace
	@echo "$(GREEN)✓ All tests passed (FTS enabled)$(RESET)"

.PHONY: test-graph
test-graph:
	@echo "$(BLUE)Running tests with graph feature...$(RESET)"
	@echo "$(CYAN)• Running all workspace tests with --features graph...$(RESET)"
	@cargo test --features graph --workspace
	@echo "$(GREEN)✓ All tests passed (graph enabled)$(RESET)"

.PHONY: test-fts-graph
test-fts-graph:
	@echo "$(BLUE)Running tests with FTS + graph features...$(RESET)"
	@echo "$(CYAN)• Running all workspace tests with --features fts,graph...$(RESET)"
	@cargo test --features fts,graph --workspace
	@echo "$(GREEN)✓ All tests passed (FTS + graph enabled)$(RESET)"

.PHONY: test-full
test-full:
	@echo "$(BLUE)Running tests with all features...$(RESET)"
	@echo "$(CYAN)• Running all workspace tests with --features full...$(RESET)"
	@cargo test --features full --workspace
	@echo "$(GREEN)✓ All tests passed (all features enabled)$(RESET)"

# Feature configurations to test across modes
FEATURE_CONFIGS := none fts graph fts,graph full

.PHONY: test-all
test-all:
	@echo "$(BLUE)Running tests in all modes and feature configurations...$(RESET)"
	@for mode in $(MODES); do \
		echo ""; \
		echo "$(CYAN)═══ Mode: $$mode ═══$(RESET)"; \
		RELEASE_FLAG=""; \
		if [ "$$mode" = "release" ]; then RELEASE_FLAG="--release"; fi; \
		for features in $(FEATURE_CONFIGS); do \
			if [ "$$features" = "none" ]; then \
				echo "$(CYAN)• [$$mode] Without features$(RESET)"; \
				cargo test --workspace $$RELEASE_FLAG; \
			else \
				echo "$(CYAN)• [$$mode] With $$features$(RESET)"; \
				cargo test --features $$features --workspace $$RELEASE_FLAG; \
			fi; \
		done; \
	done
	@echo ""
	@echo "$(GREEN)✓ All tests passed in all modes and configurations$(RESET)"

.PHONY: lint
lint:
	@echo "$(BLUE)Running linter checks (all configurations)...$(RESET)"
	@echo "$(CYAN)• Running clippy without features...$(RESET)"
	@cargo clippy --workspace -- -D warnings
	@echo "$(GREEN)✓ Clippy passed (without features)$(RESET)"
	@echo "$(CYAN)• Running clippy with FTS...$(RESET)"
	@cargo clippy --features fts --workspace -- -D warnings
	@echo "$(GREEN)✓ Clippy passed (with FTS)$(RESET)"
	@echo "$(CYAN)• Running clippy with graph...$(RESET)"
	@cargo clippy --features graph --workspace -- -D warnings
	@echo "$(GREEN)✓ Clippy passed (with graph)$(RESET)"
	@echo "$(CYAN)• Running clippy with FTS + graph...$(RESET)"
	@cargo clippy --features fts,graph --workspace -- -D warnings
	@echo "$(GREEN)✓ Clippy passed (with FTS + graph)$(RESET)"
	@echo "$(CYAN)• Running clippy with all features...$(RESET)"
	@cargo clippy --features full --workspace -- -D warnings
	@echo "$(GREEN)✓ Clippy passed (with all features)$(RESET)"
	@echo "$(CYAN)• Checking code formatting...$(RESET)"
	@cargo fmt --all -- --check
	@echo "$(GREEN)✓ Format check passed$(RESET)"

.PHONY: format
format:
	@echo "$(BLUE)Formatting code...$(RESET)"
	@echo "$(CYAN)• Running rustfmt on all files...$(RESET)"
	@cargo fmt --all
	@echo "$(GREEN)✓ Code formatted$(RESET)"

.PHONY: coverage
coverage:
	@echo "$(BLUE)Generating test coverage report (with all features)...$(RESET)"
	@echo "$(CYAN)• Running tests with coverage (server crate only)...$(RESET)"
	@cargo llvm-cov --features full --package music-theory-mcp
	@echo "$(GREEN)✓ Coverage report generated$(RESET)"
	@echo "$(YELLOW)→ For detailed HTML report, run: make coverage-html$(RESET)"

.PHONY: coverage-html
coverage-html:
	@echo "$(BLUE)Generating HTML coverage report (with all features)...$(RESET)"
	@echo "$(CYAN)• Running tests with coverage (server crate only)...$(RESET)"
	@cargo llvm-cov --html --features full --package music-theory-mcp
	@echo "$(GREEN)✓ HTML coverage report generated$(RESET)"
	@echo "$(CYAN)→ Report: target/llvm-cov/html/index.html$(RESET)"

.PHONY: deps
deps: ensure-binstall
	@echo "$(BLUE)Updating dependencies ...$(RESET)"
	@command -v cargo-upgrade >/dev/null 2>&1 || { \
		echo "$(YELLOW)→ Installing cargo-edit...$(RESET)"; \
		cargo binstall -y cargo-edit; \
	}
	@cargo upgrade
	@echo "$(GREEN)✓ Cargo deps upgraded$(RESET)"

.PHONY: check-graph
check-graph:
	@echo "$(BLUE)Running checks for graph configuration...$(RESET)"
	@echo "$(CYAN)• Building with graph...$(RESET)"
	@cargo build --features graph
	@echo "$(CYAN)• Running clippy with graph...$(RESET)"
	@cargo clippy --features graph --workspace -- -D warnings
	@echo "$(CYAN)• Running tests with graph...$(RESET)"
	@cargo test --features graph --workspace
	@echo ""
	@echo "$(GREEN)✓ All graph checks passed$(RESET)"
	@echo ""

.PHONY: check-fts-graph
check-fts-graph:
	@echo "$(BLUE)Running checks for FTS + graph configuration...$(RESET)"
	@echo "$(CYAN)• Building with FTS + graph...$(RESET)"
	@cargo build --features fts,graph
	@echo "$(CYAN)• Running clippy with FTS + graph...$(RESET)"
	@cargo clippy --features fts,graph --workspace -- -D warnings
	@echo "$(CYAN)• Running tests with FTS + graph...$(RESET)"
	@cargo test --features fts,graph --workspace
	@echo ""
	@echo "$(GREEN)✓ All FTS + graph checks passed$(RESET)"
	@echo ""

.PHONY: check-all
check-all:
	@echo "$(BLUE)Running comprehensive validation (all configurations)...$(RESET)"
	@echo ""
	@echo "$(CYAN)═══ Configuration 1: Simple Search ═══$(RESET)"
	@$(MAKE) common-checks test
	@echo ""
	@echo "$(CYAN)═══ Configuration 2: With FTS ═══$(RESET)"
	@$(MAKE) check-fts
	@echo ""
	@echo "$(CYAN)═══ Configuration 3: With Graph ═══$(RESET)"
	@$(MAKE) check-graph
	@echo ""
	@echo "$(CYAN)═══ Configuration 4: With FTS + Graph ═══$(RESET)"
	@$(MAKE) check-fts-graph
	@echo ""
	@echo "$(GREEN)✓ Full validation complete (all configurations)$(RESET)"
	@echo ""

# Utility targets
.PHONY: tracked-files
tracked-files:
	@echo "$(BLUE)Saving tracked files list...$(RESET)"
	@mkdir -p $(TARGET)
	@git ls-files > $(TARGET)/git-tracked-files.txt
	@echo "$(GREEN)✓ Tracked files saved to $(TARGET)/git-tracked-files.txt$(RESET)"
	@echo "$(CYAN)• Total files: $$(wc -l < $(TARGET)/git-tracked-files.txt)$(RESET)"

push:
	@echo "$(BLUE)Pushing changes ...$(RESET)"
	@echo "$(CYAN)• Codeberg:$(RESET)"
	@git push codeberg main && git push codeberg --tags
	@echo "$(GREEN)✓ Pushed$(RESET)"
	@echo "$(CYAN)• Github:$(RESET)"
	@git push github main && git push github --tags
	@echo "$(GREEN)✓ Pushed$(RESET)"

# Build all configurations for comparison
.PHONY: build-all
build-all: MODE = release
build-all: TARGET = ./target/$(MODE)
build-all: clean $(BIN_DIR)
	@echo "$(BLUE)Building all configurations in release mode...$(RESET)"
	@echo ""
	@echo "$(CYAN)═══ Configuration 1: Simple Search ═══$(RESET)"
	@cargo build --release
	@mkdir -p $(BIN_DIR)/simple
	@for bin in $(BINARIES); do \
		if [ -f $(TARGET)/$$bin ]; then \
			cp $(TARGET)/$$bin $(BIN_DIR)/simple/$$bin; \
			echo "  $(GREEN)✓$(RESET) $$bin (simple, size: $$(du -h $(BIN_DIR)/simple/$$bin | cut -f1))"; \
		fi; \
	done
	@echo ""
	@echo "$(CYAN)═══ Configuration 2: With FTS ═══$(RESET)"
	@cargo build --release --features fts
	@mkdir -p $(BIN_DIR)/fts
	@for bin in $(BINARIES); do \
		if [ -f $(TARGET)/$$bin ]; then \
			cp $(TARGET)/$$bin $(BIN_DIR)/fts/$$bin; \
			echo "  $(GREEN)✓$(RESET) $$bin (FTS, size: $$(du -h $(BIN_DIR)/fts/$$bin | cut -f1))"; \
		fi; \
	done
	@echo ""
	@echo "$(CYAN)═══ Configuration 3: With FTS + Graph ═══$(RESET)"
	@cargo build --release --features fts,graph
	@mkdir -p $(BIN_DIR)/all
	@for bin in $(BINARIES); do \
		if [ -f $(TARGET)/$$bin ]; then \
			cp $(TARGET)/$$bin $(BIN_DIR)/all/$$bin; \
			echo "  $(GREEN)✓$(RESET) $$bin (FTS + graph, size: $$(du -h $(BIN_DIR)/all/$$bin | cut -f1))"; \
		fi; \
	done
	@echo ""
	@echo "$(GREEN)✓ All configurations built$(RESET)"
	@echo "$(CYAN)→ Simple binaries in $(BIN_DIR)/simple/$(RESET)"
	@echo "$(CYAN)→ FTS binaries in $(BIN_DIR)/fts/$(RESET)"
	@echo "$(CYAN)→ FTS + graph binaries in $(BIN_DIR)/all/$(RESET)"
	@echo "$(YELLOW)→ Run 'make size-compare' to see size comparison$(RESET)"

# Compare binary sizes across all configurations
.PHONY: size-compare
size-compare:
	@echo ""
	@echo "$(CYAN)╔══════════════════════════════════════════════════════════╗$(RESET)"
	@echo "$(CYAN)║$(RESET) $(BLUE)Binary Size Comparison$(RESET)                                   $(CYAN)║$(RESET)"
	@echo "$(CYAN)╚══════════════════════════════════════════════════════════╝$(RESET)"
	@echo ""
	@if [ ! -d "$(BIN_DIR)/simple" ] || [ ! -d "$(BIN_DIR)/fts" ] || [ ! -d "$(BIN_DIR)/all" ]; then \
		echo "$(RED)✗ Build all configurations first:$(RESET) make build-all"; \
		exit 1; \
	fi
	@for bin in $(BINARIES); do \
		if [ -f "$(BIN_DIR)/simple/$$bin" ] && [ -f "$(BIN_DIR)/fts/$$bin" ] && [ -f "$(BIN_DIR)/all/$$bin" ]; then \
			SIMPLE_SIZE=$$(stat -f%z "$(BIN_DIR)/simple/$$bin" 2>/dev/null || stat -c%s "$(BIN_DIR)/simple/$$bin" 2>/dev/null); \
			FTS_SIZE=$$(stat -f%z "$(BIN_DIR)/fts/$$bin" 2>/dev/null || stat -c%s "$(BIN_DIR)/fts/$$bin" 2>/dev/null); \
			ALL_SIZE=$$(stat -f%z "$(BIN_DIR)/all/$$bin" 2>/dev/null || stat -c%s "$(BIN_DIR)/all/$$bin" 2>/dev/null); \
			SIMPLE_SIZE_MB=$$(echo "scale=1; $$SIMPLE_SIZE / 1024 / 1024" | bc); \
			FTS_SIZE_MB=$$(echo "scale=1; $$FTS_SIZE / 1024 / 1024" | bc); \
			ALL_SIZE_MB=$$(echo "scale=1; $$ALL_SIZE / 1024 / 1024" | bc); \
			FTS_DIFF=$$(echo "scale=1; $$FTS_SIZE_MB - $$SIMPLE_SIZE_MB" | bc); \
			ALL_DIFF=$$(echo "scale=1; $$ALL_SIZE_MB - $$SIMPLE_SIZE_MB" | bc); \
			FTS_PCT=$$(echo "scale=0; ($$FTS_SIZE - $$SIMPLE_SIZE) * 100 / $$SIMPLE_SIZE" | bc); \
			ALL_PCT=$$(echo "scale=0; ($$ALL_SIZE - $$SIMPLE_SIZE) * 100 / $$SIMPLE_SIZE" | bc); \
			echo "$(GREEN)$$bin:$(RESET)"; \
			echo "  Simple:      $${SIMPLE_SIZE_MB}M"; \
			echo "  FTS:         $${FTS_SIZE_MB}M  $(YELLOW)(+$${FTS_DIFF}M, +$${FTS_PCT}%)$(RESET)"; \
			echo "  FTS + graph: $${ALL_SIZE_MB}M  $(YELLOW)(+$${ALL_DIFF}M, +$${ALL_PCT}%)$(RESET)"; \
			echo ""; \
		fi; \
	done

# CLI tool helpers (require FTS feature)
.PHONY: index
index:
	@echo "$(BLUE)Building FTS index...$(RESET)"
	@if [ ! -f "$(BIN_DIR)/music-theory-mcp" ]; then \
		echo "$(YELLOW)→ Building binary with FTS feature first...$(RESET)"; \
		$(MAKE) build-fts MODE=release; \
	fi
	@$(BIN_DIR)/music-theory-mcp index
	@echo "$(GREEN)✓ Index build complete$(RESET)"

.PHONY: index-force
index-force:
	@echo "$(BLUE)Force rebuilding FTS index...$(RESET)"
	@if [ ! -f "$(BIN_DIR)/music-theory-mcp" ]; then \
		echo "$(YELLOW)→ Building binary with FTS feature first...$(RESET)"; \
		$(MAKE) build-fts MODE=release; \
	fi
	@$(BIN_DIR)/music-theory-mcp index --force
	@echo "$(GREEN)✓ Index rebuild complete$(RESET)"

.PHONY: fts-status
fts-status:
	@echo "$(BLUE)FTS index status:$(RESET)"
	@echo ""
	@if [ ! -f "$(BIN_DIR)/music-theory-mcp" ]; then \
		echo "$(YELLOW)→ Building binary with FTS feature first...$(RESET)"; \
		$(MAKE) build-fts MODE=release; \
	fi
	@$(BIN_DIR)/music-theory-mcp status

# Backward compatibility alias
.PHONY: status
status: fts-status

# Graph CLI tool helpers (require graph feature)
.PHONY: graph-build
graph-build:
	@echo "$(BLUE)Building concept graph database...$(RESET)"
	@if [ ! -f "$(BIN_DIR)/music-theory-mcp" ]; then \
		echo "$(YELLOW)→ Building binary with graph feature first...$(RESET)"; \
		$(MAKE) build-fts-graph MODE=release; \
	fi
	@$(BIN_DIR)/music-theory-mcp graph build
	@echo "$(GREEN)✓ Graph build complete$(RESET)"

.PHONY: graph-build-verbose
graph-build-verbose:
	@echo "$(BLUE)Building concept graph database (verbose)...$(RESET)"
	@if [ ! -f "$(BIN_DIR)/music-theory-mcp" ]; then \
		echo "$(YELLOW)→ Building binary with graph feature first...$(RESET)"; \
		$(MAKE) build-fts-graph MODE=release; \
	fi
	@$(BIN_DIR)/music-theory-mcp graph build --verbose
	@echo "$(GREEN)✓ Graph build complete$(RESET)"

.PHONY: graph-validate
graph-validate:
	@echo "$(BLUE)Validating concept graph...$(RESET)"
	@echo ""
	@if [ ! -f "$(BIN_DIR)/music-theory-mcp" ]; then \
		echo "$(YELLOW)→ Building binary with graph feature first...$(RESET)"; \
		$(MAKE) build-fts-graph MODE=release; \
	fi
	@$(BIN_DIR)/music-theory-mcp graph validate

.PHONY: graph-stats
graph-stats:
	@echo "$(BLUE)Graph statistics:$(RESET)"
	@echo ""
	@if [ ! -f "$(BIN_DIR)/music-theory-mcp" ]; then \
		echo "$(YELLOW)→ Building binary with graph feature first...$(RESET)"; \
		$(MAKE) build-fts-graph MODE=release; \
	fi
	@$(BIN_DIR)/music-theory-mcp graph stats

# Check target for FTS configuration
.PHONY: check-fts
check-fts:
	@echo "$(BLUE)Running checks for FTS configuration...$(RESET)"
	@echo "$(CYAN)• Building with FTS...$(RESET)"
	@cargo build --features fts
	@echo "$(CYAN)• Running clippy with FTS...$(RESET)"
	@cargo clippy --features fts --workspace -- -D warnings
	@echo "$(CYAN)• Running tests with FTS...$(RESET)"
	@cargo test --features fts --workspace
	@echo ""
	@echo "$(GREEN)✓ All FTS checks passed$(RESET)"
	@echo ""
