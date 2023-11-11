use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::sms_repository::{RecordEntity, SmsRepository};

const TEMPLATE_TABLE: &str = "template";

#[derive(Debug, Serialize, Deserialize)]
pub struct Template {
    pub id: Thing,
    pub name: String,
    pub text: String,
}

impl Template {
    pub fn new(name: String, text: String) -> Self {
        Self {
            id: Self::random_id(),
            name,
            text,
        }
    }
}

impl RecordEntity for Template {
    fn table_name() -> &'static str {
        TEMPLATE_TABLE
    }

    fn id(&self) -> &Thing {
        &self.id
    }
}

impl SmsRepository<Template> {
    pub async fn find_one_by_name(&self, name: &str) -> Result<Option<Template>, String> {
        self.find_one_by_field("name", name).await
    }
}
