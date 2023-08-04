
use axum::extract::{Path, Query, State};
use axum::response::Html;
use axum::Json;
use tera::Context;
use tracing::error;
use serde_json::{json, Value};
use tracing::info; //allows us to print to the console using info!()

use crate::db::Store;
use crate::error::AppError;
use crate::models::asteroid::Asteroid;

//bring in the models files here

//we need the templates crate at some point
//use crate::template::TEMPLATES;

#[allow(dead_code)]
pub async fn root() {
    //Does nothing right now
    info!("will add later")
}


pub async fn get_asteroids(
    State(mut am_database): State<Store>,
) -> Result<Json<Vec<Asteroid>>, AppError> {
    let asteroids = am_database.get_all_asteroids().await?;

    Ok(Json(asteroids))
}

//Build functions here as we make new CRUD stuff in db.rs
//all handlers call some function from db.store
