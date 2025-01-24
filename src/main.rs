mod commons;
mod configs;
mod handlers;
mod models;
mod routes;
mod services;
mod utils;

use dotenv::dotenv;
use sqlx::mysql::MySqlPoolOptions;

use crate::routes::routers::create_router;

#[tokio::main]
async fn main() {
    println!("🚀 Server is starting...");
    dotenv().ok();

    println!("🔧 Connecting to database server...");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅ Database connection established successfully 😎");
            pool
        }
        Err(err) => {
            println!("❌ Database connection failed: {}", err);
            std::process::exit(1);
        }
    };

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    let app = create_router(pool);

    println!("✅ Server is ready to accept requests at 0.0.0.0:8080 😎");
    axum::serve(listener, app).await.unwrap();
}
