mod auth;
mod cards;
mod decks;
mod types;
mod users;

use std::{sync::Arc, time::Duration};

use axum::{error_handling::HandleErrorLayer, http::StatusCode, routing::get, Router};
use decks::{create_deck, deck, decks, delete_deck, update_deck};
use prisma::PrismaClient;
use tower::{BoxError, ServiceBuilder};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use cards::{card, cards, create_card, delete_card, update_card};
use users::{create_user, delete_user, update_user, user, users};

#[derive(Debug, Clone)]
struct AppState {
    prisma_client: Arc<PrismaClient>,
}

#[tokio::main]
#[doc(hidden)]
async fn main() {
    // setup log tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .try_init()
        .expect("Global default subscriber has already been set");

    // create prisma client
    let client = prisma::new_client()
        .await
        .expect("failed to create prisma client");
    // wrap client in Arc
    let client = Arc::new(client);
    // create app state
    let state = AppState {
        prisma_client: client,
    };

    // users router
    let users_router = Router::new()
        .route("/", get(users).post(create_user))
        .route("/:id", get(user).put(update_user).delete(delete_user));
    // cards router
    let cards_router = Router::new()
        .route("/", get(cards).post(create_card))
        .route("/:id", get(card).put(update_card).delete(delete_card));
    // decks router
    let decks_router = Router::new()
        .route("/decks", get(decks).post(create_deck))
        .route("/:id", get(deck).put(update_deck).delete(delete_deck));
    // auth router
    let auth_router = Router::new();

    // app router
    let app = Router::new()
        // add nested routers
        .nest("/users", users_router)
        .nest("/cards", cards_router)
        .nest("/decks", decks_router)
        .nest("/auth", auth_router)
        // add middleware for request timeout
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|error: BoxError| async move {
                    if error.is::<tower::timeout::error::Elapsed>() {
                        Ok(StatusCode::REQUEST_TIMEOUT)
                    } else {
                        Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Unhandled internal error: {error}"),
                        ))
                    }
                }))
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        )
        .with_state(state);

    // declare listener
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    tracing::info!("Listening on {}", listener.local_addr().unwrap());

    // serve the app
    axum::serve(listener, app).await.unwrap();
}
