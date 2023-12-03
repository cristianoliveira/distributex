.Phony: run
run:
	@echo "Running news app"
	@gunicorn -b 0.0.0.0:4003 'app:server'

.Phony: run-dev
run-dev:
	@echo "Running news app"
	@python -m app

.Phony: setup
setup:
	@echo "Setting up news app"
	@pip install -r requirements.txt
	# Check if sqlite3 is installed
	@sqlite3 --version || echo "ERROR: Please install sqlite3!"
