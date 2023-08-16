#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use crate::routes::main_routes;
use axum::body::{boxed, BoxBody};
use axum::extract::Path;
use axum::response::Response;
use axum::Json;
use chrono::{Duration as ChronoDuration, NaiveDate};
use derive_more::Display;
use dotenvy::dotenv;
use http::{Request, StatusCode, Uri};
use hyper::Body;
use models::asteroid::{NasaData, NearEarthObject};
use reqwest::Client;
use serde_derive::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::PgPool;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tower::ServiceExt;
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use reqwest::Response as ReqResponse;
use std::collections::HashMap;

//we will let our Store struct handle creation of a new pool
use crate::db::new_pool;
use crate::db::Store;
use crate::error::AppError;
use crate::handlers::main_handlers::post_current_nasa;
use crate::models::asteroid::Asteroid;

//Don't forget to make all your files accessible to the crate root HERE
pub mod db;
pub mod error;
pub mod handlers;
pub mod layers;
pub mod models;
pub mod routes;
pub mod template;

use crate::routes::main_routes::app;

pub async fn run_backend() {
    dotenv().ok();
    init_logging();

    //get the socket Addr, based off the .env info
    let addr = get_host_from_env();

    let pool = new_pool().await; //TODO made pool not mut

    //grab the nasa data
    let db = upload_nasa_data(pool).await;

    //this will do all the things, attach to the db, insert cors, set up the router
    let app = routes::main_routes::app(db).await;

    info!("Listening...");

    //bind the server to the socket address
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

///Grabs the data from the .env to set up the databases socket address
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

///Allows logging
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

pub async fn upload_nasa_data(pool: PgPool) -> Store {
    let mut db = Store::with_pool(pool);
    let posted = db.post_current_from_nasa_api().await;
    //return ownership of the pool here
    db
}

///Retrieves all asteroid data from NASA API, formats to our Rust Asteroid struct in preparation for a POST route
/// Makes a request, using reqwest, to NASA's NeoW's APIT using the api key listed in our .env
/// The API only allows for up to 7 days worth of data to be pulled
pub async fn pull_nasa_api_data(date: NaiveDate) -> Result<Vec<NearEarthObject>, AppError> {
    dotenv().ok();
    //get the API key from .env
    let api_key = std::env::var("NASA_API_KEY").unwrap();
    //pull up to a weeks worth of data before the requested date
    let start_date = date - ChronoDuration::days(7);

    let client = Client::new();

    println!("Getting from NASA...");
    let request = format!(
        "https://api.nasa.gov/neo/rest/v1/feed?start_date={}&end_date={}&api_key={}",
        start_date, date, api_key
    );
    let response = client.get(request).send().await?;

    let body = response.text().await?;
    //let all_asteroids = response.text().await?; //turn the JSON into a string

    //This serde magic will take the all_asteroids json and turn it into an ApiResponse struct
    //I really don't understand it, and had to get help from chatpGPT just to find out how to do it
    //But now we can directly get to our hashmap of just the date/Asteroid key/value pairs
    let parsed_asteroids: NasaData = serde_json::from_str(&body)?; //deserialize JSON into an Asteroid struct
    let data = parsed_asteroids.near_earth_objects.clone(); //grab just the HashMap<String, Vec<Asteroid>> from ApiResponse

    //collect all the Vec<Asteroids> in data, into one big Vec
    let mut every_asteroid: Vec<NearEarthObject> = Vec::new();

    //data is a hashmap where each string key is a date, and each associated value is a Vec<Asteroid>
    for (date, asteroid) in data.iter() {
        every_asteroid.extend(asteroid.clone())
    }
    Ok(every_asteroid)
}

///Sends up backwards in time all risk of such actions thrown to the wind and blown straight into a swamp
//Credit: Casey Bailey
pub fn get_timestamp_after_8_hours() -> u64 {
    let now = SystemTime::now();
    let since_epoch = now
        .duration_since(UNIX_EPOCH)
        .expect("Time somehow went backwards");
    // 8 hours later
    let eight_hours_from_now = since_epoch + Duration::from_secs(60 * 60 * 8);
    eight_hours_from_now.as_secs()
}

// https://benw.is/posts/serving-static-files-with-axum
pub async fn get_static_file(uri: Uri) -> Result<Response<BoxBody>, (StatusCode, String)> {
    let req = Request::builder().uri(uri).body(Body::empty()).unwrap();

    // `ServeDir` implements `tower::Service` so we can call it with `tower::ServiceExt::oneshot`
    // When run normally, the root is the workspace root (backend for us if we're running from backend)
    match ServeDir::new("./static").oneshot(req).await {
        Ok(res) => Ok(res.map(boxed)),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", err),
        )),
    }
}

pub async fn file_handler(
    Path(filename): Path<String>,
) -> Result<Response<BoxBody>, (StatusCode, String)> {
    let uri: Uri = format!("/{}", filename).parse().unwrap(); // Construct the URI from the filename
    let res = get_static_file(uri.clone()).await?;

    if res.status() == StatusCode::NOT_FOUND {
        match format!("{}.html", uri).parse() {
            Ok(uri_html) => get_static_file(uri_html).await,
            Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Invalid URI".to_string())),
        }
    } else {
        Ok(res)
    }
}

pub type AppResult<T> = Result<T, AppError>;

/// Basic macro to create a newtype for a database ID.
//Macros cannot manipulate strings when they come from the tokens themselves
//Credit: Casey Bailey
#[macro_export] //we need to do this since this lives in a module and it needs to export to the top level
macro_rules! make_db_id {
    ($name:ident) => {
        //the argument we pass to the macro will replace $name, 'ident' tells rust that whatever we are passing in happens to be an identifier (identifying a particular struct)
        paste::paste! { //paste is a crate that takes the stuff inside [<>] and pastes it together (concatenates strings)
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

            pub trait [<Into $name>] { //here paste concatenates Into and the arg replacing $name
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
