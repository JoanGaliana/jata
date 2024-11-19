mod todo;

use crate::todo::controller::TODO_BASE_PATH;
use axum::response::Redirect;
use axum::routing::get;
use todo::controller::get_router;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // build our application with some routes
    let app = get_router().route("/", get(Redirect::to(TODO_BASE_PATH)));

    // run it
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    tracing::debug!(
        "listening on {} http://localhost:3001",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app).await.unwrap();
}
