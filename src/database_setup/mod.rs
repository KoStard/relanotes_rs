use super::schema::*;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

embed_migrations!("migrations/");
fn setup_migrations(conn: &SqliteConnection) -> Result<(), diesel_migrations::RunMigrationsError> {
    embedded_migrations::run(conn)
}

fn add_node_types(conn: &SqliteConnection) -> Result<usize, diesel::result::Error> {
    diesel::insert_or_ignore_into(node_types::table)
        .values(&vec![
            (
                node_types::name.eq("Regular"),
                node_types::value.eq("regular"),
            ),
            (
                node_types::name.eq("Sticky Notes"),
                node_types::value.eq("sticky_notes"),
            ),
            (
                node_types::name.eq("Inherited"),
                node_types::value.eq("inherited"),
            ),
            (
                node_types::name.eq("Symlinks"),
                node_types::value.eq("symlinks"),
            ),
        ])
        .execute(conn)
}

fn setup_initial_data(conn: &SqliteConnection) -> Result<(), diesel::result::Error> {
    // But remember that this function will be called with each start-up, so we have to check if the data is loaded or not
    add_node_types(conn)?;
    Ok(())
}

pub fn setup_database(conn: &SqliteConnection) -> Option<()> {
    setup_migrations(conn).ok()?;
    setup_initial_data(conn).ok()?;
    Some(())
}
