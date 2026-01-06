#!make
PORT=8080
SERVICE_NAME = gsn_push_processing
CONTAINER_NAME = $(SERVICE_NAME)
DOCKER_COMPOSE_TAG = $(SERVICE_NAME)

# DB commands
db-create: ## Create database
	sqlx database create

db-drop: ## Drop database
	sqlx database drop

migrate-up: ## Run all pending migrations
	sqlx migrate run

migrate-down: ## Revert last migration
	sqlx migrate revert

migrate-revert: ## Revert last N migrations (use N=2 for example)
	sqlx migrate revert --target-version $(N)

migrate-status: ## Show migration status
	sqlx migrate info

migration-new: ## Create new migration (use NAME=your_migration_name)
	sqlx migrate add -r $(NAME)

# format code
format:
	cargo fmt --verbose

lint:
	cargo clippy --fix

up:
	cargo run

# Docker commands
run-external-services:
	docker-compose -f ./docker-compose.inf.yml up --detach db

down:
	docker-compose -f ./docker-compose.inf.yml down --remove-orphans

down-rm:
	docker compose -f ./docker-compose.inf.yml down --remove-orphans --rmi all --volumes
