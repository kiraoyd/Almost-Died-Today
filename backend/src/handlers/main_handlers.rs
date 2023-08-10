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

use crate::template::TEMPLATES;
//bring in the models files here

//we need the templates crate at some point
use crate::template::TEMPLATES;
//use crate::models::user::{Claims, OptionalClaims, User, UserSignup, KEYS};

#[allow(dead_code)]
pub async fn root(
    State(am_database): State<Store>,
    OptionalClaims(claims): OptionalClaims,
) -> Result<Html<String>, AppError> {

    //use Tera to load everything from our templates.rs file, into a Hasmap of templates
    //Then we tell this route which one we want to render, and provide it the context
    //Any context we establish here, we will be able to pull in to any of our html pages

    //The context is where we can add in dynamic data values to our html
    //TODO update this to the class projects format
    let mut context = Context::new();

    //set up what we want to render with, all contexts go here now
    let template_name = if let Some(claims_data) = claims {
        context.insert("claims", &claims.data);
        context.insert("is_logged_in", &true);

        //TODO make the get_all_asteroid_pages function in db.rs
        let page_packages = am_database.get_all_asteroid_pages().await?;
        context.insert("page_packages", &page_packages);
        "pages.html"
    } else {
        context.insert("is_logged_in", &false);
        "index.html"
    };
    //Along with that context and template, Tera will render everything
    let rendered = TEMPLATES
        //Convert this to class project, where we pass a variable called "template_name" TODO
        .render(template_name, &context) //render takes all the context in template_name
        .unwrap_or_else(|err| {
            error!("Template rendering error: {}", err);
            panic!()
        });
    Ok(Html(rendered)) //Then we send the html back
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
