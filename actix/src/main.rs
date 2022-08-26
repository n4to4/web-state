use actix_web::{web, App, HttpResponse, HttpServer};
use tracing_subscriber::EnvFilter;

#[derive(Clone)]
struct State1 {}

#[derive(Clone)]
struct State2 {}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::fmt().with_env_filter(env_filter).init();

    HttpServer::new(move || {
        let state1 = web::Data::new(State1 {});
        App::new()
            .app_data(state1)
            .route("/state200", web::get().to(handler200))
            .route("/state500", web::get().to(handler500))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await?;
    Ok(())
}

async fn handler200(_state: web::Data<State1>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

// DEBUG actix_web::data: Failed to extract `Data<actix::State2>` for `/state500` handler.
// For the Data extractor to work correctly, wrap the data with `Data::new()` and pass it
// to `App::app_data()`. Ensure that types align in both the set and retrieve calls.
async fn handler500(_state: web::Data<State2>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
