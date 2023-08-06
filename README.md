# Almost-Died-Today

Author: Kira Klingenberg

A Rust website built for Casey Baily and Bart Massey's 2023 Rust Web course at Portland State University.

This sit uses an axum server, tokio runtime, and sqlx to access a postgres database.

This site will query the NASA API NeoWs to grab recent data on asteroids that have passed by earth close enough for us to detect.
The landing page will display a visual representation for the asteroid which came closest to earth on today's date, or barring the existence of one today, the most recent asteroid close encounter. If there was a near miss today, you will also be greeted with a congradulatory survival message.  You will also be able to search for the asteroid with the nearest approach for a specified date.

## To Run

Clone this repository.

From the linux command line, at the root directory of the project:

```docker compose up postgres```  to start the postgres database.

```cd backend```

```cp .env.example .env```  to create a .env file locally (will update this once we move to Docker)


```cargo run``` to start the server listening.

```cd ..```
```cd client```

```cargo run``` to run the client side requests to test against the backend.

Once the frontend is finished, these instructions will change.


{"links":{"self":"
http://api.nasa.gov/neo/rest/v1/neo/54359839?api_key=ZIq46jOvcFLB3j1dX4HAWAUJ22MoOudS9HqWfzDb"},"id":"54359839","neo_reference_id":"54359839","name":"(2023 KP2)","nasa_jpl_url":"http://ssd.jpl.nasa.gov/sbdb.cgi?sstr=54
359839","absolute_magnitude_h":20.784,"estimated_diameter":{"kilometers":{"estimated_diameter_min":0.185248618,"estimated_diameter_max":0.4142285025},"meters":{"estimated_diameter_min":185.2486179515,"estimated_diamete
r_max":414.2285024775},"miles":{"estimated_diameter_min":0.115108119,"estimated_diameter_max":0.2573895788},"feet":{"estimated_diameter_min":607.77107572,"estimated_diameter_max":1359.0174400682}},"is_potentially_hazar
dous_asteroid":true,"close_approach_data":[{"close_approach_date":"2023-07-31","close_approach_date_full":"2023-Jul-31 00:51","epoch_date_close_approach":1690764660000,"relative_velocity":{"kilometers_per_second":"15.9
51318467","kilometers_per_hour":"57424.7464812389","miles_per_hour":"35681.5042788565"},"miss_distance":{"astronomical":"0.3064279038","lunar":"119.2004545782","kilometers":"45840961.717044906","miles":"28484252.778619
4628"},"orbiting_body":"Earth"}],"is_sentry_object":false}]}}
