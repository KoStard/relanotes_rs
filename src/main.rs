use relanotes_rs::*;

fn main() {
    let connection = establish_connection();
    // create_regular_node(&connection, "Some regular node", "This is description of some regular node", None).unwrap();
    // println!("{:?}", get_nodes(&connection));
    let node = get_node_by_name(&connection, "Some regular node");
    match node {
        Some(node) => {
            println!("{:?}", node);
        },
        None => {}
    }
}
