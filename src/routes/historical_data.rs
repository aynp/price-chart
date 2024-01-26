use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};

#[derive(Deserialize)]
pub struct HistoricalDataQueryParams {
    from_date: String,
    to_date: String,
    instrument_name: String,
}

#[derive(Serialize)]
pub struct HistoricalDataResponse {
    date: String,
    price: f64,
    instrument_name: String,
}

pub async fn historical_data(
    query: Query<HistoricalDataQueryParams>,
    State(pool): State<SqlitePool>,
) -> (StatusCode, Json<Vec<HistoricalDataResponse>>) {
    let result =
        sqlx::query("SELECT date, price, instrument_name FROM prices WHERE date BETWEEN $1 AND $2 AND instrument_name IS $3")
            .bind(query.from_date.to_owned())
            .bind(query.to_date.to_owned())
            .bind(query.instrument_name.to_owned())
            .fetch_all(&pool)
            .await
            .unwrap();

    let res = result
        .iter()
        .map(|row| HistoricalDataResponse {
            date: row.get("date"),
            price: row.get("price"),
            instrument_name: row.get("instrument_name"),
        })
        .collect::<Vec<HistoricalDataResponse>>();

    (StatusCode::OK, Json(res))
}
