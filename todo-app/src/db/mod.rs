use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::time::Duration;

// Type alias - similar to Python's type aliases but more commonly used in Rust
// This is like: DbPool = TypeVar('DbPool', bound=sqlalchemy.engine.Engine)
pub type DbPool = Pool<Postgres>;

// Database initialization function
// Similar to Python's "def get_db()" dependency in FastAPI
pub async fn init_db(database_url: &str) -> Result<DbPool, sqlx::Error> {
    // Create a connection pool - similar to SQLAlchemy's create_engine
    // Rust requires explicit configuration like connection limits
    // In Python, you might set these in create_engine parameters
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(2))
        .connect(database_url)
        .await?;  // ? means "propagate error if this fails" (Python's exception propagation)

    // Run migrations - similar to Alembic's upgrade in Python
    // The ! indicates this is a macro call, not a regular function
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    // Return the pool - successful Result<T, E> (like returning a value in Python)
    Ok(pool)
}

// RUST vs PYTHON:
// - Rust has no exceptions, uses Result<T, E> instead (Ok = success, Err = failure)
// - Rust requires explicit error handling with ? or match
// - Rust's async/await requires explicit return types
// - References (&str) are explicit in Rust, unlike Python where everything is a reference