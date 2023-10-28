use serde::{Deserialize, Serialize};
use surrealdb::Surreal;

use crate::{repository, AnyRecord};

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
    db: Surreal<surrealdb::engine::local::Db>,
}

impl GroupRepository {
    pub async fn new() -> Result<Self, String> {
        let db = repository::connect_to_db().await?;
        Ok(Self { db })
    }

    pub async fn create_group(&self, group: Group) -> Result<(), String> {
        let id = group.name.clone();
        let _: Option<AnyRecord> = self
            .db
            .create((GROUP_TABLE, id))
            .content(group)
            .await
            .map_err(|e| format!("Could not create group {}", e))?;
        Ok(())
    }

    pub async fn delete_group(&self, name: &str) -> Result<(), String> {
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

    pub async fn update_group(&self, group: Group) -> Result<(), String> {
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

    pub async fn get_group(&self, name: &str) -> Result<Option<Group>, String> {
        self.db
            .select((GROUP_TABLE, name))
            .await
            .map_err(|e| format!("Could not get template with name: {}, Reason: {}", name, e))
    }

    pub async fn get_all_groups(&self) -> Result<Vec<Group>, String> {
        self.db
            .select(GROUP_TABLE)
            .await
            .map_err(|e| format!("Could not get all groups, Reason: {}", e))
    }
}
