use crate::models::asteroid::Asteroid;

use crate::error::AppError;
use axum::response::{IntoResponse, Response};
use axum::Json;
use derive_more::Display;
use serde_derive::{Deserialize, Serialize};

///Packs up the information needed to supply context to our pages.html Tera template file
#[derive(Serialize, Deserialize, Debug, Clone, Display)]
#[display(
    fmt = "asteroid: {:?}, message: {:?}, has_data: {:?}",
    asteroid,
    message,
    has_data
)]
pub struct PagePackage {
    pub asteroid: Option<Asteroid>,
    pub message: String,
    pub has_data: bool,
}

///Makes it so we can transform a PagePackage into JSON
impl IntoResponse for PagePackage {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

// #[derive(Serialize, Deserialize, Debug, Clone, Display)]
// #[display(
// fmt = "err_message: {:?}, has_error: {:?}",
// err_message,
// has_error
// )]

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginErrors {
    pub missing_cred: bool,
    pub missing_cred_message: String,
    pub invalid_pass: bool,
    pub invalid_pass_message: String,
}
