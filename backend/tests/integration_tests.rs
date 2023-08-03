use http::{Request, StatusCode};
use hyper::Body;
use sqlx::PgPool;
use tower::ServiceExt;

use backend::routes::main_routes::app;

async fn test_db(db_pool:PgPool) {
    let app = app(db_pool).await;

    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::GET)
                .uri("/asteroid")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(serde_json::to_string(&asteroid).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    asswer_eq!(response.status(), StatudCode::OK);
}