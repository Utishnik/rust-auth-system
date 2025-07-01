use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use sqlx::PgPool;

use super::models::{LoginUser, RegisterUser};
use super::service::{get_user_by_email, register_user, verify_email_token};
use super::utils::{create_jwt, verify_password};

#[derive(Deserialize)]
pub struct VerifyEmailQuery {
    token: String,
}

pub fn create_router() -> Router<PgPool> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/verify-email", get(verify_email))
}

async fn register(
    State(pool): State<PgPool>,
    Json(payload): Json<RegisterUser>,
) -> Response {
    match register_user(&pool, payload).await {
        Ok(user) => (StatusCode::CREATED, Json(user)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

async fn login(State(pool): State<PgPool>, Json(payload): Json<LoginUser>) -> Response {
    let user_result = get_user_by_email(&pool, &payload.email).await;

    let user = match user_result {
        Ok(Some(user)) => user,
        Ok(None) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({"error": "Invalid email or password"})),
            )
                .into_response()
        }
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    if !user.is_email_verified {
        return (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({"error": "Email not verified"})),
        )
            .into_response();
    }

    if verify_password(&payload.password, &user.password_hash).unwrap_or(false) {
        match create_jwt(&user.id.to_string()) {
            Ok(token) => (
                StatusCode::OK,
                Json(serde_json::json!({ "token": token })),
            )
                .into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    } else {
        (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({"error": "Invalid email or password"})),
        )
            .into_response()
    }
}

async fn verify_email(
    State(pool): State<PgPool>,
    Query(params): Query<VerifyEmailQuery>,
) -> Response {
    match verify_email_token(&pool, &params.token).await {
        Ok(true) => (
            StatusCode::OK,
            Json(serde_json::json!({"message": "Email verified successfully"})),
        )
            .into_response(),
        Ok(false) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "Invalid or expired verification token"})),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}