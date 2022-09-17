use std::net::SocketAddr;

use axum::{extract::Path, routing::get, Router};
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .layer(TraceLayer::new_for_http())
        .route("/steps/:number", get(compute_steps));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn compute_steps(Path(mut number): Path<u64>) -> String {
    // TODO: implement cache
    let mut counter: u64 = 0;

    while number > 1 {
        counter += 1;
        if number % 2 == 0 {
            number /= 2;
        } else {
            number = 3 * number + 1;
        }
    }

    counter.to_string()
}
