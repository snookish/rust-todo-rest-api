.DEFAULT_GOAL := help
DIESEL ?= diesel
MYSQL_CLIENT_PREFIX ?= $(shell brew --prefix mysql-client 2>/dev/null)

ifneq ($(wildcard $(MYSQL_CLIENT_PREFIX)/lib/pkgconfig),)
export PKG_CONFIG_PATH := $(MYSQL_CLIENT_PREFIX)/lib/pkgconfig$(if $(PKG_CONFIG_PATH),:$(PKG_CONFIG_PATH))
export PATH := $(MYSQL_CLIENT_PREFIX)/bin:$(PATH)
endif

.PHONY: help build release run test check fmt fmt-check lint clean doc db-create db-drop db-setup db-migrate db-migrate-prod db-rollback db-redo db-reset db-status db-schema db-migration

help: ## Show this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2}'

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

db-create: ## Create the database from DATABASE_URL
	$(DIESEL) database create

db-drop: ## Drop the database (destructive)
	$(DIESEL) database drop

db-setup: ## Create the database, run migrations and generate the schema
	$(DIESEL) setup --no-default-migration

db-migrate: ## Run pending migrations and regenerate src/db/schema.rs
	$(DIESEL) migration run

db-migrate-prod: ## Run pending migrations in production with a locked schema
	$(DIESEL) migration run --locked-schema

db-rollback: ## Revert the most recently applied migration
	$(DIESEL) migration revert

db-redo: ## Revert and re-apply the most recently applied migration
	$(DIESEL) migration redo

db-reset: ## Drop, recreate and migrate the database (destructive)
	$(DIESEL) database reset

db-status: ## List migrations and whether they have been applied
	$(DIESEL) migration list

db-schema: ## Regenerate src/db/schema.rs from the database
	$(DIESEL) print-schema > src/db/schema.rs

db-migration: ## Generate a migration (usage: make db-migration name=add_users)
	@test -n "$(name)" || (echo "Usage: make db-migration name=<migration_name>" && exit 1)
	$(DIESEL) migration generate $(name)