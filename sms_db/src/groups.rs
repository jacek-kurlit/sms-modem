use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::{
    contacts::Contact,
    sms_repository::{RecordEntity, SmsRepository},
};

const GROUP_TABLE: &str = "group";

#[derive(Debug, Serialize, Deserialize)]
pub struct Group {
    pub id: Thing,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupDetails {
    pub id: Thing,
    pub name: String,
    pub contacts: Vec<Contact>,
}

impl Group {
    pub fn id_from_name(name: &str) -> Thing {
        Self::id_from_str(name)
    }

    pub fn new(name: String) -> Self {
        Self {
            id: Self::id_from_name(&name),
            name,
        }
    }
}

impl RecordEntity for Group {
    fn table_name() -> &'static str {
        GROUP_TABLE
    }

    fn id(&self) -> &Thing {
        &self.id
    }
}

impl<'a> SmsRepository<'a, Group> {
    pub async fn find_group_details(
        &self,
        group_id: &Thing,
    ) -> Result<Option<GroupDetails>, String> {
        let mut result = self
            .db
            .query(
                "SELECT *, <-group_assignment<-contact as contacts FROM $group_id FETCH contacts",
            )
            .bind(("group_id", group_id))
            .await
            .map_err(|e| format!("Could not find group details. Reason: {}", e))?;
        result
            .take(0)
            .map_err(|e| format!("Could not find group details. Reason: {}", e))
    }

    pub async fn assign_contact(&self, contact_id: &Thing, group_id: &Thing) -> Result<(), String> {
        self.db
            .query("RELATE $contact_id ->group_assignment-> $group_id")
            .bind(("contact_id", contact_id))
            .bind(("group_id", group_id))
            .await
            .map_err(|e| format!("Could not assign contact to group. Reason: {}", e))?;

        Ok(())
    }

    pub async fn unassign_contact(
        &self,
        contact_id: &Thing,
        group_id: &Thing,
    ) -> Result<(), String> {
        self.db
            .query("DELETE $contact_id->group_assignment WHERE out=$group_id")
            .bind(("contact_id", contact_id))
            .bind(("group_id", group_id))
            .await
            .map_err(|e| format!("Could not unassign contact from group. Reason: {}", e))?;

        Ok(())
    }
}
