use super::schema::*;

#[derive(Queryable, Debug)]
pub struct NodeType {
    pub id: i32,
    pub name: String,
    pub value: String
}

// #[derive(Insertable)]
// #[table_name = "node_type"]
// pub struct NewNodeType {
//     pub name: String,
//     pub value: String
// }

