use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::sms_repository::{RecordEntity, SmsRepository};

const CONTACT_TABLE: &str = "contact";

#[derive(Debug, Serialize, Deserialize)]
pub struct Contact {
    pub id: Thing,
    pub first_name: String,
    pub surname_name: String,
    pub phone: String,
    pub contact_name: String,
}

impl RecordEntity for Contact {
    fn table_name() -> &'static str {
        CONTACT_TABLE
    }

    fn id(&self) -> &Thing {
        &self.id
    }
}

impl Contact {
    pub fn new(
        first_name: String,
        surname_name: String,
        phone: String,
        contact_name: Option<String>,
    ) -> Self {
        Self::new_with_id(
            Self::random_id(),
            first_name,
            surname_name,
            phone,
            contact_name,
        )
    }

    pub fn new_with_id(
        id: Thing,
        first_name: String,
        surname_name: String,
        phone: String,
        contact_name: Option<String>,
    ) -> Self {
        let contact_name =
            contact_name.unwrap_or_else(|| format!("{} {}", first_name, surname_name));
        Self {
            id,
            first_name,
            surname_name,
            phone,
            contact_name,
        }
    }

    pub fn update(&mut self) {}
}

impl SmsRepository<Contact> {
    pub async fn find_by_contact_name(
        &self,
        contact_name: &str,
    ) -> Result<Option<Contact>, String> {
        self.find_one_by_field("contact_name", contact_name).await
    }
}
