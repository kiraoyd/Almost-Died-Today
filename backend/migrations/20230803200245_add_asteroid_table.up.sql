-- Add up migration script here
CREATE TABLE IF NOT EXISTS asteroids
(
    id                          serial PRIMARY KEY,
    name                        integer NOT NULL,
    diameter_meters_min         float,
    diameter_meters_max         float,
    diameter_kmeters_min        float,
    diameter_kmeters_max        float,
    diameter_miles_max          float,
    diameter_miles_min           float,
    diameter_feet_min           float,
    diameter_feet_max           float,
    is_hazardous                bool,
    close_approach_date         date,
    close_approach_datetime     timestamp,
    relative_velocity_mph       float,
    miss_distance_miles         float,
    orbiting_body               varchar(255)
);