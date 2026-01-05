#!make
PORT=8080
SERVICE_NAME = gsn_push_processing
CONTAINER_NAME = $(SERVICE_NAME)
DOCKER_COMPOSE_TAG = $(SERVICE_NAME)

# format code
format:
	cargo fmt --verbose

lint:
	cargo clippy --fix

up:
	cargo run

