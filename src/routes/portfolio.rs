use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Holding {
    tradingsymbol: String,
    exchange: String,
    isin: String,
    quantity: i32,
    authorised_date: String,
    average_price: f32,
    last_price: f32,
    close_price: f32,
    pnl: f32,
    day_change: f32,
    day_change_percentage: f32,
}

#[derive(Serialize)]
pub struct HoldingsResponse {
    status: String,
    data: Vec<Holding>,
}

pub async fn holdings() -> Json<HoldingsResponse> {
    let holdings = vec![
        Holding {
            tradingsymbol: "GOLDBEES".to_string(),
            exchange: "BSE".to_string(),
            isin: "INF204KB17I5".to_string(),
            quantity: 2,
            authorised_date: "2021-06-08 00:00:00".to_string(),
            average_price: 40.67,
            last_price: 42.47,
            close_price: 42.28,
            pnl: 3.5999999999999943,
            day_change: 0.18999999999999773,
            day_change_percentage: 0.44938505203405327,
        },
        Holding {
            tradingsymbol: "IDEA".to_string(),
            exchange: "NSE".to_string(),
            isin: "INE669E01016".to_string(),
            quantity: 5,
            authorised_date: "2021-06-08 00:00:00".to_string(),
            average_price: 8.466,
            last_price: 10.0,
            close_price: 10.1,
            pnl: 7.6700000000000035,
            day_change: -0.09999999999999964,
            day_change_percentage: -0.9900990099009866,
        },
    ];

    Json(HoldingsResponse {
        status: "success".to_string(),
        data: holdings,
    })
}
