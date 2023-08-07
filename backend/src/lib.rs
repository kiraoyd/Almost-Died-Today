#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use reqwest::Client;
use chrono::{Duration, NaiveDate};

use derive_more::Display;
use dotenvy::dotenv;
use serde_derive::{Deserialize, Serialize};
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use models::asteroid::{Asteroid, NearEarthObject};
use serde_json::{json, Value};

use std::collections::HashMap;
use reqwest::Response;

//we will let our Store struct handle creation of a new pool
use crate::db::new_pool;
use crate::error::AppError;


//Don't forget to make all your files accessible to the crate root HERE
pub mod db;
pub mod error;
pub mod handlers;
pub mod layers;
pub mod models;
pub mod routes;



use crate::routes::main_routes::app;

pub async fn run_backend() {

    dotenv().ok();
    init_logging();

    //get the socket Addr, based off the .env info
    let addr = get_host_from_env();

    //this will do all the things, attach to the db, insert cors, set up the router
    let app = routes::main_routes::app(new_pool().await).await;

    info!("Listening...");

    //bind the server to the socket address
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
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
fn init_logging() {
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    near_earth_objects: HashMap<String, Vec<Asteroid>>,
}

///Retrieves all asteroid data from NASA API, formats to our Rust Asteroid struct in preparation for a POST route
/// The API only allows for up to 7 days worth of data to be pulled
pub async fn pull_nasa_api_data(date: NaiveDate) -> Result<Vec<NearEarthObject>, AppError> {
    dotenv().ok();
    //get the API key from .env
    let api_key = std::env::var("NASA_API_KEY").unwrap();
    //pull up to a years worth of data before the requested date
    let start_date = date - Duration::days(1);

    println!("{},{}", start_date, date);
    let client = Client::new();

    println!("Getting from NASA...");
    let request = format!("https://api.nasa.gov/neo/rest/v1/feed?start_date={}&end_date={}&api_key={}", start_date, date, api_key);
    let response = client.get(request).send().await?;

    let body = response.text().await?;
    println!("Body, {}", body);
    //let all_asteroids = response.text().await?; //turn the JSON into a string


    //This serde magic will take the all_asteroids json and turn it into an ApiResponse struct
    //I really don't understand it, and had to get help from chatpGPT just to find out how to do it
    //But now we can directly get to our hashmap of just the date/Asteroid key/value pairs
    //TODO serde is having a hard time deserializing, because my Asteroid struct expects Option<i32>s?
    let parsed_asteroids: Asteroid = serde_json::from_str(&body)?; //deserialize JSON into an Asteroid struct
    let data = parsed_asteroids.near_earth_objects.clone(); //grab just the HashMap<String, Vec<Asteroid>> from ApiResponse

    //collect all the Vec<Asteroids> in data, into one big Vec
    let mut every_asteroid: Vec<NearEarthObject> = Vec::new();

    //data is a hashmap where each string key is a date, and each associated value is a Vec<Asteroid>
    for (date,asteroid) in data.iter() {
        every_asteroid.extend(asteroid.clone())
    }
    Ok(every_asteroid)
}

/// Basic macro to create a newtype for a database ID.
#[macro_export]
macro_rules! make_db_id {
    ($name:ident) => {
        paste::paste! {
            #[derive(
                Clone,
                Copy,
                Debug,
                sqlx::Type,
                Display,
                derive_more::Deref,
                PartialEq,
                Eq,
                Hash,
                Serialize,
                Deserialize,
            )]
            pub struct $name(pub i32);

            impl From<i32> for $name {
                fn from(value: i32) -> Self {
                    $name(value)
                }
            }

            impl From<$name> for i32 {
                fn from(value: $name) -> Self {
                    value.0
                }
            }

            pub trait [<Into $name>] {
                fn into_id(self) -> $name;
            }

            impl [<Into $name>] for i32 {
                fn into_id(self) -> $name {
                    $name::from(self)
                }
            }

            impl [<Into $name>] for $name {
                fn into_id(self) -> $name {
                    self
                }
            }
        }
    };
}
