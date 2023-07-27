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

So now we obviously need to set up our db.rs and rouotes.rs files, since the lib.rs functions need methods from those files.
Lets start with the routes.
Obviously we really need that db::Store method and at least one route for the route before we can test this.
So lets set up handlers.rs and db.rs basics.

The db.rs file is cool because it abstracts out all things to do with connecting to and querying a database.
We make a struct called Store, that can contain a bunch of different types of DB, in our case we care about a pool connection.
So when we impl on the Struct, we can impl functions that create a new Store struct with some established pool (see new_pool fn), and we can impl all functionality to query the database itself.
This way, all our handlers have to do for CRUD, is accept a Store struct and call it's impl function that is related to the handlers goal.

Now lets add in just the root route in our handlers.
Ok we should have all the basics set up, will commit and test at home on my machine.
