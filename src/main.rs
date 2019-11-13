use relanotes_rs::{establish_connection, get_node_types};

fn main() {
    let connection = establish_connection();
    println!("{:?}", get_node_types(&connection));
}
