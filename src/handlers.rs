use axum::{Json, extract::{State, Path}};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use crate::db::*;

// ===== API RESPONSE WRAPPER =====
#[derive(Serialize)]
pub struct ApiResponse<T> {
    status: String,
    data: T,
}

// ===== USERS =====
#[derive(Deserialize)]
pub struct CreateUserRequest {
    name: String,
}

pub async fn create_user_handler(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateUserRequest>,
) -> Json<ApiResponse<User>> {
    match create_user(&pool, payload.name).await {
        Ok(user) => Json(ApiResponse { status: "success".to_string(), data: user }),
        Err(_) => Json(ApiResponse { status: "error".to_string(), data: User { id: Uuid::nil(), name: "Failed".to_string() } }),
    }
}

pub async fn get_users_handler(State(pool): State<PgPool>) -> Json<ApiResponse<Vec<User>>> {
    match get_users(&pool).await {
        Ok(users) => Json(ApiResponse { status: "success".to_string(), data: users }),
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


// ===== SUPPLEMENTS =====
#[derive(Deserialize)]
pub struct CreateSupplementRequest {
    name: String,
    store_link: Option<String>,
    image_url: Option<String>,
}

pub async fn create_supplement_handler(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateSupplementRequest>,
) -> Json<ApiResponse<Supplement>> {
    match create_supplement(&pool, payload.name, payload.store_link, payload.image_url).await {
        Ok(supplement) => Json(ApiResponse { status: "success".to_string(), data: supplement }),
        Err(_) => Json(ApiResponse { status: "error".to_string(), data: Supplement { id: 0, name: "Failed".to_string(), store_link: None, image_url: None } }),
    }
}

pub async fn get_supplements_handler(State(pool): State<PgPool>) -> Json<ApiResponse<Vec<Supplement>>> {
    match get_supplements(&pool).await {
        Ok(supplements) => Json(ApiResponse { status: "success".to_string(), data: supplements }),
        Err(_) => Json(ApiResponse { status: "error".to_string(), data: vec![] }),
    }
}

// ===== FOLLOWS =====
#[derive(Deserialize)]
pub struct CreateFollowRequest {
    following_user: Uuid,
    followed_user: Uuid,
}

pub async fn create_follow_handler(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateFollowRequest>,
) -> Json<ApiResponse<Follow>> {
    match create_follow(&pool, payload.following_user, payload.followed_user).await {
        Ok(follow) => Json(ApiResponse { status: "success".to_string(), data: follow }),
        Err(_) => Json(ApiResponse { status: "error".to_string(), data: Follow { following_user: Uuid::nil(), followed_user: Uuid::nil() } }),
    }
}

pub async fn get_follows_handler(State(pool): State<PgPool>) -> Json<ApiResponse<Vec<Follow>>> {
    match get_follows(&pool).await {
        Ok(follows) => Json(ApiResponse { status: "success".to_string(), data: follows }),
        Err(_) => Json(ApiResponse { status: "error".to_string(), data: vec![] }),
    }
}

// ===== REVIEWS =====
#[derive(Deserialize)]
pub struct CreateReviewRequest {
    title: String,
    body: String,
    user_id: Uuid,
    supplement_id: i32,
    rating: i32,
}

pub async fn create_review_handler(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateReviewRequest>,
) -> Json<ApiResponse<Review>> {
    match create_review(&pool, payload.title, payload.body, payload.user_id, payload.supplement_id, payload.rating).await {
        Ok(review) => Json(ApiResponse { status: "success".to_string(), data: review }),
        Err(_) => Json(ApiResponse { status: "error".to_string(), data: Review { id: 0, title: "Failed".to_string(), body: "".to_string(), user_id: Uuid::nil(), supplement_id: 0, rating: 0 } }),
    }
}
