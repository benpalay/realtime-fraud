use axum::{
    routing::get,
    Router,
};
use tokio::net::TcpListener;
use prediction_api::http::{predictions, health};


#[tokio::main]  // rewrites main to be async pre-compilation (doesnt affect runtime performance)
async fn main() {
    println!("Hello, world!");

    let router = Router::new()
        .route("/health", get(health::health_handler))
        .route("/predictions", get(predictions::predictions));

    // run our app with hyper, listening globally on port 3000
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();    
}
