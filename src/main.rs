pub mod database;
pub mod sensors;

#[macro_use] extern crate rocket;

use std::thread;

use rocket::serde::json::Json;
use surrealdb::sql::Value;

use crate::database::{get_database, init_database};

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                            Queries                                             //
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Example query accessible at '.../temperature'
/// This query selects all the data from the payload table and returns it as Json
#[get("/temperature")]
async fn temperature() -> Json<Vec<Value>> {
    let db = get_database(); // The get_database() function can be used to get a reference to the database

    // execute the 'SELECt * FROM payload;' query and store the result in the query variable
    let query = db.query("SELECT * FROM payload;", None).await.unwrap();

    // Returns the result as Json
    Json(query)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                         Main Function                                          //
////////////////////////////////////////////////////////////////////////////////////////////////////

#[launch]
async fn rocket() -> _ {
    // Initialize the database (mandatory!)
    init_database().await;

    // Start the MQTT on a new thread (as mentioned in the function's documentation)
    thread::spawn(|| {
        sensors::start_mqtt();
    });

    // Build the rocket server with the desired routes
    rocket::build()
        .mount("/", routes![temperature])
}