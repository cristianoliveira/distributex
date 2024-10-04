.PHONY: help
help: ## Lists the available commands. Add a comment with '##' to describe a command.
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST)\
		| sort\
		| awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

.PHONY: start
start: ## Starts all the services.
	@echo "Starting the services..."
	@docker compose up

.PHONY: rebuild-start
rebuild-start: ## Rebuilds and starts all the services.
	@echo "rebuilding the services..."
	@docker compose up --build

.PHONY: build
build: ## Builds all the services.
	@echo "Building the services..."
	@docker compose build

.PHONY: watch
watch: ## Starts watcher that does hot-reload for docker when something changes.
	@echo "Watching the application..."
	@funzzy --config=.watch.yaml

.PHONY: kill
kill: ## Kills all the services.
	@echo "Killing all applications..."
	@docker compose kill
