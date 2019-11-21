use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;

use super::models::*;
use super::schema::*;

use std::collections::HashMap;

pub struct Groups<'a> {
    connection: &'a SqliteConnection,
    groups_map: HashMap<i32, Group>,
}

impl<'a> Groups<'a> {
    pub fn new(connection: &'a SqliteConnection) -> Self {
        Groups {
            connection,
            groups_map: HashMap::new(),
        }
    }
    pub fn create_group(&mut self, name: &'a str) -> diesel::result::QueryResult<&Group> {
        diesel::insert_into(groups::table)
            .values(groups::name.eq(name))
            .execute(self.connection)?;
        let group = groups::table
            .filter(groups::name.eq(name))
            .first::<Group>(self.connection)?;
        let id = group.id;
        self.groups_map.insert(id, group);
        Ok(self.groups_map.get(&id).unwrap())
    }
    pub fn delete_group(&mut self, id: &i32) -> diesel::result::QueryResult<()> {
        diesel::delete(groups::table.filter(groups::id.eq(id))).execute(self.connection)?;
        if self.groups_map.contains_key(id) {
            self.groups_map.remove(id);
        }
        Ok(())
    }
    pub fn load_groups(&mut self) -> diesel::result::QueryResult<()> {
        for group in groups::table.load::<Group>(self.connection)? {
            self.groups_map.insert(group.id, group);
        }
        Ok(())
    }
    pub fn group_exists(&self, name: &str) -> bool {
        self.groups_map.values().find(|e| e.name == name).is_some()
    }
}

#[cfg(test)]
mod groups_tests {
    use super::*;
    use crate::establish_connection;

    #[test]
    fn test_groups_basic_functionality() {
        let connection = establish_connection();
        let mut grps = Groups::new(&connection);
        assert!(grps.load_groups().is_ok());
        assert!(!grps.group_exists("__test__temp"));
        let grp_res = grps.create_group("__test__temp");
        assert!(grp_res.is_ok());
        let id = grp_res.unwrap().id;
        assert!(grps.group_exists("__test__temp"));
        assert!(grps.delete_group(&id).is_ok());
        assert!(!grps.group_exists("__test__temp"));
    }
}
