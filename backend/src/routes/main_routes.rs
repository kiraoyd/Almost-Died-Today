use axum::response::Response;
use axum::Router;
use axum::routing::*;
use http::StatusCode;
use hyper::Body;
use sqlx::PgPool;
use tracing::info;

use crate::db::Store;
use crate::layers;

//bring in the various handler files here
use crate::handlers::main_handlers;
use crate::handlers::main_handlers::root;

//bring in the various route files here
//use crate::routes::comment_routes::comment_routes;

//takes in a pool, sets up the db seeds, layers on middlewares, and returns a new router
pub async fn app(pool:PgPool) -> Router {
    let db = Store::with_pool(pool);

    info!("Seeded database");

    //Middlewares
    let(cors_layer, trace_layer) = layers::get_layers();

    Router::new()
        .route("/", get(root))
        .route("/asteroid", get(main_handlers::test_db))
        //add new routes here, reads top to bottom

        //------

        //this 404 route always caps off the routes
        .route("/*_", get(handle_404)) //if no other route is found, we have a page note found 404 error
        //.merge(route_file()) //uncomment this once we have more route files to merge
        .layer(cors_layer)
        .layer(trace_layer)
        .with_state(db.clone())
}

async fn handle_404() -> Response<Body>{
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("The requested page could not be found"))
        .unwrap()
}

/*
 Generic Router creation for merging.  Here, T is the type of the State, which is
 at our discretion.  In this case, it is the Store struct, which contains the database.
   The method_router is the router that we want to merge into the main router.

   The trait bounds here can be a bit confusing.  Send + Sync exist because we're in Tokioland
   where Axum may be running this on multiple threads.  The static is a bit different.

   In Rust, this essentially means that whatever T is, has to live for the entire lifetime of the program.
   This is necessary if our T has references to other data inside of it.  Rust needs to be able
   to guarantee that the data will be there for as long as this Router holds onto it.

   So this 'static lifetime is telling Rust that T can either contain no references at all,
   or they must be references that live for the life of the program (ie static themselves)

   We need it here not only because Axum's handlers require it, but also because we can tell that
   Router will exist for the life of the program an also capture the state of T.  So if T is not static,
   then we have a problem.

   Our Store, luckily, passes this test fine.  If you take a look, you'll now be able to see
   why those Arc<Mutex> were necessary!

*/
pub fn merged_route<T>(path: &str, method_router: MethodRouter<T>) -> Router<T>
    where
        T: Clone + Send + Sync + 'static,
{
    Router::new().route(path, method_router)
}