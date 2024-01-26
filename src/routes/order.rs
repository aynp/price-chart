use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
struct OrderPlacedData {
    message: String,
    order_id: String,
}

impl Default for OrderPlacedData {
    fn default() -> Self {
        Self {
            message: "Order Placed Successfully".to_string(),
            order_id: "151220000000000".to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct OrderPlacedResponse {
    status: String,
    data: OrderPlacedData,
}

pub async fn place_order() -> Json<OrderPlacedResponse> {
    Json(OrderPlacedResponse {
        status: "success".to_string(),
        data: OrderPlacedData::default(),
    })
}
