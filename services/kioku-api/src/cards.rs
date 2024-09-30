use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use axum_macros::debug_handler;
use serde::Deserialize;
use uuid::Uuid;

use crate::{types::CardData, AppState};

#[debug_handler]
pub async fn cards(
    State(state): State<AppState>,
) -> Result<Json<Vec<CardData>>, (StatusCode, String)> {
    let cards_query = state.prisma_client.card().find_many(vec![]).exec().await;

    cards_query.map(|cards| Json(cards)).map_err(|err| {
        (
            StatusCode::NOT_FOUND,
            format!("Could not find any cards in the database - {}", err),
        )
    })
}

#[debug_handler]
pub async fn card(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<CardData>, (StatusCode, String)> {
    let card_query = state
        .prisma_client
        .card()
        .find_unique(prisma::card::id::equals(id.to_string()))
        .exec()
        .await;

    card_query.map(|cards| Json(cards.unwrap())).map_err(|err| {
        (
            StatusCode::NOT_FOUND,
            format!("Could not find card with id {} - {}", id.to_string(), err),
        )
    })
}

#[derive(Deserialize, Clone)]
pub struct CreateCard {
    name: String,
    front_desc: String,
    back_desc: String,
    creator_id: String,
}

#[debug_handler]
pub async fn create_card(
    State(state): State<AppState>,
    Json(payload): Json<CreateCard>,
) -> Result<Json<CardData>, (StatusCode, String)> {
    let user_query = state
        .prisma_client
        .user()
        .find_unique(prisma::user::id::equals(payload.creator_id.clone()))
        .exec()
        .await;

    if let Err(err) = user_query {
        return Err((
            StatusCode::NOT_FOUND,
            format!(
                "Could not find user with id {} - {}",
                payload.creator_id, err
            ),
        ));
    }

    let create_card = state
        .prisma_client
        .card()
        .create(
            payload.name,
            payload.front_desc,
            payload.back_desc,
            prisma::user::id::equals(payload.creator_id),
            vec![],
        )
        .exec()
        .await;

    create_card.map(|card| Json(card)).map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Could not create card - {}", err),
        )
    })
}

#[derive(Deserialize)]
pub struct UpdateCard {
    name: String,
    front_desc: String,
    back_desc: String,
    deck_id: String,
}

// TODO: Implement
// #[debug_handler]
pub async fn update_card(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
    Json(payload): Json<UpdateCard>,
) {
}

// TODO: Implement
#[debug_handler]
pub async fn delete_card(Path(id): Path<Uuid>, State(state): State<AppState>) {}
