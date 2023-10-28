use serde::Deserialize;
use surrealdb::sql::Thing;

pub mod contacts_repo;
pub mod repository;
pub mod templates_repo;

#[derive(Debug, Deserialize)]
pub struct AnyRecord {
    #[allow(dead_code)]
    pub id: Thing,
}
