use axum::{
    Json,
    extract::Query,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]

pub struct Prediction {
    id: String,
    fraud: f64,
}
#[derive(Deserialize)]
pub struct PredictionParams {
    id: String,
}

pub async fn predictions(Query(params): Query<PredictionParams>) -> Json<Prediction> {
    Json(Prediction { //would fetch from db/model in real life
        id: params.id.to_string(),
        fraud: 0.5,
    })
}