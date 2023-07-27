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
