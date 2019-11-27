#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate serde_derive;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;

//pub mod groups_representation;
//pub mod nodes_representation;
pub mod abstracts;
pub mod database_setup; // Use this to setup the database
pub mod groups_mod;
pub mod models;
pub mod schema;

//use nodes_representation::NodesRepresentation;
//
//use self::models::*;
//use self::schema::*;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set - the path to the db!");
    SqliteConnection::establish(&url).expect("Could not connect to the DB")
}
