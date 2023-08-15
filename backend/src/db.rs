use axum::Json;
use chrono::{Duration, Local, NaiveDate, NaiveDateTime};
use futures::TryStreamExt;
use serde_json::Value;
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Row};
use std::sync::{Arc, Mutex, RwLock};
use tracing::info;

//add use crate statements for the structs we will write eventually
use crate::error::AppError;
use crate::models::asteroid::{Asteroid, AsteroidId, DiameterInfo, FloatNum, NearEarthObject};
use crate::models::user::{User, UserSignup};
use crate::pull_nasa_api_data;

//templating stuff
use crate::models::page::PagePackage;

///Holds the various types of databases/storage, could contain a postgres DB, a Vec of structs, etc.
#[derive(Clone)]
pub struct Store {
    pub conn_pool: PgPool,
}

///sets up a pool of connections to the DB URL specified in our .env
pub async fn new_pool() -> PgPool {
    let db_url = std::env::var("DATABASE_URL").unwrap();
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .unwrap()
}

//Implement all functionality to and from the DB here
impl Store {
    ///set up a Store struct, that holds a pool already created and connected to some DB
    ///Allows us to swap out the databases based on which one was connected
    pub fn with_pool(pool: PgPool) -> Self {
        Self { conn_pool: pool }
    }

    ///grabs one user from the users table based on the specified email address
    pub async fn get_user(&self, email: &str) -> Result<User, AppError> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT email, password FROM users WHERE email = $1
            "#,
        )
        .bind(email)
        .fetch_one(&self.conn_pool)
        .await?;

        Ok(user)
    }

    ///Adds one user to the users table based off their supplied email and password in the UserSignup struct
    pub async fn create_user(&self, user: UserSignup) -> Result<Json<Value>, AppError> {
        //TODO should encrypt passswords using bcrypt
        let result = sqlx::query("INSERT INTO users(email, password) VALUES ($1, $2)")
            .bind(&user.email)
            .bind(&user.password)
            .execute(&self.conn_pool)
            .await
            .map_err(|_| AppError::InternalServerError)?;

        if result.rows_affected() < 1 {
            Err(AppError::InternalServerError)
        } else {
            Ok(Json(
                serde_json::json!({"message": "User created successfully!"}),
            ))
        }
    }

    ///Retrieves all asteroids in the asteroid table, returns the data in a Vec of Asteroid structs
    pub async fn get_all_asteroids(&mut self) -> Result<Vec<Asteroid>, AppError> {
        let rows = sqlx::query!(r#" SELECT * FROM asteroids"#)
            .fetch_all(&self.conn_pool)
            .await?;

        //For every row, collect each row after mapping it to an Asteroid struct, into a Vec
        let asteroids: Vec<_> = rows
            .into_iter() //to do this, rows needs to be turned into an iterator
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
                    id: AsteroidId(row.id),
                    name: row.name.to_string(),
                    diameter: Some(size_info),
                    is_hazardous: row.is_hazardous.map(|x| x), //map returns None if x is of no value
                    close_approach_date: row.close_approach_date.map(|x| x),
                    close_approach_datetime: row.close_approach_datetime.map(|x| x),
                    relative_velocity_mph: FloatNum(row.relative_velocity_mph.map(|x| x)),
                    miss_distance_miles: FloatNum(row.miss_distance_miles.map(|x| x)),
                    orbiting_body: row.orbiting_body.map(|x| x),
                }
            })
            .collect();

        Ok(asteroids)
    }

    ///Posts Vec of Asteroids to our database, after grabbing the data from NASA's NeoW's API
    pub async fn post_current_from_nasa_api(&mut self) -> Result<Vec<Asteroid>, AppError> {
        let today: NaiveDate = Local::now().naive_utc().into(); //now() returns a datetime

        //Query NASA's API for todays date, plus 7 days prior
        let asteroids = pull_nasa_api_data(today).await?;

        //Now we have a Vec<NearEarthObjects>, inside asteroids, lets get the relevant data posted to the table

        let mut added_asteroids: Vec<Asteroid> = Vec::new(); //will add each posted Asteroid for the response

        //go through each asteroid we got from NASA (that's in the Vec<NearEarthObjects>) and parse into the correct formats to be posted to the database
        for asteroid in asteroids {
            //convert from string to NaiveDate
            let date = asteroid.close_approach_data[0]
                .close_approach_date
                .clone()
                .unwrap();
            let parsed_date = NaiveDate::parse_from_str(&date, "%Y-%m-%d");
            //match, if we got a datet back, set approach_date to it, otherwise set a default date
            let approach_date: NaiveDate = match parsed_date {
                Ok(date) => date,
                Err(err) => {
                    println!("Error matching dates: {}", err);
                    NaiveDate::from_ymd_opt(2000, 1, 1).unwrap() //default TODO
                }
            };

            //convert from string to NaiveDateTime
            let datetime = asteroid.close_approach_data[0]
                .close_approach_date_full
                .clone()
                .unwrap();
            let parsed_datetime = NaiveDateTime::parse_from_str(&datetime, "%Y-%b-%d %H:%M"); //converts string to NaiveDateTime, retunrs Result<NDT, err>
                                                                                              //match, if we got a datetime back, set approach_datetime to it, otherwise set a default date
            let approach_datetime: NaiveDateTime = match parsed_datetime {
                Ok(datetime) => datetime,
                Err(err) => {
                    println!("Error matching datetimes: {}", err);
                    NaiveDateTime::from_timestamp_opt(0, 0).unwrap() //use default if we error
                }
            };

            //these variable make the query easier to read
            let name = asteroid.name.unwrap();
            let size_meters_min = asteroid.estimated_diameter.meters.estimated_diameter_min;
            let size_meters_max = asteroid.estimated_diameter.meters.estimated_diameter_max;
            let size_kmeters_min = asteroid
                .estimated_diameter
                .kilometers
                .estimated_diameter_min;
            let size_kmeters_max = asteroid
                .estimated_diameter
                .kilometers
                .estimated_diameter_max;
            let size_miles_max = asteroid.estimated_diameter.miles.estimated_diameter_min;
            let size_miles_min = asteroid.estimated_diameter.miles.estimated_diameter_min;
            let size_feet_min = asteroid.estimated_diameter.feet.estimated_diameter_min;
            let size_feet_max = asteroid.estimated_diameter.feet.estimated_diameter_max;
            let hazardous = asteroid.is_potentially_hazardous_asteroid;
            //Remember, a FloatNum is a tuple struct, you can access the f64 inside with floatnumvar.0
            let velocity_mph = asteroid.close_approach_data[0]
                .relative_velocity
                .miles_per_hour
                .0
                .unwrap();
            let miss_distance_miles = asteroid.close_approach_data[0]
                .miss_distance
                .miles
                .0
                .unwrap();
            let orbiting_body = asteroid.close_approach_data[0]
                .orbiting_body
                .clone()
                .unwrap();

            //POST this asteroids data to the database
            let res = sqlx::query!(
            r#" INSERT INTO asteroids (name, diameter_meters_min, diameter_meters_max, diameter_kmeters_min, diameter_kmeters_max, diameter_miles_max, diameter_miles_min, diameter_feet_min, diameter_feet_max, is_hazardous, close_approach_date, close_approach_datetime, relative_velocity_mph, miss_distance_miles, orbiting_body)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
            RETURNING * ;"#,
                name, size_meters_min,size_meters_max, size_kmeters_min ,size_kmeters_max,size_miles_max, size_miles_min, size_feet_min, size_feet_max, hazardous, approach_date, approach_datetime, velocity_mph, miss_distance_miles, orbiting_body,
                )
                .fetch_one(&self.conn_pool)
                .await?;

            //Make an Asteroid struct and add it to a Vec of Asteroids
            //An Asteroid contains a DiameterInfo struct
            let size_info = DiameterInfo {
                diameter_meters_min: res.diameter_meters_min,
                diameter_meters_max: res.diameter_meters_max,
                diameter_kmeters_min: res.diameter_kmeters_min,
                diameter_kmeters_max: res.diameter_kmeters_max,
                diameter_miles_max: res.diameter_miles_max,
                diameter_miles_min: res.diameter_miles_min,
                diameter_feet_min: res.diameter_feet_min,
                diameter_feet_max: res.diameter_feet_max,
            };
            let added = Asteroid {
                id: AsteroidId(res.id),
                name: res.name.to_string(),
                diameter: Some(size_info),
                is_hazardous: res.is_hazardous.map(|x| x), //map returns None if x is of no value
                close_approach_date: res.close_approach_date.map(|x| x),
                close_approach_datetime: res.close_approach_datetime.map(|x| x),
                relative_velocity_mph: FloatNum(res.relative_velocity_mph.map(|x| x)),
                miss_distance_miles: FloatNum(res.miss_distance_miles.map(|x| x)),
                orbiting_body: res.orbiting_body.map(|x| x),
            };

            added_asteroids.push(added);
        }

        //After all asteroids from NASA process, post to the DB and are added to the added_asteroids vec, we can return it as there response
        Ok(added_asteroids)
    }

    ///Pulls all asteroids from the database that match the requested date, and are labeled as potential hazardous
    /// Parses the results to find the asteroid with the closest near miss
    /// To be used in a search function on the site
    pub async fn get_closest_by_date(
        &mut self,
        today: String,
    ) -> Result<Option<Asteroid>, AppError> {
        let parse_from_str = NaiveDate::parse_from_str;

        let date = parse_from_str(today.as_str(), "%Y-%m-%d").unwrap();
        let rows = sqlx::query!(
            r#" SELECT * FROM asteroids WHERE close_approach_date = $1 AND is_hazardous = true "#,
            date
        )
        .fetch_all(&self.conn_pool)
        .await?; //discard error info

        //What if the query comes back empty?
        if rows.is_empty() {
            println!("Nothing there....");
            Ok(None)
        } else {
            let mut asteroids: Vec<_> = rows
                .into_iter()
                .map(|row| {
                    //We only want to collect asteroids that have values for their miss distance feild

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
                        id: AsteroidId(row.id),
                        name: row.name.to_string(),
                        diameter: Some(size_info),
                        is_hazardous: row.is_hazardous.map(|x| x), //map returns None if x is of no value
                        close_approach_date: row.close_approach_date.map(|x| x),
                        close_approach_datetime: row.close_approach_datetime.map(|x| x),
                        relative_velocity_mph: FloatNum(row.relative_velocity_mph.map(|x| x)),
                        miss_distance_miles: FloatNum(row.miss_distance_miles.map(|x| x)),
                        orbiting_body: row.orbiting_body.map(|x| x),
                    }
                })
                .collect();

            //total_cmp sorts the Vec from decreasing to increasing order
            asteroids.sort_by(|a, b| {
                a.miss_distance_miles
                    .0 //Floatnum type
                    .unwrap()
                    .total_cmp(&b.miss_distance_miles.0.unwrap()) //FloatNum type
            });
            //iterate from the start of asteroids until we find the first asteroid with a valid miss_distance_miles value
            let mut index = 0;
            let mut found = false;
            let mut closest_call = asteroids[index].clone();
            while index < asteroids.len() && !found {
                match asteroids[index].miss_distance_miles.0 {
                    //FloatNum type
                    Some(x) => {
                        found = true;
                        closest_call = asteroids[index].clone();
                    }
                    None => {
                        found = false;
                        index += 1;
                    }
                }
            }

            Ok(Some(closest_call.clone())) //return the asteroid
        }
    }

    ///This site only has one main page for now, future features will allow for a search function
    /// So this function serves to grab the current asteroid (the one that came closest) of the day, and return it in a PagePackage
    pub async fn get_main_page(&mut self) -> Result<PagePackage, AppError> {
        //get todays date
        let mut today = Local::now().naive_utc().date(); //now() returns datetime, .date() grabs just the date

        //Since calling get_closest_by_date may mutate the value of "today", self needs to be mutable here as well
        let mut near_miss_result = self.get_closest_by_date(today.to_string()).await?; //To call another impl function for the same struct, use self.functionname()
        let mut near_miss = near_miss_result.clone();
        let one_day = Duration::days(1);
        let limit_back = 7; //how many days back we will check from today's date
        let mut count = 0;

        //in the event we get None back in our Option<Asteroid> from get_closest_by_date....keep trying
        while near_miss.is_none() && count < limit_back {
            //set before to be the day previous to the last we tried
            today -= one_day; //subtract one day from today
            count += 1;
            near_miss_result = self.get_closest_by_date(today.to_string()).await?; //try again with a previous date
            near_miss = near_miss_result.clone();
        }

        //Initialize and empty PagePackage to be mutated
        let mut package = PagePackage {
            asteroid: None,
            message: "empty".to_string(),
            has_data: false,
        };

        //set up the pagePackage accordingly
        match near_miss {
            Some(data) => {
                let todays_message = format!("Everyone almost died on: {}!", today).to_string();

                package.asteroid = Some(data);
                package.message = todays_message;
                package.has_data = true;
            }
            //If we checked back a week with no results...
            None => {
                let survived = "No near misses today!".to_string();
                package.message = survived;
            }
        }
        Ok(package)
    }
}
