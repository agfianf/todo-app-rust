use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// In Rust, structs are similar to Python dataclasses or Pydantic models
// This struct represents a Todo item from the database
// The derive macros are similar to Python decorators:
// - Serialize/Deserialize = Pydantic's json() and parse_obj() methods
// - Debug = Helpful for printing the struct for debugging (like Python's __repr__)
#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    // Uuid is similar to Python's UUID type
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>, // Option<T> is similar to Python's Optional[T]
    pub completed: bool,
    // DateTime<Utc> is similar to Python's datetime with timezone
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Input validation struct for creating a todo
// Similar to a Pydantic model used for request body validation in FastAPI
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTodo {
    pub title: String,
    // Option<T> means the field is optional - similar to Optional[str] in Python
    pub description: Option<String>,
}

// Input validation struct for updating a todo
// In FastAPI, you might use a Pydantic model with all optional fields
// In Rust, we use Option<T> to represent each optional field
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTodo {
    pub title: Option<String>,
    pub description: Option<String>,
    pub completed: Option<bool>,
}

// Rust TIPS:
// - Unlike Python, Rust has no implicit None/null. You must explicitly use Option<T>
// - Structs are immutable by default; use 'mut' keyword to make them mutable
// - Struct fields need explicit visibility (pub) to be accessed outside the module
