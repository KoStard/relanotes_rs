#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate diesel_migrations;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;

pub mod models;
mod nodes_representation;
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

pub fn get_node_type(connection: &SqliteConnection, value: &str) -> NodeType {
    node_types::table
        .filter(node_types::value.eq(value))
        .first::<NodeType>(connection)
        .unwrap()
}

pub fn get_node_types(connection: &SqliteConnection) -> Vec<NodeType> {
    node_types::table.load::<NodeType>(connection).unwrap()
}

pub fn get_nodes(connection: &SqliteConnection) -> Vec<Node> {
    nodes::table.load::<Node>(connection).unwrap()
}

pub fn get_node_by_name(connection: &SqliteConnection, name: &str) -> Option<Node> {
    nodes::table
        .filter(nodes::name.eq(name))
        .first::<Node>(connection)
        .ok()
}

pub fn create_regular_node(
    connection: &SqliteConnection,
    name: &str,
    description: &str,
    parent_node_id: Option<i32>,
    group_id: i32,
) -> Result<usize, diesel::result::Error> {
    diesel::insert_into(nodes::table)
        .values((
            nodes::name.eq(name),
            nodes::description.eq(description),
            nodes::type_id.eq(get_node_type(connection, "regular").id),
            nodes::linked_to_id.eq(parent_node_id),
            nodes::group_id.eq(group_id),
        ))
        .execute(connection)
}

pub fn get_group_from_name(conn: &SqliteConnection, name: &str) -> Option<Group> {
    groups::table.filter(groups::name.eq(name)).first(conn).ok()
}

pub fn load_group(connection: &SqliteConnection, name: &str) -> Option<NodesRepresentation> {
    let mut nodes: Vec<Node> = Node::belonging_to(&get_group_from_name(connection, name)?)
        .load(connection)
        .expect("Got problems while loading nodes");
    Some(NodesRepresentation::new(nodes))
}

embed_migrations!("migrations/");
pub fn setup_database(
    connection: &SqliteConnection,
) -> Result<(), diesel_migrations::RunMigrationsError> {
    embedded_migrations::run(connection)
}
