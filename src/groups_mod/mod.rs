pub mod subgroups_mod;

use subgroups_mod::{SubGroupAbstraction, SubGroups};

use crate::abstracts::{Loadable, Saveable};
use crate::models::GroupElement;
use crate::schema::groups;
use diesel::prelude::*;
use diesel::Identifiable;
use diesel::SqliteConnection;
use std::collections::HashMap;

pub struct GroupAbstraction<'a> {
    pub group: GroupElement,
    conn: &'a SqliteConnection,
    pub subgroups: SubGroups<'a>,
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

impl<'a> Saveable for GroupAbstraction<'a> {
    fn save(&self) -> Result<(), diesel::result::Error> {
        diesel::update(groups::table.filter(groups::id.eq(self.group.id)))
            .set(groups::name.eq(&self.group.name))
            .execute(self.conn)?;
        Ok(())
    }
}

/// Groups

pub struct Groups<'a> {
    conn: &'a SqliteConnection,
    pub groups_map: HashMap<i32, GroupAbstraction<'a>>,
    pub loaded: bool,
}

impl<'a> Groups<'a> {
    pub fn new(conn: &'a SqliteConnection) -> Self {
        Groups {
            conn,
            groups_map: HashMap::new(),
            loaded: false,
        }
    }
    pub fn get_subgroup(self: &Self, subgroup_id: i32) -> Option<&'a SubGroupAbstraction> {
        self.groups_map.values().by_ref().find_map(|g| {
            g.subgroups
                .subgroups_map
                .values()
                .by_ref()
                .find(|sg| sg.subgroup.id == subgroup_id)
        })
    }
    pub fn get_subgroup_mut(
        self: &mut Self,
        subgroup_id: i32,
    ) -> Option<&'a mut SubGroupAbstraction> {
        let group_id;
        {
            let subgroup = self.get_subgroup(subgroup_id)?;
            group_id = subgroup.subgroup.group_id;
        }
        self.groups_map
            .get_mut(&group_id)
            .and_then(|group| group.subgroups.subgroups_map.get_mut(&subgroup_id))
    }
}

impl<'a> Loadable for Groups<'a> {
    fn load(&mut self) -> Result<(), diesel::result::Error> {
        let groups: Vec<GroupElement> = groups::table.load::<GroupElement>(self.conn)?;
        self.groups_map = groups
            .into_iter()
            .map(|g| (*&g.id, GroupAbstraction::new(self.conn, g)))
            .collect();
        self.loaded = true;
        Ok(())
    }
}

// Adding new groups
impl<'a> Groups<'a> {
    pub fn create(&mut self, name: String) -> Result<&GroupAbstraction<'a>, diesel::result::Error> {
        diesel::insert_into(groups::table)
            .values((groups::name.eq(&name),))
            .execute(self.conn)?;
        let group = groups::table
            .filter(groups::name.eq(&name))
            .first::<GroupElement>(self.conn)?;
        let group_id = group.id;
        let group_abstraction = GroupAbstraction::new(self.conn, group);
        self.groups_map.insert(group_id, group_abstraction);
        Ok(self.groups_map.get(&group_id).unwrap())
    }
}

// Deleting existing groups
impl<'a> Groups<'a> {
    pub fn delete(&mut self, group_id: i32) -> Result<(), diesel::result::Error> {
        diesel::delete(groups::table.filter(groups::id.eq(group_id))).execute(self.conn)?;
        self.groups_map.remove(&group_id); // Even if the group was not registered, not catching the error, because the removal was successful
        Ok(())
    }
}
