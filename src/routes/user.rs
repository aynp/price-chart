use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct User {
    user_id: String,
    user_type: String,
    email: String,
    user_name: String,
    broker: String,
}

impl Default for User {
    fn default() -> Self {
        Self {
            user_id: "AB1234".to_string(),
            user_type: "individual".to_string(),
            email: "xxxyyy@gmail.com".to_string(),
            user_name: "AxAx Bxx".to_string(),
            broker: "ZERODHA".to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct UserProfileResponse {
    status: String,
    data: User,
}

pub async fn profile() -> Json<UserProfileResponse> {
    Json(UserProfileResponse {
        status: "success".to_string(),
        data: User::default(),
    })
}
