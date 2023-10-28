use serde::{Deserialize, Serialize};
use surrealdb::Surreal;

use crate::{repository, AnyRecord};

#[derive(Debug, Serialize, Deserialize)]
pub struct Contact {
    pub first_name: String,
    pub surname_name: String,
    pub phone: String,
    pub contact_name: String,
}

impl Contact {
    pub fn new(
        first_name: String,
        surname_name: String,
        phone: String,
        contact_name: Option<String>,
    ) -> Self {
        let contact_name =
            contact_name.unwrap_or_else(|| format!("{} {}", first_name, surname_name));
        Self {
            first_name,
            surname_name,
            phone,
            contact_name,
        }
    }
}

const CONTACT_TABLE: &str = "contact";

pub struct ContactRepository {
    db: Surreal<surrealdb::engine::local::Db>,
}

impl ContactRepository {
    pub async fn new() -> Result<Self, String> {
        let db = repository::connect_to_db().await?;
        Ok(Self { db })
    }
    pub async fn add_contact(&self, contact: Contact) -> Result<(), String> {
        let id = contact.contact_name.clone();
        let _: Option<AnyRecord> = self
            .db
            .create((CONTACT_TABLE, id))
            .content(contact)
            .await
            .map_err(|e| format!("Could not create contact {}", e))?;
        Ok(())
    }

    pub async fn delete_contact(&self, contact_name: &str) -> Result<(), String> {
        let _: AnyRecord = self
            .db
            .delete((CONTACT_TABLE, contact_name))
            .await
            .map_err(|e| {
                format!(
                    "Could not delete contact with name: {}, Reason: {}",
                    contact_name, e
                )
            })?
            .ok_or_else(|| {
                format!(
                    "Could not delete contact with name: {}, Reason: contact not found",
                    contact_name
                )
            })?;
        Ok(())
    }

    pub async fn get_contact(&self, contact_name: &str) -> Result<Option<Contact>, String> {
        self.db
            .select((CONTACT_TABLE, contact_name))
            .await
            .map_err(|e| {
                format!(
                    "Could not get contact with name: {}, Reason: {}",
                    contact_name, e
                )
            })
    }

    pub async fn get_all_contacts(&self) -> Result<Vec<Contact>, String> {
        self.db
            .select(CONTACT_TABLE)
            .await
            .map_err(|e| format!("Could not get all contacts, Reason: {}", e))
    }

    pub async fn update_contact(&self, contact_name: &str, contact: Contact) -> Result<(), String> {
        self.ensure_new_contact_name_does_not_exits(contact_name, &contact)
            .await?;
        self.delete_contact(contact_name).await?;
        self.add_contact(contact).await?;
        Ok(())
    }

    async fn ensure_new_contact_name_does_not_exits(
        &self,
        contact_name: &str,
        updated_contact: &Contact,
    ) -> Result<(), String> {
        if contact_name != updated_contact.contact_name {
            let persisted_contact = self.get_contact(&updated_contact.contact_name).await?;
            if persisted_contact.is_some() {
                return Err(format!(
                    "Could not update contact {} to new values as contact with name: {} already exist",
                    contact_name,
                    &updated_contact.contact_name
                ));
            }
        }
        Ok(())
    }
}
