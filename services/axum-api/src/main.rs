mod card;

use std::{sync::Arc, time::Duration};

use axum::{error_handling::HandleErrorLayer, http::StatusCode, routing::get, Router};
use prisma::PrismaClient;
use tower::{BoxError, ServiceBuilder};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use card::{cards, create_card};

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
    // create routers
    let cards_router = Router::new().route("/", get(cards).post(create_card));
    // declare routes and respective handlers
    let app = Router::new()
        .nest("/cards", cards_router)
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
