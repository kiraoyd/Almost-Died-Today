use crate::make_db_id;
use chrono::{NaiveDate, NaiveDateTime};
use derive_more::Display;
use serde_derive::{Deserialize, Serialize};

use std::str::FromStr;
use std::num::{ParseIntError, ParseFloatError};
use serde_aux::prelude::*;

use std::collections::HashMap;
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
//TODO I need the Asteroid struct to match EXACTLY with the JSON data, having chatGPT do it for me based on the JSON data returning from the response
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Asteroid {
    pub links: Links,
    pub element_count: Option<i32>,
    pub near_earth_objects: HashMap<String, Vec<NearEarthObject>>,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Links {
    pub next: Option<String>,
    pub previous: Option<String>,
    pub self_link: Option<String>,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NearEarthObject {
    pub links: Links,
    pub id: Option<String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub neo_reference_id: FloatNum,
    pub name: Option <String>,
    pub nasa_jpl_url: Option<String>,
    pub absolute_magnitude_h: Option<f64>,
    pub estimated_diameter: EstimatedDiameter,
    pub is_potentially_hazardous_asteroid: bool,
    pub close_approach_data: Vec<CloseApproachData>,
    pub is_sentry_object: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EstimatedDiameter {
    pub kilometers: Diameter,
    pub  meters: Diameter,
    pub miles: Diameter,
    pub feet: Diameter,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Diameter {
    pub estimated_diameter_min: Option<f64>,
    pub estimated_diameter_max: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CloseApproachData {
    pub close_approach_date: Option<String>,
    pub close_approach_date_full: Option<String>,
    pub epoch_date_close_approach: Option<i64>,
    pub relative_velocity: RelativeVelocity,
    pub miss_distance: MissDistance,
    pub orbiting_body: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RelativeVelocity {
    pub kilometers_per_second: Option<String>,
    pub kilometers_per_hour: Option<String>,
    pub miles_per_hour: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MissDistance {
    pub astronomical: Option<String>,
    pub lunar: Option<String>,
    pub kilometers: Option<String>,
    pub miles: Option<String>,
}












/*
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

 */



