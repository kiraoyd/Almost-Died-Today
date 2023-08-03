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
