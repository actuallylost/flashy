use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use axum_macros::debug_handler;
use serde::Deserialize;
use uuid::Uuid;

use crate::{types::DeckData, AppState};

#[debug_handler]
pub async fn decks(
    State(state): State<AppState>,
) -> Result<Json<Vec<DeckData>>, (StatusCode, String)> {
    let decks_query = state.prisma_client.deck().find_many(vec![]).exec().await;

    decks_query.map(|data| Json(data)).map_err(|err| {
        (
            StatusCode::NOT_FOUND,
            format!("Could not find any decks in the database - {}", err),
        )
    })
}

#[debug_handler]
pub async fn deck(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<DeckData>, (StatusCode, String)> {
    let deck_query = state
        .prisma_client
        .deck()
        .find_unique(prisma::deck::id::equals(id.to_string()))
        .exec()
        .await;

    deck_query.map(|data| Json(data.unwrap())).map_err(|err| {
        (
            StatusCode::NOT_FOUND,
            format!("Could not find any decks in the database - {}", err),
        )
    })
}

#[derive(Deserialize)]
pub struct CreateDeck {
    name: String,
    cards: Vec<Uuid>,
    creator_id: Uuid,
}

// TODO: Implement
// #[debug_handler]
pub async fn create_deck(Json(payload): Json<CreateDeck>, State(state): State<AppState>) {}

#[derive(Deserialize)]
pub struct UpdateDeck {
    name: String,
    cards: Vec<Uuid>,
}

// TODO: Implement
// #[debug_handler]
pub async fn update_deck(
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateDeck>,
    State(state): State<AppState>,
) {
}

// TODO: Implement
#[debug_handler]
pub async fn delete_deck(Path(id): Path<Uuid>, State(state): State<AppState>) {}
