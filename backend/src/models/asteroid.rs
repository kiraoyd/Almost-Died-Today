use chrono::{NaiveDate, NaiveDateTime};
//See the NASAAPI for JSON format of the NeoW's response

pub enum Planet{
    Earth,
    Mars,
    Venus,

}
pub struct asteroid {
    pub id: Option<AsteroidId>,
    pub name: String,
    pub diameter: Diameter_info,
    pub is_hazardous: bool,
    pub close_approach_date: NaiveDate,
    pub close_approach_datetime:NaiveDateTime,
    pub relative_velocity_mph:f64,
    pub miss_distance_miles:f64,
    pub orbiting_body: Planet,
}

pub struct Diameter_info{
    pub diameter_meters_min:f64,
    pub diameter_meters_max:f64,
    pub diameter_kmeters_min:f64,
    pub diameter_kmeters_max:f64,
    pub diameter_miles_max:f64,
    pub diamter_miles_min:f64,
    pub diameter_feet_min:f64,
    pub diameter_feet_max:f64,
}

//use the macro we build to create the AsteroidId struct type and impl all need functionality for it
make_db_id!(AsteroidId);

