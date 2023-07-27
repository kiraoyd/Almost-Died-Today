use backend::run_backend;

#[tokio::main]
async fn main() {
    //all functionality goes in this function from lib.rs
    run_backend().await;
}