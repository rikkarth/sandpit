CARGO ?= cargo
MAKEFLAGS += --no-print-directory

.DEFAULT_GOAL := help

.PHONY: help build fmt fmt-check lint test check clean

help: ## Show available targets
	@grep -hE '^[a-zA-Z_-]+:.*?## ' $(MAKEFILE_LIST) \
		| awk 'BEGIN {FS = ":.*?## "}; {printf "  %-12s %s\n", $$1, $$2}'

build: ## Compile all targets
	$(CARGO) build --all-targets

fmt: ## Format the workspace in place
	$(CARGO) fmt

fmt-check: ## Fail if formatting is off
	$(CARGO) fmt --check

lint: ## Clippy with warnings denied
	$(CARGO) clippy --all-targets -- -D warnings

test: ## Run all tests
	$(CARGO) test --all-targets

check: fmt-check lint test ## Full quality gate: what CI runs
	@echo "check: ok"

clean: ## Remove build output
	$(CARGO) clean
