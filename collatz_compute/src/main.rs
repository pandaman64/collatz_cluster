use std::net::SocketAddr;

use axum::{extract::Path, routing::get, Router};
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        // Seems not working at all :(
        .layer(TraceLayer::new_for_http())
        .route("/steps/:number", get(compute_steps));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    tracing::info!(%addr, "Serving at");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn compute_steps(Path(number): Path<u64>) -> String {
    let mut result = number;
    // TODO: implement cache
    let mut counter: u64 = 0;

    while result > 1 {
        counter += 1;
        if result % 2 == 0 {
            result /= 2;
        } else {
            result = 3 * result + 1;
        }
    }

    tracing::debug!(number, counter, "Collatz");
    counter.to_string()
}
