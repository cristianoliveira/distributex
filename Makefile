.Phony: start
start:
	@echo "Starting the services..."
	@docker compose up

.phony: rebuild-start
rebuild-start:
	@echo "rebuilding the services..."
	@docker compose up --build

.Phony: build
build:
	@echo "Building the services..."
	@docker compose build

.PHONY: watch
watch:
	@echo "Watching the application..."
	@funzzy --config=.watch.yaml

.PHONY: kill
kill:
	@echo "Killing all applications..."
	@docker compose kill
