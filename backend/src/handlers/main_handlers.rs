use axum::extract::{Path, Query, State};
use axum::response::Html;
use axum::Json;
use tera::Context;
use tracing::error;
use tracing::info; //allows us to print to the console using info!()

use crate::db::Store;
use crate::error::AppError;

//bring in the models files here
//use crate::models::answer::{models that live in answer}

//we need the templates crate at some point
//use crate::template::TEMPLATES;

pub async fn root() {
    //TODO just a test
    info!("hello");
}

//Build functions here as we make new CRUD stuff in db.rs
//all handlers call some function from db.store