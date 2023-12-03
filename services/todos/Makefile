.Phony: run
run:
	@echo "Running todo app"
	@go run cmd/todo/*.go

.Phony: build
build:
	@echo "Running todo app"
	@go run cmd/todo/main.go -o bin/todo-service

.Phony: watch
watch:
	@echo "Running todo app"
	@npx nodemon --exec 'make run' --signal SIGTERM
