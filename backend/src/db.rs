use std::sync::{Arc, Mutex, RwLock};
use axum::Json;
use futures::TryStreamExt;
use serde_json::Value;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tracing::info;

//add use crate statements for the structs we will write eventually
use crate::models::asteroid::Asteroid;
use crate::models::asteroid::DiameterInfo;
use crate::error::AppError;

#[derive(Clone)]
pub struct Store {
    pub conn_pool: PgPool,
}

//sets up a pool of connections to the DB URL specified in our .env
pub async fn new_pool() -> PgPool {
    let db_url = std::env::var("DATABASE_URL").unwrap();
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .unwrap()
}

impl Store {
    //set up a Store struct, that holds a pool already created and connected to some DB
    //Allows us to swap out the databases based on which one was connected
    pub fn with_pool(pool: PgPool) -> Self {
        Self { conn_pool: pool }
    }

    pub async fn get_all_asteroids(&mut self) -> Result<Vec<Asteroid>, AppError> {
        let rows = sqlx::query!(r#" SELECT * FROM asteroids"#)
            .fetch_all(&self.conn_pool)
            .await?;

        let asteroids: Vec<_> = rows
            .into_iter()
            .map(|row| {
                let size_info = DiameterInfo {
                    diameter_meters_min: row.diameter_meters_min,
                    diameter_meters_max: row.diameter_meters_max,
                    diameter_kmeters_min: row.diameter_kmeters_min,
                    diameter_kmeters_max: row.diameter_kmeters_max,
                    diameter_miles_max: row.diameter_miles_max,
                    diameter_miles_min: row.diameter_miles_min,
                    diameter_feet_min: row.diameter_feet_min,
                    diameter_feet_max: row.diameter_feet_max,
                };
                Asteroid {
                    id: Some(row.id),
                    name: Some(row.name),
                    diameter: Some(size_info),
                    is_hazardous: row.is_hazardous,
                    close_approach_date: row.close_approach_date,
                    close_approach_datetime: row.close_approach_datetime,
                    relative_velocity_mph: row.relative_velocity_mph,
                    miss_distance_miles: row.miss_distance_miles,
                    orbiting_body: row.orbiting_body,
                }
            })
            .collect();

        Ok(asteroids)
    }

    //addd all other handler functions that query the database here
}
