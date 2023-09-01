.Phony: start
start:
	@echo "Starting the services..."
	@docker compose up

.Phony: build
build:
	@echo "Starting the services..."
	@docker compose build

.PHONY: watch-todos-app
watch-todos-app:
	@echo "Watching the application..."
	@find services/one | entr -s "docker compose restart todos-app"
