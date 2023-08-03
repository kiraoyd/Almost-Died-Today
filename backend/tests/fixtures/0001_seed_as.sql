-- Add migration script here
DELETE FROM asteroids; --start fresh

--reset primary key id to 1
SELECT setval(pg_get_serial_sequence('asteroids', 'id'), 1, false);

INSERT INTO asteroids(name, diameter_meters_min, diameter_meters_max,diameter_kmeters_min, diameter_kmeters_max, diameter_miles_max, diamter_miles_min, diameter_feet_min, diameter_feet_max, is_hazardous, close_approach_date, close_approach_datetime, relative_velocity_mph, miss_distance_miles,orbiting_body)
VALUES (465633, 213.0860292484, 476.474846455, 0.2130860292, 0.4764748465, 0.1324054791, 0.2960676518,699.1011681995, 1563.2377352435, true,  '1901-03-15', '1901-Mar-15 22:57', 53502.3857679434, 45325522.90133255, 'Earth');