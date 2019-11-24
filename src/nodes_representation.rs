// use slotmap::{DefaultKey, SlotMap};
use super::models::Node;
use super::schema::nodes;
use diesel;
use diesel::debug_query;
use diesel::prelude::*;
use diesel::sqlite::{Sqlite, SqliteConnection};
use std::collections::HashMap;

struct GraphNode {
    node: Node,
    parent_node_id: Option<i32>,
    pub children: Vec<i32>,
}

impl GraphNode {
    pub fn new(node: Node) -> Self {
        let linked_to_id = node.linked_to_id;
        let graph_node = GraphNode {
            node: node,
            parent_node_id: linked_to_id,
            children: vec![],
        };
        graph_node
    }

    pub fn add_child(&mut self, node_id: i32) {
        self.children.push(node_id);
    }

    pub fn remove_child(&mut self, node_id: i32) -> Option<i32> {
        Some(
            self.children
                .remove(self.children.iter().position(|&e| e == node_id)?),
        )
    }
}

// #[derive(Clone, Copy)]
// struct GraphNodeRawLink {
//     graph_node: *mut GraphNode,
// }

pub struct NodesRepresentation {
    // slots: SlotMap<DefaultKey, GraphNodeRawLink>,
    map: HashMap<i32, GraphNode>,
}

impl NodesRepresentation {
    pub fn new(mut nodes: Vec<Node>) -> Self {
        // let slots = SlotMap::new();
        let mut map = HashMap::new();
        let mut children_map: HashMap<i32, Vec<i32>> = HashMap::new();

        for node in nodes {
            let node_id = node.id;
            let linked_to_id = node.linked_to_id;
            let graph_node = GraphNode::new(node);
            map.insert(node_id, graph_node);
            if let Some(parent) = linked_to_id {
                // map.get(k: &Q)
                if let Some(parent_children_vec) = children_map.get_mut(&parent) {
                    parent_children_vec.push(node_id);
                } else {
                    children_map.insert(parent, vec![node_id]);
                }
            }
        }

        for (node_id, mut parent_children_vec) in children_map {
            map.get_mut(&node_id).and_then(|graph_node| {
                graph_node.children.append(&mut parent_children_vec);
                Some(())
            });
        }
        NodesRepresentation { map: map }
    }

    pub fn create_node(
        &mut self,
        conn: &SqliteConnection,
        name: &str,
        description: Option<&str>,
        parent_node_id: Option<i32>,
        subgroup_id: i32,
        type_id: i32,
    ) -> Result<i32, diesel::result::Error> {
        if parent_node_id.is_none() {
            if nodes::table
                .filter(nodes::name.eq(name))
                .filter(nodes::linked_to_id.is_null())
                .first::<Node>(conn)
                .is_ok()
            {
                return Err(diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UniqueViolation,
                    Box::new(String::from("Duplicate name")),
                ));
            }
        }
        diesel::insert_into(nodes::table)
            .values((
                nodes::name.eq(name),
                nodes::description.eq(description),
                nodes::type_id.eq(type_id),
                nodes::linked_to_id.eq(parent_node_id),
                nodes::subgroup_id.eq(subgroup_id),
            ))
            .execute(conn)?;

        let mut filter_to_get_model = nodes::table
            .filter(nodes::name.eq(name))
            .filter(nodes::subgroup_id.eq(subgroup_id))
            .into_boxed();

        if parent_node_id.is_some() {
            filter_to_get_model =
                filter_to_get_model.filter(nodes::linked_to_id.eq(parent_node_id));
        } else {
            filter_to_get_model = filter_to_get_model.filter(nodes::linked_to_id.is_null());
        }

        let mut new_node = filter_to_get_model.first::<Node>(conn)?;

        let new_node_id = new_node.id;
        let graph_node = GraphNode::new(new_node);
        self.map.insert(new_node_id, graph_node);

        if let Some(linked_to_id) = parent_node_id {
            if let Some(parent_graph_node) = self.map.get_mut(&linked_to_id) {
                parent_graph_node.add_child(new_node_id);
            }
        }
        Ok(new_node_id)
    }

    // O(1)
    pub fn node_has_loaded_parent(&self, node_id: i32) -> bool {
        self.map
            .get(&node_id)
            .and_then(|graph_node| {
                graph_node
                    .parent_node_id
                    .and_then(|parent_node_id| self.map.get(&parent_node_id))
            })
            .is_some()
    }

    // O(1)
    fn get_graph_node_parent(&self, graph_node: &GraphNode) -> Option<&GraphNode> {
        self.map.get(&graph_node.parent_node_id?)
    }

    pub fn delete_node(&mut self, conn: &SqliteConnection, node_id: i32) -> Option<i32> {
        let graph_node = self.map.get(&node_id)?;
        if !graph_node.children.is_empty() {
            None
        } else {
            diesel::delete(nodes::table.filter(nodes::id.eq(node_id)))
                .execute(conn)
                .ok()?;
            // remove from children
            if let Some(parent_node_id) = graph_node.parent_node_id {
                if let Some(parent) = self.map.get_mut(&parent_node_id) {
                    parent.remove_child(node_id);
                }
            }
            Some(node_id)
        }
    }

    pub fn set_node_name(
        &mut self,
        conn: &SqliteConnection,
        node_id: i32,
        new_name: &str,
    ) -> diesel::result::QueryResult<()> {
        let graph_node = self
            .map
            .get_mut(&node_id)
            .ok_or(diesel::result::Error::NotFound)?;
        diesel::update(nodes::table.filter(nodes::id.eq(node_id)))
            .set(nodes::name.eq(new_name))
            .execute(conn)?;
        graph_node.node.name = new_name.into();
        Ok(())
    }

    pub fn set_node_description(
        &mut self,
        conn: &SqliteConnection,
        node_id: &i32,
        new_description: Option<&str>,
    ) -> diesel::result::QueryResult<()> {
        let graph_node = self
            .map
            .get_mut(node_id)
            .ok_or(diesel::result::Error::NotFound)?;
        diesel::update(nodes::table.filter(nodes::id.eq(node_id)))
            .set(nodes::description.eq(new_description))
            .execute(conn)?;
        graph_node.node.description = new_description.map(|d| d.into());
        Ok(())
    }

    pub fn get_node(&self, id: &i32) -> Option<&Node> {
        Some(&self.map.get(id)?.node)
    }

    fn get_graph_node(&self, id: &i32) -> Option<&GraphNode> {
        self.map.get(id)
    }

    pub fn get_roots(&self) -> Vec<i32> {
        let mut roots = self
            .map
            .keys()
            .filter(|id| !self.node_has_loaded_parent(**id))
            .map(|e| *e)
            .collect::<Vec<i32>>();
        roots.sort();
        roots
    }

    pub fn get_node_loaded_path_names(&self, id: &i32) -> Option<Vec<String>> {
        let mut reversed_path = Vec::new();
        let mut graph_node = self.get_graph_node(id)?;
        while let Some(parent) = self.get_graph_node_parent(graph_node) {
            reversed_path.push(String::from(&parent.node.name));
            graph_node = parent;
        }
        reversed_path.reverse();
        Some(reversed_path)
    }

    pub fn get_node_loaded_children_count(&self, id: &i32) -> Option<usize> {
        Some(self.map.get(id)?.children.len())
    }

    pub fn get_node_loaded_children(&self, id: &i32) -> Option<Vec<i32>> {
        Some(self.map.get(id)?.children.clone())
    }

    pub fn get_node_loaded_parent(&self, id: &i32) -> Option<i32> {
        self.map
            .get(&self.map.get(id)?.parent_node_id?)
            .map(|n| n.node.id)
    }
}
