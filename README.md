# Almost-Died-Today

Author: Kira Klingenberg

A Rust website built for Casey Baily and Bart Massey's 2023 Rust Web course at Portland State University.

This sit uses an axum server, tokio runtime, and sqlx to access a postgres database.

This site will query the NASA API NeoWs to grab recent data on asteroids that have passed by earth close enough for us to detect.
The landing page will display a visual representation for the asteroid which came closest to earth on today's date, or barring the existence of one today, the most recent asteroid close encounter.
You will also be greeted with a congradulatory survival message.  


Features still to be implemented:

- Being able to search for the asteroid with the nearest approach for a specified date (the route for this exists, the frontend just needs to support it)
- Implement user/admin role distinctions
- Provide logout functionality
- Place the NASA API query on a timed fetch.
- Render a completely different page style if no asteroid near miss is found within the last week.


## To Run

Clone this repository.

From the linux command line, at the root directory of the project:

```docker compose up postgres```  to start the postgres database.

```cd backend```

```cp .env.example .env```  to create a .env file locally (will update this once we move to Docker)

```sqlx database drop``` to refresh the database.

```sqlx database create``` to start the database.

```sqlx migrate run --ignore-missing``` run the migrations.

```sqlx migrate run --source ./tests/fixtures --ignore-missing``` seed the tables.

```cargo run``` to start the server listening.

Navigate to: ```localhost:3000/```

When prompted, sign in using:

Username: me@gmail.com
Password: banana

You should now see basic HTML reporting which(if any) asteroid almost killed us all.

## Project Summary

(To see the full journal of my journey building this site, including what went wrong and how I fixed it, please see JOURNAL.md. Consider this blurb a TL:DR. )

I was able to successfully drive the MVP for this site: 
On first navigation to the home page, you are presented with the option to Login or Register for the first time.
Registration will add your username (email) and password to the User table in the site's database, if it does not already exist.
Once registered with the site, a user can use the login form to sign in.
The login form sends their provided information to the backend and queries the database to verify it.
If verified, a JWT token is created and stored as a browser cookie (with an 8 hour expiration time).
Once logged in, the user is routed to the main page of the site.
The main page displays the near earth object recorded for today's date, or the most recent date within a 7 day period, that came the closest to hitting earth.
Some facts about the asteroid, such as how big it was, and how fast it was travelling, are also shown.

The backend runs a query to the NASA NeoW's API to grab 7-days worth of data and dump it to our database.
It performs this action once everytime the server is started running.
I would like to update this to periodically and automatically update as the server is running, on some sort of regular schedule.

To find the asteroid among that data that was the most recent to pass near earth, and was the closest to earth of those most recent asteroids, 
the backend queries our database for all the asteroids matching todays date using the get_closest_by_date handler.
This handler will grab all asteroids from the database that match todays date. 
If it finds any, they get collected up and then sorted and parsed based on approach distance, so the handler can pick out and return the one with the smallest (closest) appraoch distance.
If no Asteroids are found for todays date, it returns None.
In this case the backend will continue to call this handler, but with dates going back to up to 7 days prior to todays date.
As soon as the result comes back with one or more asteroids, the queries end.
If no asteroids are found after shifting the dates back 7 days, the PagePackage is sent back with None for the Asteroid feild, and a message reporting no near misses were found recently.
If an asteroid is found, it's data is set to the asteroid field in the PagePackage, and it is returned.

Once the page package reaches the root handler, asteroid data to be displayed on the frontend is extracted and stored as context, and the templates are rendered.
If the PagePackage contains not asteroid, a message tailored to this situation is displayed in leiu of any Asteroid Data.
Each time the main page is refreshed, the database is queried again for the right asteroid.
Beacuse I don't have the query to NASA set up on a schedule yet, there is no reason for the displayed asteroid to change unless the server is stopped and restarted on a new day.
Likely I will want the scheduled query to NASA to run every hour or so, in case any new asteroid data was added to their API for todays date.

There are a few requirements I chose to omit, or just didn't get to in time:

- Since my site is more of an information hub, rather than an interactive place, I didn't come up with a way to has users save and delete some kind of choice.
If I wanted to add something like this in the future, I suppose I could add a feature to allow users to save the asteroid data displayed on some particular day, so they could go back and view their saved doomsday info whenever they want. But this is a stretch goal.
- I wasn't able to get the search function running just yet (if I do I will delete this comment), so I don't have an interactive user query to the NASA API route to contend with yet.
- Lastly, I ran out of time before implementing the user roles and allowing admins to "ban" users accounts.


