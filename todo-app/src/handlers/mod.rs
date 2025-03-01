use actix_web::{web, HttpResponse, Error};
use uuid::Uuid;

use crate::db::DbPool;
use crate::models::{CreateTodo, UpdateTodo, Todo};

// HANDLERS OVERVIEW:
// These handlers are similar to FastAPI route functions with dependency injection
// - web::Data<DbPool> is like Depends(get_db) in FastAPI
// - web::Json<CreateTodo> is like Body(..., CreateTodo) in FastAPI 
// - web::Path<Uuid> is like Path(...) in FastAPI
// - Result<HttpResponse, Error> is like returning a Response object in FastAPI

// GET /todos - Get all todos
// Similar to @app.get("/todos") in FastAPI
pub async fn get_todos(db: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    // sqlx::query_as! is a macro that maps SQL results to structs
    // Similar to SQLAlchemy's session.query(Todo).all() in Python
    let todos = sqlx::query_as!(
        Todo,
        r#"
        SELECT id, title, description, completed, created_at, updated_at
        FROM todos
        "#
    )
    .fetch_all(db.get_ref())
    .await
    .map_err(|e| {
        // Error handling in Rust is more explicit than Python's try/except
        eprintln!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;  // ? operator unwraps the Result or returns early with the error

    // Return HTTP response (similar to returning a dict or Pydantic model in FastAPI)
    Ok(HttpResponse::Ok().json(todos))
}

// POST /todos - Membuat todo baru
pub async fn create_todo(
    todo: web::Json<CreateTodo>,
    db: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    // let id_uuid = id.into_inner();

    // mirip dengan SQLAlchemy insert
    let todo = sqlx::query_as!(
        Todo,
        r#"
        INSERT INTO todos (title, description)
        VALUES ($1, $2)
        RETURNING id, title, description, completed, created_at, updated_at
        "#,
        todo.title,
        todo.description
    )
    .fetch_one(db.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    Ok(HttpResponse::Created().json(todo))
}

// GET /todos/{id} - Mendapatkan todo berdasarkan ID
pub async fn get_todo(
    id: web::Path<Uuid>,
    db: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let id_uuid = id.into_inner();

    let todo = sqlx::query_as!(
        Todo,
        r#"
        SELECT id, title, description, completed, created_at, updated_at
        FROM todos
        WHERE id = $1
        "#,
        id_uuid
    )
    .fetch_optional(db.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    match todo {
        Some(todo) => Ok(HttpResponse::Ok().json(todo)),
        None => Ok(HttpResponse::NotFound().finish()),
    }
}

// PUT /todos/{id} - Update todo
pub async fn update_todo(
    id: web::Path<Uuid>,
    todo: web::Json<UpdateTodo>,
    db: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {

    let id_uuid = id.into_inner();
    // Cek apakah todo ada
    let existing = sqlx::query!("SELECT id FROM todos WHERE id = $1", id_uuid)
        .fetch_optional(db.get_ref())
        .await
        .map_err(|e| {
            eprintln!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    if existing.is_none() {
        return Ok(HttpResponse::NotFound().finish());
    }

    // Update todo
    let todo = sqlx::query_as!(
        Todo,
        r#"
        UPDATE todos
        SET 
            title = COALESCE($1, title),
            description = COALESCE($2, description),
            completed = COALESCE($3, completed),
            updated_at = NOW()
        WHERE id = $4
        RETURNING id, title, description, completed, created_at, updated_at
        "#,
        todo.title,
        todo.description,
        todo.completed,
        id_uuid
    )
    .fetch_one(db.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    Ok(HttpResponse::Ok().json(todo))
}

// DELETE /todos/{id} - Hapus todo
pub async fn delete_todo(
    id: web::Path<Uuid>,
    db: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let id_uuid = id.into_inner();
    let result = sqlx::query!("DELETE FROM todos WHERE id = $1", id_uuid)
        .execute(db.get_ref())
        .await
        .map_err(|e| {
            eprintln!("Database error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    if result.rows_affected() == 0 {
        return Ok(HttpResponse::NotFound().finish());
    }

    Ok(HttpResponse::NoContent().finish())
}

// RUST TIPS for Pythonistas:
// 1. ? operator: Unwraps Result/Option or returns early with error (like try/except shorthand)
// 2. match: Similar to Python's match/case or more powerful if/elif/else
// 3. into_inner(): Extracts the value from a wrapper (similar to Python's value extraction)
// 4. map_err(): Transform errors (like catching and re-raising exceptions in Python)
// 5. async/await: Works similarly to Python but with more type safety
