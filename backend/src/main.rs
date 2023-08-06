use backend::run_backend;
use backend::pull_NASA_API_data;
use axum::Json;
use backend::error::AppError;
use backend::models::asteroid::Asteroid;

#[tokio::main]
async fn main() {
    //all functionality goes in this function from lib.rs
    //run_backend().await;
    //TODO test:
    let today = chrono::offset::Utc::now();
    let naive_today = today.date().naive_utc();  //chatGPT
    let response = pull_NASA_API_data(naive_today).await;

}
