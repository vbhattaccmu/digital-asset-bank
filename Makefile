.DEFAULT_GOAL := start_service

start_service: pull_images setup_service

pull_images:
	@echo "[Pulling images] Pulling Docker images..."
	@docker compose --file service.yaml pull

setup_service:
	@echo "[Setting up environment] Starting services and initializing database..."
	@docker compose --file service.yaml up -d
	@echo "[Database start up] Starting postgres database.."
	docker exec -d service_backend bash -c "service postgresql start"
	@until docker exec service_backend pg_isready -h localhost -U postgres -d postgres -t 1; do sleep 1; done
	docker exec -d service_backend sudo -u postgres psql -c "ALTER USER postgres PASSWORD 'postgres';"
	docker exec -d service_backend sudo -u postgres psql -c "CREATE DATABASE backend;"

	@echo "Service Binary started. Please use the postman collection/front end to use the endpoints or you can use the test suite by running make service_tests on the terminal."
	
	@make service_tests
		
	@echo "[Start binary] Starting service binary.."
	docker exec -d service_backend bash -c "/home/target/debug/submission --config-path /home/service/example_configuration_file/config.toml --start-anew"

service_tests:
	@echo "[Tests] Running tests..container service_backend should be up and running if need to run this make command separately.."
	docker exec service_backend bash -c "/root/.cargo/bin/cargo test -- --test-threads=3"

service_clean_up: 
	@echo "[Service Clean Up] Cleaning up artifacts..Please wait.."
	@docker compose --file service.yaml down
	@docker rmi vbhattac453/rust_backend:latest
	@docker rmi vbhattac453/frontend:latest

help:
	@echo "Available targets:"
	@echo "  start_service      - Set up environment, start services, and run the service binary."
	@echo "  service_tests      - Run tests on the service."
	@echo "  service_clean_up   - Clean up artifacts and containers."
