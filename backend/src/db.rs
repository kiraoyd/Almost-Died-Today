use std::sync::{Arc, Mutex, RwLock};

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tracing::info;

//add use crate statements for the structs we will write eventually

#[derive(Clone)]
pub struct Store {
    pub conn_pool:PgPool,
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
    pub fn with_pool(pool:PgPool) -> Self {
        Self {
            conn_pool: pool,
        }
    }
    pub async fn test_db(&self) -> Result<(), sqlx::Error> {
        let row:(i64,) = sqlx::query_as("SELECT $1")
            .bind(150_i64)
            .fetch_one(&self.conn_pool)
            .await?;

        info!("{}", &row.0);

        assert_eq!(row.0, 150);
        Ok(())
    }

    //addd all other handler functions that query the database here
}