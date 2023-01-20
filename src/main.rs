pub mod database;
pub mod sensors;
pub mod routes;
pub mod address_api;

#[macro_use] extern crate rocket;

use std::thread;
//use surrealdb::sql::Value;

//use rocket::serde::json::Json;
//use rocket::serde::json::serde_json::json;
//use rocket::serde::{Serialize, Deserialize};

use crate::database::{init_database};

use routes::py_saxion_request;
use routes::py_wierden_request;
use routes::lht_gronau_request;
use routes::lht_wierden_request;
use routes::lht_saxion_request;
use routes::all_request;

//use address_api::get_address;

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
        .mount("/", routes![py_saxion_request, py_wierden_request, lht_gronau_request, lht_wierden_request, lht_saxion_request])
	.mount("/", routes![all_request])


}
