use axum::extract::FromRequestParts;
use axum::headers::authorization::Bearer;
use axum::headers::Authorization;
use axum::{async_trait, RequestPartsExt, TypedHeader};
use cookie::Cookie;
use http::request::Parts;
use jsonwebtoken::{decode, DecodingKey, EncodingKey, Validation};
use once_cell::sync::Lazy;
use std::convert::Infallible;

use crate::error::AppError;
use serde_derive::{Deserialize, Serialize};
use sqlx::decode;

//Credit for the structs and impls below: Casey Bailey 2023

///Contains login info for one user, their email and password
#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub email: String,
    pub password: String,
}

///Contains information for a user trying to sign up, includes the confirm_password information
#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct UserSignup {
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}

///Carries the data for a LoggedInUser....TODO
pub struct LoggedInUser {
    pub token: Claims,
}

///Claims information for a user, the ID ref to the user table, the email for that user, and when their logged in status expires
#[derive(Serialize, Deserialize, derive_more::Display)]
#[display(fmt = "id: {}, email: {}, exp: {}", id, email, exp)]
pub struct Claims {
    pub id: i32,
    pub email: String,
    pub exp: u64,
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AppError;

    ///Gets the claims from a token stored in the Authorization Header
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        //extract a token claims from our Authorization header, established during login
        let jwt_token = parts
            .headers
            .get("cookie")
            .and_then(|value| Cookie::parse(value.to_str().unwrap_or_default()).ok())
            .and_then(|cookie| {
                if cookie.name() == "jwt" {
                    Some(cookie.value().to_string())
                } else {
                    None
                }
            })
            .ok_or(AppError::InvalidToken)?;

        let token_data = decode::<Claims>(&jwt_token, &KEYS.decoding, &Validation::default())
            .map_err(|_| AppError::InvalidToken)?;

        Ok(token_data.claims)
    }
}

//TODO purpose of having this struct?
pub struct OptionalClaims(pub Option<Claims>);

#[async_trait]
impl<S> FromRequestParts<S> for OptionalClaims
where
    S: Send + Sync,
{
    type Rejection = Infallible; // Use Infallible since we're not rejecting the request

    ///Gets the claims from the jwt cookie
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Try extracting the JWT token from the "jwt" cookie
        let jwt_token = parts
            .headers
            .get("cookie")
            .and_then(|value| Cookie::parse(value.to_str().unwrap_or_default()).ok())
            .and_then(|cookie| {
                if cookie.name() == "jwt" {
                    Some(cookie.value().to_string())
                } else {
                    None
                }
            });

        // If we have a JWT token, try to decode it
        if let Some(jwt) = jwt_token {
            if let Ok(token_data) = decode::<Claims>(&jwt, &KEYS.decoding, &Validation::default()) {
                return Ok(OptionalClaims(Some(token_data.claims)));
            }
        }

        Ok(OptionalClaims(None))
    }
}

///Encoding and Decoding key values for a JWT token
pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

///Build new keys using the SECRET, this essentially takes the secret as u8 bytes and turns it into our pair of encoding and decoding keys
/// This only needs to be called once, when we define the the pub static variable: KEYS
impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

///Define the static variable KEYS
pub static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("MISSING JWT SECRET!");

    Keys::new(secret.as_bytes()) //fills out our keys so they can now be used as KEYS here-on out
});
