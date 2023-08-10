use crate::models::asteroid::Asteroid;

use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_derive::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PagePackage {
    pub asteroid: Asteroid,
    pub message: String,
}


//make it so we can transform a PagePackage into JSON
impl IntoResponse for PagePackage {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}