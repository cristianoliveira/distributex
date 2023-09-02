.Phony: start
start:
	@echo "Starting the services..."
	@docker compose up

.Phony: build
build:
	@echo "Starting the services..."
	@docker compose build

.PHONY: watch
watch:
	@echo "Watching the application..."
	@funzzy
