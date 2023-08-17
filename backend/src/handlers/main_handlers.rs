use argon2::Config; //for hashing the password
use axum::extract::{Path, Query, State};
use axum::response::{Html, Response};
use axum::{Form, Json};
//HWT token stuff
use http::header::{LOCATION, SET_COOKIE};
use http::{HeaderValue, StatusCode};
use hyper::Body;
use jsonwebtoken::Header;
use serde_json::{json, Value};
use tera::Context;
use tracing::error;
use tracing::info; //allows us to print to the console using info!()

use crate::db::Store;
use crate::error::AppError;
use crate::get_timestamp_after_8_hours;

//bring in the models files here
use crate::models::asteroid::{Asteroid, NearEarthObject, SearchResult, UserSearch};
use crate::models::page::LoginErrors;
use crate::models::user::{Claims, OptionalClaims, User, UserSignup, KEYS};

//we need the templates crate at some point
use crate::template::TEMPLATES;

#[allow(dead_code)]
///loads context to the main landing page, differing depending on if a user is logged on (has a JWT token stored as a browser cooke) or not
pub async fn root(
    State(mut am_database): State<Store>, //has to be mutable to let db.rs call one of it's own function inside another of its functions
    OptionalClaims(claims): OptionalClaims,
) -> Result<Html<String>, AppError> {
    //use Tera to load everything from our templates.rs file, into a Hashmap of templates
    //Then we tell this route which one we want to render, and provide it the context
    //Any context we establish here, we will be able to pull into the related html page

    //The context is where we can add in dynamic data values to our html
    let mut context = Context::new();

    //set up what we want to render with, all contexts go here now and will be available to the specified .html files
    let template_name = if let Some(claims_data) = claims {
        //user is logged in and we have the claims to prove it
        context.insert("claims", &claims_data);
        context.insert("is_logged_in", &true);

        let page_package = am_database.get_main_page().await?;
        println!("Got the page package: {}", page_package);
        context.insert("page_package", &page_package);
        "pages.html" //route to logged in template when logged in
    } else {
        //user is NOT logged in
        error!("is_logged_in is FALSE now");
        context.insert("is_logged_in", &false);
        "index.html" //route to original template when not logged in
    };

    //Along with that context and template, Tera will render everything
    //Note: this render will err if the context placeholders in the html are done incorrectly
    let rendered = TEMPLATES
        .render(template_name, &context) //render takes all the context attached in template_name and inserts it
        .unwrap_or_else(|err| {
            error!("Template rendering error: {}", err);
            panic!()
        });
    Ok(Html(rendered)) //Then we send the html back
}

///Retreives asteroid that passed closest to earth on the date (a String) provided by the html Form, returns the found Asteroid (which will either be None or Asteroid)
pub async fn search_from_form(
    State(mut am_database): State<Store>,
    Form(search_date): Form<UserSearch>, //search_date comes in from the frontend, is the info typed into the search form input field
) -> Result<Html<String>, AppError> {
    //make a SearchResult in preparation to be sent back
    let mut result = SearchResult {
        asteroid: None,
        message: "No asteroid found to match that date requested.".to_string(),
        has_data: false,
    };

    let date = search_date.search_date.clone();

    let found_asteroid = am_database.get_closest_by_date(date).await?;
    if let Some(asteroid) = found_asteroid {
        //then we have data
        result.asteroid = Some(asteroid);
        result.message = format!(
            "Looky here, we all got lucky on {} too!",
            search_date.search_date
        )
        .to_string();
        result.has_data = true;
    }

    //The context is where we can add in dynamic data values to our html
    let mut context = Context::new();

    //insert the search result as the context
    let template_name = {
        context.insert("found_asteroid", &result);
        "search_result.html" //route to logged in template when logged in
    };
    //Render search_result.html with all the context
    let rendered = TEMPLATES
        .render(template_name, &context) //render takes all the context attached in template_name and inserts it
        .unwrap_or_else(|err| {
            error!("Template rendering error: {}", err);
            panic!()
        });
    Ok(Html(rendered)) //Then we send the html back
}

//Build functions here as we make new CRUD stuff in db.rs
//all handlers call some function from db.store

///Retrieves all asteroids from the database
pub async fn get_asteroids(
    State(mut am_database): State<Store>,
) -> Result<Json<Vec<Asteroid>>, AppError> {
    let asteroids = am_database.get_all_asteroids().await?;

    Ok(Json(asteroids))
}

///Queries NASA's NeoW API for 7 days (the limit) worth of data, then posts that data to our database
pub async fn post_current_nasa(
    State(mut am_database): State<Store>,
) -> Result<Json<Vec<Asteroid>>, AppError> {
    let posted = am_database.post_current_from_nasa_api().await?;

    Ok(Json(posted))
}

///Retrieves the asteroid that passed closest to earth on the date given in the query params
pub async fn get_closest(
    State(mut am_database): State<Store>,
    Path(query): Path<String>,
) -> Result<Json<Asteroid>, AppError> {
    let date = query.to_owned();
    let closest = am_database.get_closest_by_date(date).await?;
    Ok(Json(closest.unwrap()))
}

//Handlers below handle functionality related to login/users
//In a real production site, we would use 3rd party Authorizaton instead of implementing our own
//Credit: Casey Bailey 2023
///Accepts user Credentials in a UserSignup struct, checks all credentials exist and are valid, checks that requested user email for signup is not already
/// present in the database. If everything this valid, hashes the password using argon2, resets the password feild in credentials, add thes new user to the
/// database and returns the users information for confirmation
pub async fn register(
    State(database): State<Store>, //TODO removed mut from database
    Form(mut credentials): Form<UserSignup>, //credentials come in from the frontend, after a user attempts to login
) -> Result<Json<Value>, AppError> {
    //missing feilds
    if credentials.email.is_empty() || credentials.password.is_empty() {
        return Err(AppError::MissingCredentials);
    }

    //password and password confirmation do not match
    if credentials.password != credentials.confirm_password {
        return Err(AppError::MissingCredentials);
    }

    //user already is in database with this email address
    let existing_user = database.get_user(&credentials.email).await;
    //TODO removed: if let Ok(_) = existing_user
    if existing_user.is_ok() {
        return Err(AppError::UserAlreadyExists);
    }

    //if user and credentials are valid, hash their password
    let hash_config = Config::default();
    let salt = std::env::var("SALT").expect("Missing SALT");
    //use argon2 to hash with the given SALT
    let hashed_password = match argon2::hash_encoded(
        credentials.password.as_bytes(),
        salt.as_bytes(),
        &hash_config,
    ) {
        Ok(result) => result,
        Err(_) => {
            return Err(AppError::Any(anyhow::anyhow!("Password hashing failed")));
        }
    };

    credentials.password = hashed_password; //reset the password feild in credentials to be the hashed one
    let new_user = database.create_user(credentials).await?;
    Ok(new_user)
}

///Verifies the credentials given by a user trying to login, if valid,make a JWT token and store it as a browser cookie
///Has one heck of an ugly if/else logic set up for rendering a particular html file...but oh well, good enough for now
/// Still needs to handle if we don't find the requested user
pub async fn login(
    State(database): State<Store>,
    Form(creds): Form<User>,       //The credentials will be sent back on submit of the html form
) -> Result<Response<String>, AppError> {
    let mut context = Context::new();
    //will store the context info if any errors related to login occur
    let mut error = LoginErrors {
        missing_cred: false,
        missing_cred_message: "".to_string(),
        invalid_pass: false,
        invalid_pass_message: "".to_string(),
    };

    //basic template with an empty error context
    let mut template_name = {
        context.insert("error", &error);
        "index.html"
    };

    //Missing Credentials ERROR situation
    if creds.email.is_empty() || creds.password.is_empty() {
        error.missing_cred = true;
        error.missing_cred_message = "Please enter your username and password to login".to_string();
        template_name = {
            context.insert("error", &error);
            "index.html"
        };

        let rendered = TEMPLATES
            .render(template_name, &context) //render takes all the context attached in template_name and inserts it
            .unwrap_or_else(|err| {
                error!("Template rendering error: {}", err);
                panic!()
            });

        let mut response = Response::builder()
            .status(StatusCode::FOUND)
            .body(rendered) //stick the html that gets rendered, inside the body here!
            .unwrap();

        Ok(response) //return right away on error

    } else {
        //We have credentials, but what if one is invalid?
        let existing_user = database.get_user(&creds.email).await?;
        //TODO what to do if we can't find the user?

        //use argon2 to reverse hash and verify the given password
        let is_password_correct =
            match argon2::verify_encoded(&existing_user.password, creds.password.as_bytes()) {
                //TODO removed * in &*existing_user.password
                Ok(result) => result,
                Err(_) => {
                    return Err(AppError::InternalServerError);
                }
            };

        //INVALID PASSWORD ERROR situation, we need to render the template with that error
        if !is_password_correct {
            error.invalid_pass = true;
            error.invalid_pass_message =
                "The password you entered was not correct, please try again.".to_string();
            template_name = {
                context.insert("error", &error);
                "index.html"
            };


            let rendered = TEMPLATES
                .render(template_name, &context) //render takes all the context attached in template_name and inserts it
                .unwrap_or_else(|err| {
                    error!("Template rendering error: {}", err);
                    panic!()
                });

            let mut response = Response::builder()
                .status(StatusCode::FOUND)
                .body(rendered) //stick the html that gets rendered, inside the body here!
                .unwrap();

            Ok(response) //return right away on error

        } else {
            //the password is CORRECT, and we can create the JWT token

            //Now we have validated the users creds, lets make claims piece of the token
            let claims = Claims {
                id: 0,
                email: creds.email.to_owned(),
                exp: get_timestamp_after_8_hours(),
            };

            //Build the JWT token from it's parts: Header, Payload(claims), and the Encoding keys built from the Secret
            //Thanks to jsonwebtoken library, we can easily encode if we have these three things
            //the header we use will just be set to default for now
            let token = jsonwebtoken::encode(&Header::default(), &claims, &KEYS.encoding) //here is where we use the KEYS we build in user.rs
                .map_err(|_| AppError::MissingCredentials)?;

            //we will store the token as a cookie
            let cookie = cookie::Cookie::build("jwt", token).http_only(true).finish();

            let rendered = TEMPLATES
                .render(template_name, &context) //render takes all the context attached in template_name and inserts it
                .unwrap_or_else(|err| {
                    error!("Template rendering error: {}", err);
                    panic!()
                });

            let mut response = Response::builder()
                .status(StatusCode::FOUND)
                .body(rendered) //stick the html that gets rendered, inside the body here!
                .unwrap();

            response
                .headers_mut()
                .insert(LOCATION, HeaderValue::from_static("/"));
            response.headers_mut().insert(
                SET_COOKIE,
                HeaderValue::from_str(&cookie.to_string()).unwrap(),
            );

            Ok(response) //return with NO ERROR
        }
    }
}

///Silly welcome message to test we did claims right
pub async fn protected(claims: Claims) -> Result<String, AppError> {
    Ok(format!(
        "Welcome to the PROTECTED area: \n Your claim data is: {}",
        claims
    ))
}
