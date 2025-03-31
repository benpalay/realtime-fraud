use axum::{
    routing::get,
    Router,
    Json,
};

use tokio::net::TcpListener;

use serde::{Deserialize, Serialize};

#[tokio::main]  // rewrites main to be async pre-compilation (doesnt affect runtime performance)
async fn main() {
    println!("Hello, world!");

    let router = Router::new()
        .route("/health", get(hello_handler))
        .route("/predictions", get(predictions));

    // run our app with hyper, listening globally on port 3000
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();    
}

async fn hello_handler() -> &'static str {
    "I am Healthy!"
}

#[derive(Serialize, Deserialize)]
struct Prediction {
    id: String,
    fraud: f64,
}
async fn predictions() -> Json<Prediction> {
    Json(Prediction {
        id: "123".to_string(),
        fraud: 0.5,
    })
}

