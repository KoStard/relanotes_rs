use crate::abstracts::Loadable;
use crate::models::{NodeElement, NodeTypeElement};
use crate::schema::{node_types, nodes};
use diesel::prelude::*;
use diesel::result::Error;
use diesel::SqliteConnection;
use std::collections::HashMap;

pub enum NodeType {
    // Just the type
    Regular,
    StickyNotes,
    Inherited,
    SymLink,
}

#[derive(Serialize)]
#[serde(tag = "node_type")]
pub enum Node<'a> {
    Regular {
        #[serde(skip_serializing)]
        conn: &'a SqliteConnection,
        id: i32,
        name: String,
        description: Option<String>,
        associated_node_id: Option<i32>,
    },
    StickyNotes {
        #[serde(skip_serializing)]
        conn: &'a SqliteConnection,
        id: i32,
        name: String,
        description: Option<String>,
        owner_id: i32,
    },
    Inherited {
        #[serde(skip_serializing)]
        conn: &'a SqliteConnection,
        id: i32,
        name: String,
        description: Option<String>,
        parent_node_id: i32,
    },
    SymLink {
        #[serde(skip_serializing)]
        conn: &'a SqliteConnection,
        id: i32,
        source_node_id: i32,
        source_node_name: String, // Is not loaded from this node's name field
    },
}

impl<'a> Node<'a> {
    fn get_node_id(&self) -> i32 {
        match self {
            Node::Regular {
                conn: _,
                id,
                name: _,
                description: _,
                associated_node_id: _,
            } => *id,
            Node::StickyNotes {
                conn: _,
                id,
                name: _,
                description: _,
                owner_id: _,
            } => *id,
            Node::Inherited {
                conn: _,
                id,
                name: _,
                description: _,
                parent_node_id: _,
            } => *id,
            Node::SymLink {
                conn: _,
                id,
                source_node_id: _,
                source_node_name: _,
            } => *id,
        }
    }
}

pub struct GraphNode<'a> {
    pub node: Node<'a>,
    pub parent_node_id: Option<i32>,
    pub children: Vec<i32>,
}

impl<'a> GraphNode<'a> {
    pub fn new(conn: &'a SqliteConnection, node_element: NodeElement, node_type: NodeType) -> Self {
        let linked_to_id = node_element.linked_to_id;
        let node = match node_type {
            NodeType::Regular => Node::Regular {
                conn,
                id: node_element.id,
                name: node_element.name,
                description: node_element.description,
                associated_node_id: linked_to_id,
            },
            NodeType::StickyNotes => Node::StickyNotes {
                conn,
                id: node_element.id,
                name: node_element.name,
                description: node_element.description,
                owner_id: linked_to_id.unwrap(),
            },
            NodeType::Inherited => Node::Inherited {
                conn,
                id: node_element.id,
                name: node_element.name,
                description: node_element.description,
                parent_node_id: linked_to_id.unwrap(),
            },
            NodeType::SymLink => Node::SymLink {
                conn,
                id: node_element.id,
                source_node_id: linked_to_id.unwrap(),
                source_node_name: nodes::table
                    .filter(nodes::id.eq(linked_to_id.unwrap()))
                    .select(nodes::name)
                    .first::<(String)>(conn)
                    .unwrap(),
            },
        };
        let graph_node = GraphNode {
            node,
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

pub struct NodesTree<'a> {
    pub nodes_map: HashMap<i32, GraphNode<'a>>,
    conn: &'a SqliteConnection,
    subgroup_id: i32,
    node_types_mapping: HashMap<i32, String>,
    pub loaded: bool,
}

impl<'a> NodesTree<'a> {
    pub fn new(conn: &'a SqliteConnection, subgroup_id: i32) -> Self {
        NodesTree {
            nodes_map: HashMap::new(),
            conn,
            subgroup_id,
            node_types_mapping: node_types::table
                .select((node_types::id, node_types::value))
                .load::<(i32, String)>(conn)
                .unwrap()
                .into_iter()
                .collect(),
            loaded: false,
        }
    }

    pub fn create_node(
        &mut self,
        conn: &'a SqliteConnection,
        name: &str,
        description: Option<&str>,
        parent_node_id: Option<i32>,
        subgroup_id: i32,
        type_id: i32,
    ) -> Result<&Node, diesel::result::Error> {
        // Some more checking here!
        if parent_node_id.is_none() {
            if nodes::table
                .filter(nodes::name.eq(name))
                .filter(nodes::linked_to_id.is_null())
                .first::<NodeElement>(conn)
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

        let mut new_node = filter_to_get_model.first::<NodeElement>(conn)?;

        let new_node_id = new_node.id;
        let graph_node = GraphNode::new(conn, new_node, self.get_node_type(&type_id).unwrap());
        self.nodes_map.insert(new_node_id, graph_node);

        if let Some(linked_to_id) = parent_node_id {
            if let Some(parent_graph_node) = self.nodes_map.get_mut(&linked_to_id) {
                parent_graph_node.add_child(new_node_id);
            }
        }
        Ok(&self.nodes_map.get(&new_node_id).unwrap().node)
    }

    fn get_node_type(&self, type_id: &i32) -> Option<NodeType> {
        let type_value = self.node_types_mapping.get(type_id)?;
        match &type_value[..] {
            "regular" => Some(NodeType::Regular),
            "sticky_notes" => Some(NodeType::StickyNotes),
            "inherited" => Some(NodeType::Inherited),
            "symlinks" => Some(NodeType::SymLink),
            _ => None,
        }
    }

    // O(1)
    pub fn node_has_loaded_parent(&self, node_id: i32) -> bool {
        self.nodes_map
            .get(&node_id)
            .and_then(|graph_node| {
                graph_node
                    .parent_node_id
                    .and_then(|parent_node_id| self.nodes_map.get(&parent_node_id))
            })
            .is_some()
    }

    // O(1)
    fn get_graph_node_parent(&self, graph_node: &GraphNode) -> Option<&GraphNode> {
        self.nodes_map.get(&graph_node.parent_node_id?)
    }

    //    pub fn delete_node(&mut self, conn: &SqliteConnection, node_id: i32) -> Option<i32> {
    //        let graph_node = self.nodes_map.get(&node_id)?;
    //        if !graph_node.children.is_empty() {
    //            None
    //        } else {
    //            diesel::delete(nodes::table.filter(nodes::id.eq(node_id)))
    //                .execute(conn)
    //                .ok()?;
    //            // remove from children
    //            if let Some(parent_node_id) = graph_node.parent_node_id {
    //                if let Some(parent) = self.nodes_map.get_mut(&parent_node_id) {
    //                    parent.remove_child(node_id);
    //                }
    //            }
    //            Some(node_id)
    //        }
    //    }

    pub fn get_roots(&self) -> Vec<i32> {
        let mut roots = self
            .nodes_map
            .keys()
            .filter(|id| !self.node_has_loaded_parent(**id))
            .map(|e| *e)
            .collect::<Vec<i32>>();
        roots.sort();
        roots
    }

    pub fn get_node_loaded_children_count(&self, id: &i32) -> Option<usize> {
        Some(self.nodes_map.get(id)?.children.len())
    }

    pub fn get_node_loaded_children(&self, id: &i32) -> Option<Vec<i32>> {
        Some(self.nodes_map.get(id)?.children.clone())
    }

    pub fn get_node_loaded_parent(&self, id: &i32) -> Option<i32> {
        self.nodes_map
            .get(&self.nodes_map.get(id)?.parent_node_id?)
            .map(|n| n.node.get_node_id())
    }
}

impl<'a> Loadable for NodesTree<'a> {
    fn load(&mut self) -> Result<(), Error> {
        let nodes: Vec<NodeElement> = nodes::table
            .filter(nodes::subgroup_id.eq(self.subgroup_id))
            .load::<NodeElement>(self.conn)?;

        let mut nodes_map = HashMap::new();
        let mut children_map: HashMap<i32, Vec<i32>> = HashMap::new();

        for node in nodes {
            let node_id = node.id;
            let linked_to_id = node.linked_to_id;
            let node_type = self.get_node_type(&node.type_id).unwrap();
            let graph_node = GraphNode::new(self.conn, node, node_type);
            nodes_map.insert(node_id, graph_node);
            if let Some(parent) = linked_to_id {
                // nodes_map.get(k: &Q)
                if let Some(parent_children_vec) = children_map.get_mut(&parent) {
                    parent_children_vec.push(node_id);
                } else {
                    children_map.insert(parent, vec![node_id]);
                }
            }
        }

        for (node_id, mut parent_children_vec) in children_map {
            nodes_map.get_mut(&node_id).and_then(|graph_node| {
                graph_node.children.append(&mut parent_children_vec);
                Some(())
            });
        }

        self.nodes_map = nodes_map;
        self.loaded = true;

        Ok(())
    }
}
