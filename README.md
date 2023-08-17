# Almost-Died-Today

Author: Kira Klingenberg

### A Rust fullstack website built for Casey Baily and Bart Massey's 2023 Rust Web course at Portland State University.

This site uses an axum server, tokio runtime, and sqlx to access a postgres database.

It queries the NASA API NeoWs to grab recent data on asteroids that have passed by earth close enough for us to detect.
Once logged in, the landing page displays a visual representation for the asteroid which came closest to earth on today's date, or barring the existence of one today, the most recent asteroid close encounter.
You will also be greeted with a congradulatory survival message.  

The frontend is built with HTML Tera Templates, CSS, and terrible free stock images sourced directly from the internet.

#### Features still to be implemented:

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

#### Try out these user error situations:

1. Hit the login button with one or more empty login feilds.
2. Attempt to login with a real username (me@gmail.com will work),
but a crap password (try "whoops").
3. Hit the register button with one or more empty feilds.
4. Attempt to register with a username, and only one password feild filled out.

Error messages should appear on the screen accrodingly.

KNOWN BUG: I have been unable to fix a Rust Compiler error that would let me run a check to see if the username provided was indeed in our database.
So presently, if you try to login with a bad username, you just get rerouted to a terrible error page. Please don't try this or you will just be sad.

#### Now try registering a new user, you should be rerouted to a success screen.

Once there, navigate back to the login page and try to login with that just-created username and password.
Or alternatively.....
#### Login using this pre-seeded user:

>Username: me@gmail.com
> 
>Password: banana

You should now be redirected to a page reporting which(if any) asteroid almost killed us all.

#### To try out the search feature, try searching for a few dates within the week. 
If no asteroid is found for a date the site will report that, otherwise the data for the found asteroid will be shown.
Presently we only will have data in the database going back 7-days prior to todays date (per the NASA API limits).

Note: The picture of the angry asteroid is just a placeholder, so it is the same each time, eventually I want to have images of various sizes that are selected based on diameter info.

Clicking on the "go to home page" button will take a logged in user back to the landing page that displays the asteroid of the day.

Note: The client crate in this project is so far unused, but is present in case I want to write any reqwests from there at any point in the future.

## Project Summary

#### There are a few requirements I chose to omit, or just didn't get to in time:

- Since my site is more of an information hub, rather than an interactive place, I didn't come up with a way to has users save and delete some kind of choice.
  If I wanted to add something like this in the future, I suppose I could add a feature to allow users to save the asteroid data displayed on some particular day, so they could go back and view their saved doomsday info whenever they want. But this is a stretch goal.
- I just got the search feature running, and if I have time I will update it to have an interactive user query to the NASA API route if the date is not found in our database.
  (The plan: check our database first, if nothing found, query nasa for a result, if we get one, post it to our database and render it for display)
- Lastly, I ran out of time before implementing the user roles and allowing admins to "ban" users accounts.
- Also a bug to fix: we panic if the date entered by the user is not valid
- I left some clippy warnings about unneeded mapping, because I don't fully understand why I don't need them. I asked about this on zulip, too late probably, and don't have a response yet so am just going to leave those in for now.


To see the full journal of my journey building this site, including what went wrong and how I fixed it, please see JOURNAL.md.

#### Consider this blurb below a TL:DR of JOURNAL.md:

Overall, this project went really well. I was able to successfully build the MVP that I wanted, and I had a lot of fun! 

Here is what it does: 

On first navigation to the home page, you are presented with the option to Login or Register for the first time.
Registration will add your username (email) and password to the User table in the site's database, if it does not already exist.
Something I still need to get to is making sure that successfully registration reroutes you back to the login page.

Once registered with the site, a user can use the login form to sign in.
The login form sends their provided information to the backend and queries the database to verify it.
If verified, a JWT token is created and stored as a browser cookie (with an 8 hour expiration time).
Once logged in, the user is routed to the main page of the site.
The main page displays asteroid recorded for today's date, or the most recent date within a 7 day period, that came the closest to hitting earth.
Some facts about the asteroid, such as how big it was, and how fast it was travelling, are also shown.

Whenever we start things up, run_backend() runs a query to the NASA NeoW's API to grab 7-days worth of data and dump it to our database.
It performs this action once everytime the server is started running.
I would like to update this to periodically and automatically update as the server is running, on some sort of regular schedule.
I would also like to keep a years worth of asteroid data in the database, that accumulates every call to NASA.

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
If the PagePackage contains no asteroid, a message tailored to this situation is displayed in leiu of any Asteroid Data.

Each time the main page is refreshed, the database is queried again for the right asteroid.
Beacuse I don't have the query to NASA set up on a schedule yet, there is no reason for the displayed asteroid to change unless the server is stopped and restarted on a new day.
Likely I will want the scheduled query to NASA to run every hour or so, in case any new asteroid data was added to their API for todays date.

LAST MINUTE ADD ON: I just added in the feature to the login handler, so that if the user hits the login button while leaving the form blank, or if the users password is wrong, the index.html refreshes and displays new error context.
To do this I made a LoginErrors struct to hold the flags for each kind of login error, and the message associated. 
I want to make one for if the user isn't found as well, and will update this struct to have those feilds once I get to it.
I didn't write about this part in the Journal as it was a last minute addition. And also, I got the registration page to reroute as well!
I'm still trying to run a check for if we don't find the username in our db, but currently there is a weird cargo error I cannot debug.
(See commented out lines around line 293 of main_handlers, I left them in in case anyone wants to recreate the bug, just uncomment and run cargo check).
So presently, nothing happens other than an error if we don't have the user requested in the db....

On the CSS: I took Casey's advice and had chatGPT help me out with this. 
I was pleasantly surprised at how much I did actually remember, and was able to use this to remedy the MANY MANY times chatGPT was just plain wrong.
It's definitely not perfect, but I made the executive decision to spend more time on the backend than the front, so the ugly CSS issues will have to be fixed another day.

