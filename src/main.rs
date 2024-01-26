mod routes;

use axum::{routing::{get, post}, Router};
use sqlx::sqlite::SqlitePoolOptions;

#[tokio::main]
async fn main() {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite:./db.sqlite")
        .await
        .unwrap();

    let app = Router::new()
        .route("/", get(root))
        .route(
            "/historical-data",
            get(routes::historical_data::historical_data),
        )
        .route("/portfolio/holdings", get(routes::portfolio::holdings))
        .route("/user/profile", get(routes::user::profile))
        .route("/order/place", post(routes::order::place_order))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}
