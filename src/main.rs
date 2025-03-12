use axum::{Router, routing::{get, post, delete}, extract::State};
use sqlx::PgPool;
use std::env;
use dotenv::dotenv;
use tokio::net::TcpListener;

mod db;
mod handlers;

use handlers::*;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await.expect("Failed to connect to DB");

    let app = Router::new()
        .route("/users", post(create_user_handler))
        .route("/users", get(get_users_handler))
        .route("/users/:id", delete(delete_user_handler))
        .route("/follows", post(create_follow_handler))
        .route("/follows", get(get_follows_handler))
        .route("/supplements", post(create_supplement_handler))
        .route("/supplements", get(get_supplements_handler))
        .route("/reviews", post(create_review_handler))
        .with_state(pool);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
