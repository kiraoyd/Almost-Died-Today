//NeoW's JSON for asteroids on one date:
{
    "links":{"next":"http://api.nasa.gov/neo/rest/v1/feed?start_date=2015-09-08&end_date=2015-09-09&detailed=false&api_key=DEMO_KEY","previous":"http://api.nasa.gov/neo/rest/v1/feed?start_date=2015-09-06&end_date=2015-09-07&detailed=false&api_key=DEMO_KEY","self":"http://api.nasa.gov/neo/rest/v1/feed?start_date=2015-09-07&end_date=2015-09-08&detailed=false&api_key=DEMO_KEY" },
    "element_count":25,
    "near_earth_objects":
        {"2015-09-08":[
            {
                "links":{"self":"http://api.nasa.gov/neo/rest/v1/neo/2465633?api_key=DEMO_KEY"},
                "id":"2465633",
                "neo_reference_id":"2465633",
                "name":"465633 (2009 JR5)",
                "nasa_jpl_url":"http://ssd.jpl.nasa.gov/sbdb.cgi?sstr=2465633",
                "absolute_magnitude_h":20.48,
                "estimated_diameter":{
                    "kilometers":{"estimated_diameter_min":0.2130860292,"estimated_diameter_max":0.4764748465},
                    "meters":{"estimated_diameter_min":213.0860292484,"estimated_diameter_max":476.474846455},
                    "miles":{"estimated_diameter_min":0.1324054791,"estimated_diameter_max":0.2960676518},
                    "feet":{"estimated_diameter_min":699.1011681995,"estimated_diameter_max":1563.2377352435}},
                "is_potentially_hazardous_asteroid":true,
                "close_approach_data":[{"close_approach_date":"2015-09-08","close_approach_date_full":"2015-Sep-08 20:28","epoch_date_close_approach":1441744080000,"relative_velocity":{"kilometers_per_second":"18.127936605","kilometers_per_hour":"65260.5717781091","miles_per_hour":"40550.3813917923"}, "miss_distance":{"astronomical":"0.3027469593","lunar":"117.7685671677","kilometers":"45290300.260256691","miles":"28142087.6157806958"},"orbiting_body":"Earth"}],
                "is_sentry_object":false
            },
            {...<second asteroids data>...}, 
            {...<third, etc>...}  
        ]},
        {...Next date...},
        {...next date...},
}