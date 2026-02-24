.PHONY: help build build-all test clean bump-patch bump-minor bump-major install

# Variables
BINARY_NAME=agesmith
VERSION=$(shell grep '^version' Cargo.toml | head -1 | cut -d'"' -f2)
BUILD_DIR=dist
TARGETS=x86_64-unknown-linux-gnu x86_64-apple-darwin aarch64-apple-darwin x86_64-pc-windows-gnu

help: ## Show this help message
	@echo "AgeSmith - Makefile commands"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

build: ## Build optimized release binary for current platform (target/release/)
	@echo "Building $(BINARY_NAME) v$(VERSION)..."
	cargo build --release
	@echo "Binary created at: target/release/$(BINARY_NAME)"

build-all: ## Build release binaries for all platforms: Linux, macOS (Intel/ARM), Windows (requires cross)
	@echo "Building $(BINARY_NAME) v$(VERSION) for all platforms..."
	@mkdir -p $(BUILD_DIR)
	@for target in $(TARGETS); do \
		echo "Building for $$target..."; \
		cross build --release --target $$target 2>/dev/null || cargo build --release --target $$target; \
		if [ $$? -eq 0 ]; then \
			if echo $$target | grep -q windows; then \
				cp target/$$target/release/$(BINARY_NAME).exe $(BUILD_DIR)/$(BINARY_NAME)-$(VERSION)-$$target.exe 2>/dev/null || true; \
			else \
				cp target/$$target/release/$(BINARY_NAME) $(BUILD_DIR)/$(BINARY_NAME)-$(VERSION)-$$target 2>/dev/null || true; \
			fi; \
		fi; \
	done
	@echo "Binaries created in $(BUILD_DIR)/"
	@ls -lh $(BUILD_DIR)/

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

release: build-all ## Build release binaries for all platforms, generate checksums, and display info
	@echo ""
	@echo "=========================================="
	@echo "Release build completed!"
	@echo "Version: $(VERSION)"
	@echo "=========================================="
	@echo ""
	@echo "Generating checksums..."
	@cd $(BUILD_DIR) && shasum -a 256 * > checksums.txt 2>/dev/null || true
	@echo ""
	@echo "Binaries available in $(BUILD_DIR)/:"
	@ls -lh $(BUILD_DIR)/
	@echo ""
	@echo "Checksums saved to $(BUILD_DIR)/checksums.txt"
	@cat $(BUILD_DIR)/checksums.txt 2>/dev/null || echo "No checksums generated"
