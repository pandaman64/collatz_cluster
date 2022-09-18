use std::net::SocketAddr;

use axum::{extract::Path, routing::get, Json, Router};
use moka::sync::Cache;
use serde::Serialize;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt::init();

    // The number of capacity depents on the memory size
    let cache = Cache::new(10000);

    let app = Router::new()
        // Seems not working at all :(
        .layer(TraceLayer::new_for_http())
        .route(
            "/steps/:number",
            get(move |number| compute_steps(number, cache.clone())),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    tracing::info!(%addr, "Serving at");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

fn compute(number: u64) -> u64 {
    let mut result = number;
    let mut counter: u64 = 0;

    while result > 1 {
        counter += 1;
        if result % 2 == 0 {
            result /= 2;
        } else {
            result = 3 * result + 1;
        }
    }

    tracing::debug!(number, counter, "Cache miss");
    counter
}

#[derive(Serialize)]
struct ComputeStepsResponse {
    steps: u64,
    cache_hit: bool,
}

async fn compute_steps(
    Path(number): Path<u64>,
    cache: Cache<u64, u64>,
) -> Json<ComputeStepsResponse> {
    let cache_hit = cache.contains_key(&number);
    let steps = cache.get_with(number, move || compute(number));

    Json(ComputeStepsResponse { steps, cache_hit })
}
