use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use axum_macros::debug_handler;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    types::{CardData, DeckData},
    AppState,
};

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
    cards: Vec<CardData>,
    creator_id: Uuid,
}

#[debug_handler]
pub async fn create_deck(
    State(state): State<AppState>,
    Json(payload): Json<CreateDeck>,
) -> Result<Json<DeckData>, (StatusCode, String)> {
    for card in payload.cards {
        let card_query = state
            .prisma_client
            .card()
            .find_unique(prisma::card::id::equals(card.id))
            .exec()
            .await;

        if let Err(err) = card_query {
            return Err((
                StatusCode::NOT_FOUND,
                format!("Could not find card in the database - {}", err),
            ));
        }
    }
}

#[derive(Deserialize)]
pub struct UpdateDeck {
    name: String,
    cards: Vec<CardData>,
}

// TODO: Implement
#[debug_handler]
pub async fn update_deck(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
    Json(payload): Json<UpdateDeck>,
) {
}

// TODO: Implement
#[debug_handler]
pub async fn delete_deck(Path(id): Path<Uuid>, State(state): State<AppState>) {}
