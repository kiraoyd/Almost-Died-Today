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
pub struct FloatNum(pub Option<f64>);

impl FromStr for FloatNum {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<FloatNum, Self::Err>{
        Ok(FloatNum(Some(f64::from_str(s)?)))
    }
}


///Holds the response from NASA's NeoW's API
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NasaData {
    pub links: Links,
    pub element_count: Option<i32>,
    pub near_earth_objects: HashMap<String, Vec<NearEarthObject>>,
}

///Defines the possible feilds in the Links feild of NasaData
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Links {
    pub next: Option<String>,
    pub previous: Option<String>,
    pub self_link: Option<String>,
}

///Holds all data pertaining to a specific asteroid close call
/// Will be used in all routing and has all info needed for our database
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

///Contains information about diameter min and max for these specific units of measurement
//TODO, do these need to be options too?
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EstimatedDiameter {
    pub kilometers: Diameter,
    pub  meters: Diameter,
    pub miles: Diameter,
    pub feet: Diameter,
}


///Contains the min and max values for some unit of measurements diameter values
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Diameter {
    pub estimated_diameter_min: Option<f64>,
    pub estimated_diameter_max: Option<f64>,
}

///Contains the information contained in the CloseApproachData section of a NearEarthObject
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CloseApproachData {
    pub close_approach_date: Option<String>,
    pub close_approach_date_full: Option<String>,
    pub epoch_date_close_approach: Option<i64>,
    pub relative_velocity: RelativeVelocity,
    pub miss_distance: MissDistance,
    pub orbiting_body: Option<String>,
}


///Contains values for relative velocity in three different units of measurement
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RelativeVelocity {
    pub kilometers_per_second: Option<String>,
    pub kilometers_per_hour: Option<String>,
    pub miles_per_hour: Option<String>,
}


///Contains values for miss distance in four units of measurement
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MissDistance {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub astronomical: FloatNum,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub lunar: FloatNum,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub kilometers: FloatNum,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub miles: FloatNum,
}


///Struct for modeling the NASA data to the AlmostDiedToday database (only contains the info from NasaData that we actually need)
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
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub id: i32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub name: String,
    pub diameter: Option<DiameterInfo>,
    pub is_hazardous: Option<bool>,
    pub close_approach_date: Option<NaiveDate>,
    pub close_approach_datetime: Option<NaiveDateTime>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub relative_velocity_mph: FloatNum,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub miss_distance_miles: FloatNum,
    pub orbiting_body: Option<String>,
}


///Contains the specific diameter info for a particular asteroid
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





