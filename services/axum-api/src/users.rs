use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use axum_macros::debug_handler;
use serde::Deserialize;
use uuid::Uuid;

use crate::AppState;

pub async fn users(
    State(state): State<AppState>,
) -> Result<Json<Vec<prisma::user::Data>>, (StatusCode, String)> {
    let users_query = state.prisma_client.user().find_many(vec![]).exec().await;

    users_query.map(|users| Json(users)).map_err(|err| {
        (
            StatusCode::NOT_FOUND,
            format!("Could not find any users in the database - {}", err),
        )
    })
}

pub async fn user(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<prisma::user::Data>, (StatusCode, String)> {
    let user_query = state
        .prisma_client
        .user()
        .find_unique(prisma::user::id::equals(id.to_string()))
        .exec()
        .await;

    user_query.map(|user| Json(user.unwrap())).map_err(|err| {
        (
            StatusCode::NOT_FOUND,
            format!("Could not find user - {}", err),
        )
    })
}

#[derive(Deserialize)]
pub struct CreateUser {
    username: String,
    email: String,
}

// TODO: Implement
// #[debug_handler]
pub async fn create_user(Json(payload): Json<CreateUser>, State(state): State<AppState>) {}

#[derive(Deserialize)]
pub struct UpdateUser {
    username: String,
}

// TODO: Implement
// #[debug_handler]
pub async fn update_user(
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUser>,
    State(state): State<AppState>,
) {
}

// TODO: Implement
#[debug_handler]
pub async fn delete_user(Path(id): Path<Uuid>, State(state): State<AppState>) {}
