// Here order matters

use super::schema::*;

#[derive(Queryable, Identifiable, Debug, Clone, Serialize, Deserialize)]
#[table_name = "node_types"]
pub struct NodeTypeElement {
    pub id: i32,
    pub name: String,
    pub value: String,
}

#[derive(Queryable, Identifiable, Debug, Clone, Serialize, Deserialize)]
#[table_name = "groups"]
pub struct GroupElement {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Identifiable, Debug, Clone, Associations, Serialize, Deserialize)]
#[table_name = "subgroups"]
#[belongs_to(GroupElement, foreign_key = "group_id")]
pub struct SubGroupElement {
    pub id: i32,
    pub group_id: i32,
    pub name: String,
}

#[derive(Queryable, Identifiable, Clone, Associations, Debug, Serialize, Deserialize)]
#[belongs_to(NodeTypeElement, foreign_key = "type_id")]
#[belongs_to(NodeElement, foreign_key = "linked_to_id")]
#[belongs_to(SubGroupElement, foreign_key = "subgroup_id")]
#[table_name = "nodes"]
pub struct NodeElement {
    pub id: i32,
    pub linked_to_id: Option<i32>,
    pub type_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub subgroup_id: i32,
}
