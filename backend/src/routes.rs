use axum::response::Response;
use axum::Router;
use axum::routing::*;
use http::StatusCode;
use hyper::Body;
use sqlx::PgPool;
use tracing::info;

use crate::{handlers, layers};
use crate::db::Store;
use crate::handlers::root;

//takes in a pool, sets up the db seeds, layers on middlewares, and returns a new router
pub async fn app(pool:PgPool) -> Router {
    let db = Store::with_pool(pool);

    info!("Seeded database");

    //Middlewares
    let(cors_layer, trace_layer) = layers::get_layers();

    Router::new()
        .route("/", get(root))
        //add new routes here, reads top to bottom
        .route("/*_", get(handle_404)) //if no other rouote is found, we have a page note found 404 error
        .layer(cors_layer)
        .layer(trace_layer)
        .with_state(db)
}

async fn handle_404() -> Response<Body>{
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("The requested page could not be found"))
        .unwrap()
}