.Phony: run
run:
	@echo "Running news app"
	@gunicorn -b 0.0.0.0:4003 'app:server'

.Phony: run-dev
run-dev:
	@echo "Running news app"
	@python -m app
