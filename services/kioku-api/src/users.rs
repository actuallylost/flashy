use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use axum_macros::debug_handler;
use serde::Deserialize;
use uuid::Uuid;

use crate::{types::UserData, AppState};

pub async fn users(
    State(state): State<AppState>,
) -> Result<Json<Vec<UserData>>, (StatusCode, String)> {
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
) -> Result<Json<UserData>, (StatusCode, String)> {
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

#[derive(Deserialize, Clone)]
pub struct CreateUser {
    username: String,
    email: String,
}

#[debug_handler]
pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<UserData>, (StatusCode, String)> {
    let user_query = state
        .prisma_client
        .user()
        .find_unique(prisma::user::username::equals(payload.username.clone()))
        .exec()
        .await;

    if user_query.is_ok() {
        return Err((StatusCode::CONFLICT, format!("User already exists")));
    }

    let create_user = state
        .prisma_client
        .user()
        .create(payload.username, payload.email, vec![])
        .exec()
        .await;

    create_user.map(|user| Json(user)).map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Could not create user - {}", err),
        )
    })
}

#[derive(Deserialize)]
pub struct UpdateUser {
    username: String,
}

#[debug_handler]
pub async fn update_user(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
    Json(payload): Json<UpdateUser>,
) -> Result<Json<UserData>, (StatusCode, String)> {
    let user_query = state
        .prisma_client
        .user()
        .find_unique(prisma::user::id::equals(id.to_string()))
        .exec()
        .await;

    if let Err(err) = user_query {
        return Err((
            StatusCode::NOT_FOUND,
            format!("Could not find user with id {} - {}", id.to_string(), err),
        ));
    }

    let updated_user = state
        .prisma_client
        .user()
        .update(
            prisma::user::id::equals(id.to_string()),
            vec![prisma::user::username::set(payload.username)],
        )
        .exec()
        .await;

    updated_user.map(|user| Json(user)).map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Could not update user - {}", err),
        )
    })
}

// TODO: Implement
#[debug_handler]
pub async fn delete_user(Path(id): Path<Uuid>, State(state): State<AppState>) {}
