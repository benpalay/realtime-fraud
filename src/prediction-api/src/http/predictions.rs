use axum::{
    Json,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]

pub struct Prediction {
    id: String,
    fraud: f64,
}
pub async fn predictions() -> Json<Prediction> {
    Json(Prediction {
        id: "123".to_string(),
        fraud: 0.5,
    })
}