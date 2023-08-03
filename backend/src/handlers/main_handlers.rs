use axum::extract::{Path, Query, State};
use axum::response::Html;
use axum::Json;
use tera::Context;
use tracing::error;
use tracing::info; //allows us to print to the console using info!()

use crate::db::Store;
use crate::error::AppError;
use crate::models::asteroid::Asteroid;

//bring in the models files here
//use crate::models::answer::{models that live in answer}

//we need the templates crate at some point
//use crate::template::TEMPLATES;

pub async fn root() {
    //Does nothing right now
    info!("will add later")
}

pub async fn test_db( State(mut am_database): State<Store>,) -> Result<(), sqlx::Error>  {
    let asteroid = am_database.test_db().await?;

    Ok(())
}

//Build functions here as we make new CRUD stuff in db.rs
//all handlers call some function from db.store