.PHONY: help build build-all test clean bump-patch bump-minor bump-major install

# Variables
BINARY_NAME=agesmith
VERSION=$(shell grep '^version' Cargo.toml | head -1 | cut -d'"' -f2)
BUILD_DIR=dist
TARGETS=x86_64-unknown-linux-gnu x86_64-apple-darwin aarch64-apple-darwin

help: ## Show this help message
	@echo "AgeSmith - Makefile commands"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

build: ## Build optimized release binary for current platform (target/release/)
	@echo "Building $(BINARY_NAME) v$(VERSION)..."
	cargo build --release
	@echo "Binary created at: target/release/$(BINARY_NAME)"

build-all: ## Build release binaries for Linux and macOS (Intel/ARM) - requires cross and Docker
build-all: ## Build release binaries for Linux and macOS using cross (requires Docker)
	@./scripts/build-release.sh

test: ## Run all tests (integration and unit tests)
	@echo "Running tests..."
	cargo test
	@echo "Tests completed!"

test-verbose: ## Run all tests with full output (useful for debugging test failures)
	@echo "Running tests (verbose)..."
	cargo test -- --nocapture

check: ## Run cargo check and clippy to verify code quality without building
	@echo "Checking code..."
	cargo check
	cargo clippy -- -D warnings

fmt: ## Format all Rust code using rustfmt (auto-fix code style)
	@echo "Formatting code..."
	cargo fmt

clean: ## Remove all build artifacts (target/ and dist/ directories)
	@echo "Cleaning build artifacts..."
	cargo clean
	rm -rf $(BUILD_DIR)
	@echo "Clean completed!"

bump-patch: ## Bump patch version (0.0.X) - for bug fixes (requires cargo-edit)
	@echo "Current version: $(VERSION)"
	@cargo install cargo-edit 2>/dev/null || true
	cargo set-version --bump patch
	@echo "New version: $$(grep '^version' Cargo.toml | head -1 | cut -d'\"' -f2)"

bump-minor: ## Bump minor version (0.X.0) - for new features (requires cargo-edit)
	@echo "Current version: $(VERSION)"
	@cargo install cargo-edit 2>/dev/null || true
	cargo set-version --bump minor
	@echo "New version: $$(grep '^version' Cargo.toml | head -1 | cut -d'\"' -f2)"

bump-major: ## Bump major version (X.0.0) - for breaking changes (requires cargo-edit)
	@echo "Current version: $(VERSION)"
	@cargo install cargo-edit 2>/dev/null || true
	cargo set-version --bump major
	@echo "New version: $$(grep '^version' Cargo.toml | head -1 | cut -d'\"' -f2)"

install: ## Install binary to system (~/.cargo/bin/) for global use
	@echo "Installing $(BINARY_NAME)..."
	cargo install --path .
	@echo "$(BINARY_NAME) installed successfully!"

dev: ## Run application in development mode (unoptimized, faster compilation)
	@echo "Running $(BINARY_NAME) in dev mode..."
	cargo run

release: build-all ## Build release binaries for all platforms and display info

release-patch: ## Create a patch release (0.0.X) - bump, build, tag, and push
	@./scripts/release.sh patch

release-minor: ## Create a minor release (0.X.0) - bump, build, tag, and push
	@./scripts/release.sh minor

release-major: ## Create a major release (X.0.0) - bump, build, tag, and push
	@./scripts/release.sh major
