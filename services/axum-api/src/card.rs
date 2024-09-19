use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_macros::debug_handler;
use serde::Deserialize;

use crate::AppState;

pub async fn cards(State(state): State<AppState>) -> impl IntoResponse {
    let cards_query = state.prisma_client.card().find_many(vec![]).exec().await;

    cards_query.map(|cards| Json(cards)).map_err(|_| {
        (
            StatusCode::NOT_FOUND,
            format!("Could not find any cards in the database."),
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

#[debug_handler(state = AppState)]
pub async fn create_card(
    State(state): State<AppState>,
    Json(payload): Json<CreateCard>,
) -> Result<Json<prisma::card::Data>, (StatusCode, String)> {
    let user_query = state
        .prisma_client
        .user()
        .find_unique(prisma::user::id::equals(payload.creator_id.clone()))
        .exec()
        .await
        .unwrap();

    if user_query.is_none() {
        return Err((
            StatusCode::NOT_FOUND,
            format!("Could not find user with id {}", payload.creator_id),
        ));
    }

    let card_query = state
        .prisma_client
        .card()
        .create(
            payload.name,
            payload.front_desc,
            payload.back_desc,
            prisma::user::UniqueWhereParam::IdEquals(payload.creator_id),
            vec![],
        )
        .exec()
        .await;

    match card_query {
        Ok(card) => Ok(Json(card)),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Could not create card: {}", err),
        )),
    }
}
