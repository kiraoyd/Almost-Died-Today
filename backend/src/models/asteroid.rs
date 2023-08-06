use crate::make_db_id;
use chrono::{NaiveDate, NaiveDateTime};
use derive_more::Display;
use serde_derive::{Deserialize, Serialize};

//See the NASAAPI for JSON format of the NeoW's response

//use the macro we build to create the AsteroidId struct type and impl all need functionality for it
make_db_id!(AsteroidId);

#[derive(Clone, Debug, Display, Serialize, Deserialize, sqlx::FromRow)]
#[display(
    fmt = "id: {:?}, name: {:?}, diameter: {:?}, is_hazardous: {:?}, close_approach_date: {:?}, close_approach_datetime: {:?}, relative_velocity_mph: {:?}, miss_distance_miles: {:?}, orbiting_body: {:?}  ",
    id,
    name,
    diameter,
    is_hazardous,
    close_approach_date,
    close_approach_datetime,
    relative_velocity_mph,
    miss_distance_miles,
    orbiting_body
)]
pub struct Asteroid {
    pub id: Option<AsteroidId>, // TODO: Making this PKID an Option and trying to map it in db.rs creates a buggy compiler error
    pub name: Option<i32>, //TODO: This is also a not null value, will likly trigger the same thing if left and mapped
    pub diameter: Option<DiameterInfo>,
    pub is_hazardous: Option<bool>,
    pub close_approach_date: Option<NaiveDate>,
    pub close_approach_datetime: Option<NaiveDateTime>,
    pub relative_velocity_mph: Option<f64>,
    pub miss_distance_miles: Option<f64>,
    pub orbiting_body: Option<String>,
}

#[derive(Clone, Debug, Display, Serialize, Deserialize, sqlx::FromRow)]
#[display(
    fmt = "diameter_meters_min: {:?}, diameter_meters_max: {:?}, diameter_kmeters_min: {:?}, diameter_kmeters_max: {:?}, diameter_miles_max: {:?}, diameter_miles_min: {:?}, diameter_feet_min: {:?}, diameter_feet_max: {:?} ",
    diameter_meters_min,
    diameter_meters_max,
    diameter_kmeters_min,
    diameter_kmeters_max,
    diameter_miles_max,
    diameter_miles_min,
    diameter_feet_min,
    diameter_feet_max
)]
pub struct DiameterInfo {
    pub diameter_meters_min: Option<f64>,
    pub diameter_meters_max: Option<f64>,
    pub diameter_kmeters_min: Option<f64>,
    pub diameter_kmeters_max: Option<f64>,
    pub diameter_miles_max: Option<f64>,
    pub diameter_miles_min: Option<f64>,
    pub diameter_feet_min: Option<f64>,
    pub diameter_feet_max: Option<f64>,
}


