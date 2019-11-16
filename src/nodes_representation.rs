// use slotmap::{DefaultKey, SlotMap};
use super::models::Node;
use super::schema::nodes;
use diesel;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
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
        description: &str,
        parent_node_id: Option<i32>,
        group_id: i32,
        type_id: i32,
    ) -> Result<i32, diesel::result::Error> {
        diesel::insert_into(nodes::table)
            .values((
                nodes::name.eq(name),
                nodes::description.eq(description),
                nodes::type_id.eq(type_id),
                nodes::linked_to_id.eq(parent_node_id),
                nodes::group_id.eq(group_id),
            ))
            .execute(conn)?;
        let mut new_node = nodes::table
            .filter(nodes::name.eq(name))
            .filter(nodes::linked_to_id.eq(parent_node_id))
            .filter(nodes::group_id.eq(group_id))
            .first::<Node>(conn)?;
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
    pub fn node_has_loaded_parent(&self, node_id: i32) -> Option<bool> {
        self.map
            .get(&node_id)?
            .parent_node_id
            .and_then(|parent_node_id| self.map.get(&parent_node_id).map(|_| true))
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
        let graph_node = self.map
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
        node_id: i32,
        new_description: Option<&str>,
    ) -> diesel::result::QueryResult<()> {
        let graph_node = self.map
            .get_mut(&node_id)
            .ok_or(diesel::result::Error::NotFound)?;
        diesel::update(nodes::table.filter(nodes::id.eq(node_id)))
            .set(nodes::description.eq(new_description))
            .execute(conn)?;
        graph_node.node.description = new_description.map(|d| d.into());
        Ok(())
    }
}
