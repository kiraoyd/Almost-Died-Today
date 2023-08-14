use crate::models::asteroid::Asteroid;

use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_derive::{Deserialize, Serialize};
use derive_more::Display;


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


//make it so we can transform a PagePackage into JSON
impl IntoResponse for PagePackage {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}