use std::sync::{Arc, Mutex, RwLock};
use axum::Json;
use futures::TryStreamExt;
use serde_json::Value;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tracing::info;

//add use crate statements for the structs we will write eventually
use crate::models::asteroid::{Asteroid, AsteroidId};
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
                    id: row.id.into(),
                    name: row.name,
                    diameter: Some(size_info),
                    is_hazardous: row.is_hazardous.map(|x| x), //map returns None if x is of no value
                    close_approach_date: row.close_approach_date.map(|x| x),
                    close_approach_datetime: row.close_approach_datetime.map(|x| x),
                    relative_velocity_mph: row.relative_velocity_mph.map(|x| x),
                    miss_distance_miles: row.miss_distance_miles.map(|x| x),
                    orbiting_body: row.orbiting_body.map(|x| x),
                }
            })
            .collect();

        Ok(asteroids)
    }

    ///Pulls all asteroids from the database that match the requested date, and are labeled as potential hazardous
    /// Parses the results to find the asteroid with the closest near miss
    pub async fn get_closest_by_date(&mut self, today:T,) -> Result<Asteroid, AppError> {
        let date = today;
        let rows = sqlx::query!(r#" SELECT * FROM asteroids WHERE close_approach_date = $1 AND is_hazardous = true "#, today)
            .fetch_all(&self.conn_pool)
            .await?;

        //iterate through rows and pick out the asteroid with the closest near miss distance
        Ok()

    }

    ///Pulls all asteroids from the database that match the requested date, and are labeled as potential hazardous
    /// Parses the results to find the biggest asteroid
    pub async fn get_biggest_by_date(&mut self) -> Result<Asteroid, AppError> {
        let rows = sqlx::query!(r#" SELECT * FROM asteroids WHERE close_approach_date = $1 AND is_hazardous = true "#, today)
            .fetch_all(&self.conn_pool)
            .await?;

        //iterate through rows and pick out the asteroid with the biggest max diameter
    }
}
