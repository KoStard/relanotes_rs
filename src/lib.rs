#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate diesel_migrations;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;

pub mod models;
pub mod nodes_representation;
pub mod schema;

use nodes_representation::NodesRepresentation;

use self::models::*;
use self::schema::*;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set - the path to the db!");
    SqliteConnection::establish(&url).expect("Could not connect to the DB")
}

pub fn get_node_type(connection: &SqliteConnection, value: &str) -> Option<NodeType> {
    node_types::table
        .filter(node_types::value.eq(value))
        .first::<NodeType>(connection)
        .ok()
}

pub fn get_group_by_name(conn: &SqliteConnection, name: &str) -> Option<Group> {
    groups::table.filter(groups::name.eq(name)).first(conn).ok()
}

pub fn load_group(connection: &SqliteConnection, group: &Group) -> NodesRepresentation {
    let mut nodes: Vec<Node> = Node::belonging_to(group)
        .load(connection)
        .expect("Got problems while loading nodes");
    NodesRepresentation::new(nodes)
}

pub fn create_group(conn: &SqliteConnection, name: &str) -> diesel::result::QueryResult<Group> {
    diesel::insert_into(groups::table)
        .values(groups::name.eq(name))
        .execute(conn)?;
    groups::table.filter(groups::name.eq(name)).first(conn)
}

pub fn list_groups(conn: &SqliteConnection) -> diesel::result::QueryResult<Vec<Group>> {
    groups::table.load::<Group>(conn)
}

//pub fn delete_group(conn: &SqliteConnection, id: i32) -> Option<i32> {
//    diesel::delete(
//        groups::table.filter(groups::id.eq(id))
//    ).execute(conn)
//}

embed_migrations!("migrations/");
pub fn setup_database(
    connection: &SqliteConnection,
) -> Result<(), diesel_migrations::RunMigrationsError> {
    embedded_migrations::run(connection)
}
