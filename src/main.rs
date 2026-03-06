use axum::Router;
use dotenv::dotenv;
use sqlx::sqlite::SqlitePoolOptions;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

mod auth;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url: String = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool: sqlx::Pool<sqlx::Sqlite> = SqlitePoolOptions::new()
    .max_connections(5)
    .connect(&database_url)
    .await
    .expect("Failed to create pool.");

    let cors: CorsLayer = CorsLayer::new().allow_origin(Any).allow_methods(Any);

    let app: Router = Router::new()
        .nest("/auth", auth::handlers::create_router())
        .with_state(pool)
        .layer(cors);

    let addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("listening on {}", addr);

    let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
