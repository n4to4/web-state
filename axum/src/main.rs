use axum::{extract::State, routing::get, Router};
use tracing_subscriber::EnvFilter;

#[derive(Debug, Clone)]
struct State1 {}

#[derive(Debug, Clone)]
struct State2 {}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::fmt().with_env_filter(env_filter).init();

    let state = State1 {};
    let app = Router::with_state(state)
        // |         .route("/state500", get(handler2))
        // |          -----              ^^^^^^^^^^^^^ expected struct `State1`, found struct `State2`
        // |          |
        // |          arguments to this function are incorrect
        // |
        //.route("/state500", get(handler2))
        .route("/state200", get(handler));

    axum::Server::bind(&"127.0.0.1:8001".parse()?)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

async fn handler(State(state): State<State1>) {
    tracing::info!("handler: {:#?}", state);
}

#[allow(dead_code)]
async fn handler2(State(state): State<State2>) {
    tracing::info!("handler: {:#?}", state);
}
