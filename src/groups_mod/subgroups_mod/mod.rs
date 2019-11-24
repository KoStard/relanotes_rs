use crate::models::SubGroupElement;
use crate::schema::subgroups;
use diesel::SqliteConnection;
mod nodes_mod;
use crate::abstracts::Loadable;
use diesel::prelude::*;
use diesel::result::Error;
use nodes_mod::NodesTree;
use std::collections::HashMap;

pub struct SubGroupAbstraction<'a> {
    conn: &'a SqliteConnection,
    subgroup: SubGroupElement,
    nodes: NodesTree<'a>,
}

impl<'a> SubGroupAbstraction<'a> {
    pub fn new(conn: &'a SqliteConnection, subgroup: SubGroupElement) -> Self {
        let nodes_tree = NodesTree::new(conn, subgroup.id);
        SubGroupAbstraction {
            conn,
            subgroup,
            nodes: nodes_tree,
        }
    }
}

pub struct SubGroups<'a> {
    conn: &'a SqliteConnection,
    group_id: i32,
    subgroups_map: HashMap<i32, SubGroupAbstraction<'a>>,
}

impl<'a> SubGroups<'a> {
    pub fn new(conn: &'a SqliteConnection, group_id: i32) -> Self {
        SubGroups {
            conn,
            group_id,
            subgroups_map: HashMap::new(),
        }
    }
}

impl<'a> Loadable for SubGroups<'a> {
    fn load(&mut self) -> Result<(), Error> {
        let subgroups: Vec<SubGroupElement> = subgroups::table
            .filter(subgroups::group_id.eq(self.group_id))
            .load::<SubGroupElement>(self.conn)?;
        self.subgroups_map = subgroups
            .into_iter()
            .map(|g| (*&g.id, SubGroupAbstraction::new(self.conn, g)))
            .collect();
        Ok(())
    }
}
