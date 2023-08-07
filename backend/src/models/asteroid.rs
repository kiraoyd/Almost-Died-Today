use crate::make_db_id;
use chrono::{NaiveDate, NaiveDateTime};
use derive_more::Display;
use serde_derive::{Deserialize, Serialize};

use std::str::FromStr;
use std::num::{ParseIntError, ParseFloatError};
use serde_aux::prelude::*;
//See the NASAAPI for JSON format of the NeoW's response

//use the macro we build to create the AsteroidId struct type and impl all need functionality for it
make_db_id!(AsteroidId);

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Clone)]
pub struct FloatNum(Option<f64>);

impl FromStr for FloatNum {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<FloatNum, Self::Err>{
        Ok(FloatNum(Some(f64::from_str(s)?)))
    }
}
#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Clone)]
pub struct Float(f64);

impl FromStr for Float {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Float, Self::Err>{
        Ok(Float(f64::from_str(s)?))
    }
}
//TODO I need the Asteroid struct to match EXACTLY with the JSON data

#[derive(Clone, Debug, Display, Serialize, Deserialize, sqlx::FromRow)]
#[display(
    fmt = "id: {:?}, name: {:?}, diameter: {:?}, is_hazardous: {:?}, close_approach_date: {:?}, close_approach_datetime: {:?}, relative_velocity_mph: {:?}, miss_distance_miles: {:?}, orbiting_body: {:?}  ",
    id,
    name,
    diameter,
    is_hazardous,
    close_approach_date,
    close_approach_datetime,
    relative_velocity,
    miss_distance,
    orbiting_body
)]
pub struct Asteroid {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub id: Float, // TODO: Making this PKID an Option and trying to map it in db.rs creates a buggy compiler error
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub name: String, //TODO: This is also a not null value, will likly trigger the same thing if left and mapped
    pub diameter: Option<DiameterInfo>,
    pub is_hazardous: Option<bool>,
    pub close_approach_date: Option<NaiveDate>,
    pub close_approach_datetime: Option<NaiveDateTime>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub close_approach_data: FloatNum,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub miss_distance: FloatNum,
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
    pub diameter_meters_min: FloatNum,
    pub diameter_meters_max: FloatNum,
    pub diameter_kmeters_min: FloatNum,
    pub diameter_kmeters_max: FloatNum,
    pub diameter_miles_max: FloatNum,
    pub diameter_miles_min: FloatNum,
    pub diameter_feet_min: FloatNum,
    pub diameter_feet_max: FloatNum,
}



