use axum::Json;
use backend::error::AppError;
use backend::models::asteroid::Asteroid;
use backend::run_backend;

#[tokio::main]
async fn main() {
    //all functionality goes in this function from lib.rs
    run_backend().await;
}
