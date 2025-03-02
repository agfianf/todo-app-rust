// Modules import in Rust (like Python's import statements)
// actix_web is a web framework similar to FastAPI in Python
use actix_web::{web, App, HttpServer, middleware::Logger};
use actix_cors::Cors; // Tambah ini
use dotenv::dotenv;
use std::env;

// Module declarations - similar to Python's from x import y
// but they refer to local files/modules instead of packages
mod db;
mod handlers;
mod models;

// The #[actix_web::main] is a macro (starts with #) - similar to Python decorators
// It sets up the async runtime (like asyncio in Python)
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables (similar to Python's dotenv.load_dotenv())
    dotenv().ok();
    
    // Setup logging (similar to Python's logging configuration)
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    // Get environment variables (like os.getenv in Python)
    // The expect method is similar to Python's raise Exception if None
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    // Initialize the database - creates a connection pool
    // Similar to SQLAlchemy's create_engine() in Python
    let db_pool = db::init_db(&database_url)
        .await  // .await is similar to Python's await keyword
        .expect("Failed to create pool");

    log::info!("Starting server at http://localhost:8080");
    
    // Create and start HTTP server
    // Similar to FastAPI's uvicorn.run() in Python
    HttpServer::new(move || {
        // Tambah CORS middleware, mirip FastAPI CORSMiddleware
        let cors = Cors::default()
            .allowed_origin("http://localhost:8081") // Izinin origin frontend
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![actix_web::http::header::CONTENT_TYPE])
            .max_age(3600);

        // App::new() is similar to FastAPI() in Python
        App::new()
            .wrap(cors)
            // Middleware in Rust (similar to FastAPI's app.add_middleware())
            .wrap(Logger::default())
            // Dependency injection (similar to FastAPI's Depends())
            .app_data(web::Data::new(db_pool.clone()))
            // Route configuration (similar to FastAPI's app.include_router())
            .service(
                // Routes grouped under /api (similar to APIRouter with prefix in FastAPI)
                web::scope("/api")
                    .route("/todos", web::get().to(handlers::get_todos))
                    .route("/todos", web::post().to(handlers::create_todo))
                    .route("/todos/{id}", web::get().to(handlers::get_todo))
                    .route("/todos/{id}", web::put().to(handlers::update_todo))
                    .route("/todos/{id}", web::delete().to(handlers::delete_todo))
            )
    })
    .bind("0.0.0.0:8080")?  // ? is error propagation (similar to Python's try/except)
    .run()
    .await
}