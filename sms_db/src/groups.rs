use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::sms_repository::{RecordEntity, SmsRepository};

const GROUP_TABLE: &str = "group";

#[derive(Debug, Serialize, Deserialize)]
pub struct Group {
    pub id: Thing,
    pub name: String,
}

impl Group {
    pub fn new(name: String) -> Self {
        Self {
            id: Self::random_id(),
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

impl SmsRepository<Group> {
    pub async fn find_one_by_name(&self, name: &str) -> Result<Option<Group>, String> {
        self.find_one_by_field("name", name).await
    }
}
