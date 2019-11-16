use super::schema::*;

#[derive(Queryable, Identifiable, Debug, Clone)]
pub struct NodeType {
    pub id: i32,
    pub name: String,
    pub value: String,
}

#[derive(Queryable, Identifiable, Debug, Clone)]
pub struct Group {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[table_name = "groups"]
pub struct NewGroup {
    pub name: String,
}

#[derive(Queryable, Identifiable, Clone, Associations, Debug)]
#[belongs_to(NodeType, foreign_key = "type_id")]
#[belongs_to(Node, foreign_key = "linked_to_id")]
#[belongs_to(Group, foreign_key = "group_id")]
pub struct Node {
    pub id: i32,
    pub linked_to_id: Option<i32>,
    pub type_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub group_id: i32,
}

#[derive(Insertable, Associations)]
#[belongs_to(NodeType, foreign_key = "type_id")]
#[belongs_to(Node, foreign_key = "linked_to_id")]
#[belongs_to(Group, foreign_key = "group_id")]
#[table_name = "nodes"]
pub struct NewNode {
    pub linked_to_id: Option<i32>,
    pub type_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub group_id: i32,
}
