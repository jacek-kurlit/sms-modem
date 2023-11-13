use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::sms_repository::RecordEntity;

const TEMPLATE_TABLE: &str = "template";

#[derive(Debug, Serialize, Deserialize)]
pub struct Template {
    pub id: Thing,
    pub name: String,
    pub text: String,
}

impl Template {
    pub fn id_from_name(name: &str) -> Thing {
        Self::id_from_str(name)
    }

    pub fn new(name: String, text: String) -> Self {
        Self {
            id: Self::id_from_name(&name),
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
