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



### 8/4/23

Ok I'm writing a new route to query for all asteroids that are flagged as hazardous, for a specific date. This is assuming that I've pulled in NASA's data from their API already, and it's living in my DB.
This brings up a good question, how much of NASA's data do I want in my own DB at any given time?

So the logic here will be: query for the asteroids of that date and specification, then iterate over the rows and pick out the one with the smallest (closest) near miss date.
So I'll iterate over them, tracking the smallest seen until I find it. Then I will store that in an Asteroid struct and return it.

I was hoping iter had some kind of "find the max" function, and this reddit post might have the answer: https://www.reddit.com/r/rust/comments/31syce/using_iterators_to_find_the_index_of_the_min_or/
Looks liek there is an enumerate().min_by() function available. 
I found it in the docs and will try min_by_key(), sending it the miss_distance_miles value for each row in rows.
https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.max_by_key
But iterating over the rows from the query seems complicated due to the type, so I'm going to try to map to Rust structs, then iterate over THOSE.

Oh, to pass in the &str type to the SQL I need it to be in NaiveDate form, look what I found! https://docs.rs/chrono/latest/chrono/naive/struct.NaiveDate.html#method.parse_from_str
That fixed it, just needed to parseit from &str to NaiveDate.

But now....I can't use min_by_key to find the max value in my vec of structs, because Ord isn't implemented for an f64. BWAAAAA.
Ok so I am going to try another approach: sort the Vec from smallest to largest miss_distance_miles values: https://www.reddit.com/r/rust/comments/yl3tov/total_cmp_on_f32_and_f64_and_ord/
Using total_cmp as seen in the docs example here:https://doc.rust-lang.org/std/primitive.f64.html#method.total_cmp
The only issue is, how do I handle if there are any null values here? Ok I'll only collect up asteroids that have values in that feild, how bout that.
Grrrrg it_empty() is not impld for f64. Nothing seems to be available for f64 whyyyy. Lets go to the docs again.

Ok I changed direction, I'll just iterate over the sorted asteroids, and use a match statement to pull the first one that has Some() value, then stop.
Now I'm setting up the handler and route for this endpoint, and here is my first lifetime issue! 
I need an &str to pass through the functions, and I think this is problematic somehow.
I tried to clone the query param for the date, rather than try to keep passing the reference, but clone wasn't working.
I ended up finding to_owned, and learning that a clone converts an &T to a T, wheras to_owned can convert &T to another target type, and allow the ownership to be passed.
https://www.reddit.com/r/rust/comments/l5uih4/what_is_the_difference_between_clone_and_to_owned/
Ok but this appears to convert the &str to a....String type, so I think I need to change what get_closest_by_date expects as an arg.
Ah and I need to adjust how I pass the date into the parse_from_str function, as it needs a str not a String.
Ok that just pushed the lifetime error up one level, to the router itself. 
So maybe passing an &str at all is just a bad idea here. Lets try get_closest accepting the query as a String from the get go, then we only turn it into a str when we need to in the final query code (when we convert it to a NaiveDate).
Yes that worked! So far no errors. Tho I'm not sure if having the query param be of type String will cause more problems when I try to hit the endpoint.
Ok I tested from postman, it works! One issue, what if we don't find a matching asteroid? I need to handle the return differently.


### 8/6/2023

Time to try and connect to the NASA API NeoW's. I generated an API key, and stored it in my untracked .env file. 
I'll accesss it via dotenv in the lib.rs function I'm writing to bring in NASA data and convert it to a Vec<Asteroid>.
I'm going to write a function that can take any NaiveDate as the requested date to pull info from, then will grab a years worth of data from NeoWs leading up to that date.
That way if there was no close call on the exact requested date, we can report the most recent asteroid around that date.
It would be cool to be able to grab pre and post asteroids for dates requested in the past, but I'll save that for an extra feature later.
I want to be able to construct the request url with specific values, so I'm using rusts format! macro to insert values.

Ah interesting, the feed only allows for up to 7 days of data, thats fine.
Ha we got some data back! I had to mess around with the error typing to make one for Reqwest specifically, but I think this is gonna work!
I just have to think about WHERE I want to call this function.

I also need to now deserialize the API JSON response, into my own Asteroid structs.
I'll be using serde for this, but I'm having trouble figuring out how to iterate over the JSON string....
Bart helped out on Zulip and pointed me in the direction of a HashMap.
I think I got it set up, but the issue now is getting serde to deserialize from the JSON response into my Rust Struct Types.
I'm hitting a wall so I'll have to return to this later.

Ahahahahahahaha (maniacal laughter continues)....so After much pain I have finally figured out what's wrong: My Rust struct and the incoming NASA JSON were not matching, and they have to EXACTLY match in every way or serde gets mad.
I could have left out some feilds with data I didn't want, serde can handle that, but after a day lost to this, I decided to bite the bullet and feed ChatGPT the NASA JSON and ask it really nicely to show me the Rust struct equivalents.
I am not sorry I did this, the struct heirarchy for this API was kind of crazy, and what chatGPT gave me was way nicer than what I would have produced.
I adjusted some things like making feilds public, and adjusting the Derives a bit, as well as making All the necesary feilds Options and handling the one case where I needed a String to deserialize into a number.
So NOW all I need to do is go back through the code I'd previously written (commented out at this commit to test the NASA route easily), and redo with the new struct structures.
Actually I think I can keep the old Asteroid struct (renaming it tho) to represent the data I want stored from that API response, in my database.
That way I don't need to change the SQL or actually, much of the routes either, I'll just need to be careful how I post from the new ASteroid struct to the darabase representation one.
Ok that wasn't so bad, It's running! 

Still to do: Post the nasa data to the database
Add authorization (just do it like we did in class, with user table storing id and password)
Add a frontend to display data (build out index.html)
Fix the get closest by date route to handle when there is noting found for the date requested

### 8/10/2023

Ok, time to get Auth inserted to this thing.
First I'm going to finish this Post route, to make sure I can add the relevant info I need to my DB.
Now it's time to add in the user.rs file and build out the needed functionality for a User and the KEYS we use to encode and decode a JWT token.
Next I added the Register, Login, and Protected handlers to handers.rs and routes.rs.

By adding all these in I realised I am missing a bunch of AppeError types and the get_user and create_user handlers.


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
31. Build out new routes: GET Asteroid that came closest on a specified date
32. Build out new method in lib.rs: Grab all data from the NASA API, convert to Vec<Asteroid> and return it, in preparation for POSTing to our db
33. Go to api.nasa.gov, generate an API Key. Account info and API key is in the .env
34. To query the NeoW's API:  GET https://api.nasa.gov/neo/rest/v1/feed?start_date=START_DATE&end_date=END_DATE&api_key=API_KEY


