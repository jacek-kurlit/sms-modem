use std::rc::Rc;

use serde::{Deserialize, Serialize};
use surrealdb::{engine::local::Db, Surreal};

use crate::AnyRecord;

#[derive(Debug, Serialize, Deserialize)]
pub struct Group {
    pub name: String,
    pub assigned_contacts: Vec<String>,
}

impl Group {
    pub fn new(name: String) -> Self {
        Self {
            name,
            assigned_contacts: Vec::new(),
        }
    }
}

const GROUP_TABLE: &str = "group";

pub struct GroupRepository {
    db: Rc<Surreal<Db>>,
}

impl GroupRepository {
    pub fn new(db: Rc<Surreal<Db>>) -> Self {
        Self { db }
    }

    pub async fn create(&self, group: Group) -> Result<(), String> {
        let id = group.name.clone();
        let _: Option<AnyRecord> = self
            .db
            .create((GROUP_TABLE, id))
            .content(group)
            .await
            .map_err(|e| format!("Could not create group {}", e))?;
        Ok(())
    }

    pub async fn delete(&self, name: &str) -> Result<(), String> {
        let _: AnyRecord = self
            .db
            .delete((GROUP_TABLE, name))
            .await
            .map_err(|e| format!("Could not delete group with name: {}, Reason: {}", name, e))?
            .ok_or_else(|| {
                format!(
                    "Could not delete group with name: {}, Reason: group not found",
                    name
                )
            })?;
        Ok(())
    }

    pub async fn update(&self, group: Group) -> Result<(), String> {
        let group_name = group.name.clone();
        let _: Option<Group> = self
            .db
            .update((GROUP_TABLE, &group_name))
            .content(group)
            .await
            .map_err(|e| {
                format!(
                    "Could not update group with name: {}, Reason: {}",
                    group_name, e
                )
            })?;
        Ok(())
    }

    pub async fn get(&self, name: &str) -> Result<Option<Group>, String> {
        self.db
            .select((GROUP_TABLE, name))
            .await
            .map_err(|e| format!("Could not get template with name: {}, Reason: {}", name, e))
    }

    pub async fn get_all(&self) -> Result<Vec<Group>, String> {
        self.db
            .select(GROUP_TABLE)
            .await
            .map_err(|e| format!("Could not get all groups, Reason: {}", e))
    }
}
