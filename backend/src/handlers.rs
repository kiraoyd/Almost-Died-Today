use axum::extract::{Path, Query, State};
use axum::response::Html;
use axum::Json;
use tera::Context;
use tracing::error;

use crate::db::Store;

pub async fn root() {
    //TODO just a test
    info!("hello");
}

//Build functions here as we make new CRUD stuff in db.rs
