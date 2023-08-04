# Journal

### 7/27/23

First things first, lets copy over the docker-compose.yaml for postgres, and change the DB password, username, and db name to "asteroid"
Set up the gitignore.
Create a backend directory.
Make the backend .env (make sure it isn't being tracked in git!).
In the .env, change the names to match the .yaml, aso the URL changes to: ```DATABASE_URL=postgres://asteroid:asteroid@localhost/asteroid```
Create the .env.example file.
Create the cargo.toml file for backend, OH I should have done a cargo new for this, oh well ha.
Brought in everything from the class project .toml for now.
Create the src folder, and it's main.rs, in backend.

Create the client directory with cargo new.
Add the cargo.toml info to client's same file, based on whats in class project up to now.
Set the main.rs in client, and in backend up with the right imports and async main function.

Make the lib.rs file in backend, set up with the same functionality as Casey built.
The use statements can copy right over, but the pub mods will be different as I build my projects files.
I'm  pulling init_logging directly from the same source Casey did.

So now we obviously need to set up our db.rs and routes.rs files, since the lib.rs functions need methods from those files.
Lets start with the routes.
Obviously we really need that db::Store method and at least one route for the route before we can test this.
So lets set up handlers.rs and db.rs basics.

The db.rs file is cool because it abstracts out all things to do with connecting to and querying a database.
We make a struct called Store, that can contain a bunch of different types of DB, in our case we care about a pool connection.
So when we impl on the Struct, we can impl functions that create a new Store struct with some established pool (see new_pool fn), and we can impl all functionality to query the database itself.
This way, all our handlers have to do for CRUD, is accept a Store struct and call it's impl function that is related to the handlers goal.

Now lets add in just the root route in our handlers.
Ok we should have all the basics set up, will commit and test at home on my machine.

### 8/3/2023

Alright time to test this thing and keep building...

postgres connects to it's Docker container and runs great.
Whoops, forgot to write the layers.rs file, I'm pulling this one direct from the class project repo.
Also forgot to pub mod all the new files inside of lib.rs.
Oh year you know what might help, making a .env on this machine haaaaaa: ```cp .env.example .env```
Ok guess I forgot where I was, and I haven't built my DB schema or migrations yet, so time to do that.
But the good news is the server itself appears to be set up correctly!

But real quick first, I'm going to refactor things like Casey showed us in class yesterday.
That's set up, now on to flush out the models directory with my db structs, and make my actual schema in migrations.
I'm going to reference the NASA API's sample query data to determine what information exactly I'd like to store in my sites DB for use.
Yeuch, looking at raw JSON sure is a pain. 
Oh side quest: I need to make the error.rs file now too.
So parsin the API response JSON is fun, I decided to build a NASAAPI.md file just to keep track of the JSON format for a query rsponse to NeoW's
That way I could keep track of how to GET to the data in that response in my own queries.
Now I need to decide what bits I actually want to use on my site and keep in my database. 
I think I'll want all the diameter/size data, to be able to display.
I'll want the asteroid name (which looks like I need to query ANOTHER api to get), itss is_hazardous indication, the close approach datetime, the relative velcocity, miss distance, and orbiting body

Looks like orbiting body can only be earth, mars or venus so I will make a Planet enum to hold these and add to it if I find any other planets in the resposnes.
All numerical data is represented by floats, so I'll type them as f64's
For the date and datetime I'll use chrono's NaiveDate and NaiveDateTime types (thanks chatGPT)

I want the id to be of type ASteroidId, so I'm using the macro Casey showed us in class to handle templating out the struct for an ID type and it's impls.
Ok now to add some migrations. 
First thing I had to stop and think about is: in my rust struct for an asteroid, I'm packaging up all the diamter info into a seperate struct, do I need to reflect this in the DB table columns in some way?
The conclusion I have drawn is, no. I don't. I'll just have a column for each and when I create a new rust struct in a route, handle popualting the struct feild from the table there.
Great, I have a table add and a seed for that table (with only one row but oh well good enough for now) up and running! Verified in teh DB tool in intellij.
Now I am going to try to build my backend again, and test the basics I built there.
Ok forgot to pub mod the error and to change the import for app.
Yes! we are listening!
Now on to writing the integration test to hit the test_db route. But I'll have to stop and do that next as my next class is starting.

Woah, lots of errors. I ended up throwing out the enums as they don't support the derives I want for my asteroid struct.
The orbiting body can just be represented by a String, that's fine.
Ok I think I got it all fixed, just a bunch of little annoying things to do with typing. 
I wanted to move on to using templates for a front end, but I think maybe I've lost my notes or remembered incorrectly what we went over.
I guess I'll just write another route to get an asteroid of a certain date back from the database.
Ok having issues using map on my iterator.....asking on zulip.

Casey pointed out I was missing templates.rs for the Tera templating, I'll go add that now!

Also thanks to casey, I learned that anythign set to NOTNULL in my db including the PKID, will not need to be an option type in its corresponding struct feild.
To that point, I only need to use the .map() function on feilds that are an Option type.
.map(|x| x) sets the closure inside the map function, to say: if there is a value here, keep it, otherwise return None


## Tracking my workflow step by step (some modifications for what I discovered later that should be done earlier)

1.First things first, lets copy over the docker-compose.yaml for postgres, and change the DB password, username, and db name to "asteroid"
2. Set up the gitignore. 
3. Create a backend directory. 
4. Make the backend .env (make sure it isn't being tracked in git!). 
5. In the .env, change the names to match the .yaml, aso the URL changes to: ```DATABASE_URL=postgres://asteroid:asteroid@localhost/asteroid```
6. Create the .env.example file. 
7. Create the cargo.toml file for backend, with cargo new 
8. Bring in everything from the class project .toml 
9. Create the src folder, and it's main.rs, in backend.
10. Create err.rs and move over the Apperror code from class, modify to prepare for our sites specific error types
10. Create the client directory with cargo new (will use this for basic testing until we get test up and running)
11. Add the cargo.toml info to client's same file, based on whats in class project up to now. 
12. Set the main.rs in client, and in backend up with the right imports and async main function.
13. Make the lib.rs file in backend, set up with the same functionality as Casey built. The use statements can copy right over, but the pub mods will be different as I build my projects files.
    I'm  pulling init_logging directly from the same source Casey did. 
14. set up our db.rs, we really need that db::Store method and at least one route for the route before we can move on to routes. build out the new_pool(), Store struct and basic impl functions for Store (with_pool, test_db).
We will add in more impl's for the queries we want, after geting the database schema itself set up.
15. in src, mkdir handlers, mkdir models, mkdir routes
16. Add a mod.rs to each of these directories (where we will pub mod any files that live in each of these folders)
17. Leave models alone for now, focus on handlers and routes
18. In handlers, make main_handlers.rs, add in just the root route in our handlers with some simple info! print for testing, add pub mod main_handlers to mod.rs
19. In routes make main_handlers.rs, buld out the app(), handle_404() and merged_route() functions as seen on CaseyTV, add pub mod main_routes to mod.rs
16. Add the layers.rs file, as seen on CaseyTV
18. Test the server to make sure we set everything up correcly (docker compose up postgres, cd backend cargo run). There will be some panicks as we don't have the DB scheme built at all yet, but there should be no clippy warnings or errors.
19. head back to models, and create a new .rs file for each of the structs we will want to have to represent each table in the DB.
20. Make a NASAAPI file to hold the JSOn format of a resposne coming back from the API, just for reference
21. Add the macro template to lib.rs for making a new ID type (we'll need one for each struct-table type at some point)
22. Inside the backend/tests folder, make a fixtures directory (so we can write some seed migrations)
23. In backend, lets get some database tables created! ```sqlx migrate add -r add_asteroid_table```
24. Now write the SQL to create the table
25. Now lets write a basic seeder for it: ```sqlx migrate add  --sequential --source ./tests/fixtures seed_asteroids```
26. Note: if that command fails, reinstall sqlx-cli: ```cargo install sqlx-cli --git https://github.com/launchbadge/sqlx.git --force```
27. Run the migrations ``` sqlx database reset -y && sqlx migrate run --source ./tests/fixtures --ignore-missing```
28. Note: To view the db tool in intellij for the first time: hit plus button, select postgres, change user, password, and database name to match the URL in the .env
29. Check to see that we can run the backend
30. Build out a test route that just grabs all rows from the db, just to test the DB in an integration test
30. Now build an integration test (in lieu of writing client code)
31.



