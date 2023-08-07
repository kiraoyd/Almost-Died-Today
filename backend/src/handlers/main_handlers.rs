use axum::extract::{Path, Query, State};
use axum::response::Html;
use axum::Json;
use serde_json::{json, Value};
use tera::Context;
use tracing::error;
use tracing::info; //allows us to print to the console using info!()

use crate::db::Store;
use crate::error::AppError;
use crate::models::asteroid::{Asteroid, NearEarthObject};

//bring in the models files here

//we need the templates crate at some point
//use crate::template::TEMPLATES;

#[allow(dead_code)]
pub async fn root() {
    //use Tera to load everything from our templates.rs file, into a Hasmap of templates
    //Then we tell this route which one we want to render, and provide it the context

    //The context is where we can add dynamic data to our templates
    let mut context = Context::new();
    context.insert("name", "Casey"); //here is where we build little bits of context

    //Along with that context and template, Tera will render everything
    let rendered = TEMPLATES
        .render("index.html", &context) //if someone is logged in, send them here (or whatever page you want to be restricted by login)
        .unwrap_or_else(|err| {
            error!("Template rendering error: {}", err);
            panic!()
        });
    Html(rendered) //Then we send the html back
}

pub async fn get_asteroids(
    State(mut am_database): State<Store>,
) -> Result<Json<Vec<Asteroid>>, AppError> {
    let asteroids = am_database.get_all_asteroids().await?;

    Ok(Json(asteroids))
}



pub async fn post_current_nasa(
    State(mut am_database): State<Store>,
) -> Result<Json<Vec<Asteroid>>, AppError> {
    let posted = am_database.post_current_from_nasa_api().await?;

    Ok(Json(posted))
}


pub async fn get_closest(
    State(mut am_database): State<Store>,
    Path(query): Path<String>,
) -> Result<Json<Asteroid>, AppError> {
    let date = query.to_owned();
    let closest = am_database.get_closest_by_date(date).await?;
    Ok(Json(closest))
}



//Build functions here as we make new CRUD stuff in db.rs
//all handlers call some function from db.store
