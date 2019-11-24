pub mod subgroups_mod;

use subgroups_mod::SubGroups;

use crate::abstracts::Loadable;
use crate::models::GroupElement;
use crate::schema::groups;
use diesel::prelude::*;
use diesel::SqliteConnection;
use std::collections::HashMap;

pub struct GroupAbstraction<'a> {
    group: GroupElement,
    conn: &'a SqliteConnection,
    subgroups: SubGroups<'a>,
}

impl<'a> GroupAbstraction<'a> {
    fn new(conn: &'a SqliteConnection, group: GroupElement) -> Self {
        let subgroups = SubGroups::new(conn, group.id);
        GroupAbstraction {
            group,
            conn,
            subgroups,
        }
    }
}

pub struct Groups<'a> {
    conn: &'a SqliteConnection,
    groups_map: HashMap<i32, GroupAbstraction<'a>>,
}

impl<'a> Groups<'a> {
    pub fn new(conn: &'a SqliteConnection) -> Self {
        Groups {
            conn,
            groups_map: HashMap::new(),
        }
    }
}

impl Loadable for Groups {
    fn load(&mut self) -> Result<(), diesel::result::Error> {
        let groups: Vec<GroupElement> = groups::table.load::<GroupElement>(self.conn)?;
        self.groups_map = groups
            .into_iter()
            .map(|g| (*&g.id, GroupAbstraction::new(self.conn, g)))
            .collect();
        Ok(())
    }
}
