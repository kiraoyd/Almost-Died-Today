use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;

use dotenvy::dotenv;
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

//we will let our Store struct handle creation of a new pool
use crate::db::new_pool;

pub mod routes;

pub async fn run_backend() {
    dotenv().ok();
    init_logging();

    //get the socket Addr, based off the .env info
    let addr = get_host_from_env();

    //this will do all the things, attach to the db, insert cors, set up the router
    let app = routes::app(new_pool().await).await;

    info!("Listening...");

    //bind the server to the socket address
    axum::Server::bind(&addr)
        .server(app.into_make_service())
        .await
        .unwrap();
}


fn get_host_from_env() -> SocketAddr {
    let host = std::env::var("API_HOST").unwrap();
    let api_host = IpAddr::from_str(&host).unwrap();
    let api_port: u16 = std::env::var("API_PORT")
        .expect("Could not find API_PORT in .env file")
        .parse()
        .expect("Can't create a u16 from he given API_PORT string");

    //return the socketAddr
    SocketAddr::from((api_host, api_port))
}
fn init_logging(){
    // https://github.com/tokio-rs/axum/blob/main/examples/tracing-aka-logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                "backend=trace,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}