#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::sqlite::{SqliteConnection, Sqlite};
use dotenv::dotenv;

pub mod schema;
pub mod models;

use self::models::*;
use self::schema::*;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set - the path to the db!");
    SqliteConnection::establish(&url).expect("Could not connect to the DB")
}


pub fn get_node_types(connection: &SqliteConnection) -> Vec<NodeType>{
    node_type::table.load::<NodeType>(connection).unwrap()
}
