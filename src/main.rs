mod routings;

use routings::routers::create_router;

#[tokio::main]
async fn main() {
    println!("🚀 Server is starting...");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    let app = create_router();

    println!("✅ Server is ready to accept requests at 0.0.0.0:8080 😎");
    axum::serve(listener, app).await.unwrap();
}
