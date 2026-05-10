dev:
	docker compose -f docker-compose.dev.yml up -d

watch:
	cd server && cargo watch -x run -w crates/

stop:
	docker compose -f docker-compose.dev.yml down

migrate:
	cd server && cargo sqlx migrate run
