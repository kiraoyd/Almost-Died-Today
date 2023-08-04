use http::{Request, StatusCode};
use hyper::Body;
use sqlx::PgPool;
use tower::ServiceExt;

use backend::routes::main_routes::app;
use backend::models::asteroid::Asteroid;

#[sqlx::test(fixtures("0001_seed_as"))]
async fn test_get_asteroids(db_pool: PgPool) {
    let app = app(db_pool).await;

    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::GET)
                .uri("/asteroids")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let asteroids: Vec<Asteroid> = serde_json::from_slice(&body).unwrap();
    assert!(!asteroids.is_empty());
}
