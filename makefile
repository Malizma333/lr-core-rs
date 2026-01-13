.DEFAULT_GOAL: help

.PHONY: help
help: ## Show this help message
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

.PHONY: install
install: ## Install development tooling locally at .cargo
	# Test coverage
	cargo install cargo-tarpaulin --version 0.35.0 --locked --root .cargo

.PHONY: new
new: ## Create a new library crate
	@if [ -n "$(CRATE)" ]; then \
		cargo init --lib --vcs none $(CRATE); \
	else \
		echo "Usage: make new CRATE=[name]"; \
	fi

.PHONY: doc
doc: ## Create documentation
	@if [ -z $(CRATE) ]; then\
		cargo doc --workspace;\
	else\
		cargo doc -p $(CRATE);\
	fi

.PHONY: format
format: ## Format files with rustfmt
	cargo fmt --all

.PHONY: lint
lint: ## Lint files with clippy
	cargo clippy --all-targets --all-features -- -Dwarnings

.PHONY: test
test: ## Run tests
	@if [ -z $(CRATE) ]; then\
		cargo test --workspace;\
	else\
		cargo test -p $(CRATE) -- --no-capture;\
	fi

.PHONY: coverage
coverage: ## Run coverage on tests, with report outputted to target/coverage
	@if [ -z $(CRATE) ]; then\
		.cargo/bin/cargo-tarpaulin --workspace -o Html --output-dir target/coverage;\
	else\
		.cargo/bin/cargo-tarpaulin -p $(CRATE) -o Html --output-dir target/coverage;\
	fi

.PHONY: benchmark
benchmark: ## Run benchmarks, with report outputted to target/criterion
	@if [ -z $(CRATE) ]; then\
		cargo bench --workspace;\
	else\
		cargo bench -p $(CRATE);\
	fi

.PHONY: run
run: ## Run the lr_studio application
	cargo run