# docker commands --------
build:
	docker build -t todo-app:latest .

up:
	docker run \
	-p 8080:8080 \
	-e DATABASE_URL=postgres://postgres:postgres@localhost:5432/todo_db \
	todo-app:latest

# Example curl commands to test the API ----- 
post:
	curl -X POST http://localhost:8080/api/todos \
	-H "Content-Type: application/json" \
	-d '{"title": "Test Todo", "description": "This is a test todo"}'

get:
	curl -X GET http://localhost:8080/api/todos

# database migration --------
up-migrate:
	sqlx migrate run

down-migrate:
	sqlx migrate revert