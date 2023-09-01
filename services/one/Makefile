.Phony: run
run:
	@echo "Running service one"
	@go run cmd/todo/*.go

.Phony: build
build:
	@echo "Running service one"
	@go run cmd/todo/main.go -o bin/todo-service

.Phony: watch
watch:
	@echo "Running service one"
	@find . -name "*.go" | entr go run cmd/todo/main.go
