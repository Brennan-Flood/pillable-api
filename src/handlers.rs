use axum::{Json, extract::{State, Path}};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::db::{create_user, delete_user, get_users};

#[derive(Deserialize)]
pub struct CreateUserRequest {
    name: String,
}

#[derive(Serialize)]
pub struct ApiResponse<T> {
    status: String,
    data: T,
}

pub async fn create_user_handler(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateUserRequest>,
) -> Json<ApiResponse<String>> {
    match create_user(&pool, payload.name).await {
        Ok(user) => Json(ApiResponse { status: "success".to_string(), data: user.id.to_string() }),
        Err(_) => Json(ApiResponse { status: "error".to_string(), data: "Failed to create user".to_string() }),
    }
}

pub async fn get_users_handler(State(pool): State<PgPool>) -> Json<ApiResponse<Vec<(Uuid, String)>>> {
    match get_users(&pool).await {
        Ok(users) => Json(ApiResponse { status: "success".to_string(), data: users.into_iter().map(|u| (u.id, u.name)).collect() }),
        Err(_) => Json(ApiResponse { status: "error".to_string(), data: vec![] }),
    }
}

pub async fn delete_user_handler(
    State(pool): State<PgPool>,
    Path(user_id): Path<Uuid>,
) -> Json<ApiResponse<String>> {
    match delete_user(&pool, user_id).await {
        Ok(_) => Json(ApiResponse { status: "success".to_string(), data: format!("Deleted user {}", user_id) }),
        Err(_) => Json(ApiResponse { status: "error".to_string(), data: "Failed to delete user".to_string() }),
    }
}
