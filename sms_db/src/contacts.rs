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

    pub async fn find_all_by_contact_name(
        &self,
        contact_name: &str,
    ) -> Result<Vec<Contact>, String> {
        self.find_by_field("contact_name", contact_name).await
    }

    pub async fn find_exatcly_one_by_contact_name(
        &self,
        contact_name: &str,
        index: Option<usize>,
    ) -> Result<Contact, String> {
        let contacts = self.find_by_field("contact_name", contact_name).await?;
        let number_of_contacts = contacts.len();
        if contacts.is_empty() {
            return Err(format!(
                "Could not find contact with name: '{}'",
                contact_name
            ));
        }
        if number_of_contacts > 1 && index.is_none() {
            return Err(format!(
                "Expected to find exactly one contact with name: '{}', but found {}. Use index to refer correct one",
                contact_name,
            number_of_contacts
            ));
        }
        let index = index.unwrap_or(0);
        contacts.into_iter().nth(index).ok_or_else(|| {
            format!(
                "Could not find contact with name: '{}' at index {} out of {} contacts available",
                contact_name, index, number_of_contacts
            )
        })
    }

    pub async fn find_all_or_select_at_index(
        &self,
        contact_name: &str,
        index: Option<usize>,
    ) -> Result<Vec<Contact>, String> {
        let contacts = self.find_all_by_contact_name(contact_name).await?;
        if contacts.is_empty() {
            return Ok(vec![]);
        }
        if let Some(index) = index {
            Ok(contacts
                .into_iter()
                .nth(index)
                .map(|c| vec![c])
                .unwrap_or_default())
        } else {
            Ok(contacts)
        }
    }
}
