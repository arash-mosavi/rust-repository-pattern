# Makefile for development tasks

.PHONY: help build test clean run fmt clippy check dev docker-build docker-up docker-down

help: ## Show this help message
	@echo "Available commands:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'

build: ## Build all workspace crates
	cargo build --workspace

build-release: ## Build all workspace crates in release mode
	cargo build --workspace --release

test: ## Run tests for all workspace crates
	cargo test --workspace

clean: ## Clean build artifacts
	cargo clean

run: ## Run the HTTP server (apps/server)
	cargo run -p server

run-release: ## Run the HTTP server in release mode
	cargo run -p server --release

fmt: ## Format code using rustfmt
	cargo fmt --all

clippy: ## Run clippy lints
	cargo clippy --workspace --all-targets --all-features -- -D warnings

check: ## Run cargo check on all workspace crates
	cargo check --workspace --all-targets --all-features

dev: ## Run server in development mode with auto-reload (requires cargo-watch)
	cargo watch -x "run -p server"

install-dev-tools: ## Install development tools (cargo-watch, etc.)
	cargo install cargo-watch

# Docker commands
docker-build: ## Build Docker image
	docker build -t repository-pattern:latest -f deployment/Dockerfile .

docker-up: ## Start services with docker-compose
	docker-compose -f deployment/docker-compose.yml up -d

docker-down: ## Stop services
	docker-compose -f deployment/docker-compose.yml down

docker-logs: ## View docker logs
	docker-compose -f deployment/docker-compose.yml logs -f

# Database migrations
migrate-up: ## Run database migrations
	cargo run -p server -- migrate up

migrate-down: ## Rollback database migrations
	cargo run -p server -- migrate down

# Documentation
docs: ## Generate and open documentation
	cargo doc --workspace --no-deps --open

# Create new module template
new-module: ## Create a new module (usage: make new-module NAME=<module-name>)
	@if [ -z "$(NAME)" ]; then \
		echo "Error: NAME is required. Usage: make new-module NAME=<module-name>"; \
		exit 1; \
	fi
	@echo "Creating new module: $(NAME)"
	@mkdir -p crates/modules/$(NAME)/src/{constants,delivery/http/dto,domain/entities,repositories,service,types}
	@echo "Module $(NAME) created at crates/modules/$(NAME)"
