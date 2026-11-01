.DEFAULT_GOAL := help

.PHONY: help build release run test check fmt fmt-check lint clean doc

help: ## Show this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-12s\033[0m %s\n", $$1, $$2}'

build: ## Build the project
	cargo build

release: ## Build an optimized release binary
	cargo build --release

run: ## Run the server
	cargo run

test: ## Run the test suite
	cargo test

check: ## Type-check without producing a binary
	cargo check --all-targets

fmt: ## Format the code
	cargo fmt --all

fmt-check: ## Verify formatting without changing files
	cargo fmt --all -- --check

lint: ## Run clippy and treat warnings as errors
	cargo clippy --all-targets --all-features -- -D warnings

clean: ## Remove build artifacts
	cargo clean

doc: ## Build and open the documentation
	cargo doc --no-deps --open
