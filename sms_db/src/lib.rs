use std::rc::Rc;

use contacts_repo::ContactRepository;
use groups_repo::GroupRepository;
use serde::Deserialize;
use surrealdb::{sql::Thing, Surreal};
use templates_repo::TemplateRepository;

pub mod contacts_repo;
pub mod groups_repo;
pub mod repository;
pub mod templates_repo;

#[derive(Debug, Deserialize)]
pub struct AnyRecord {
    #[allow(dead_code)]
    pub id: Thing,
}

pub struct RepositoriesManager {
    db_ref: Rc<Surreal<surrealdb::engine::local::Db>>,
}

impl RepositoriesManager {
    pub async fn new() -> Result<Self, String> {
        let db_ref = Rc::new(repository::connect_to_db().await?);
        Ok(Self { db_ref })
    }

    pub fn contacts(&self) -> ContactRepository {
        ContactRepository::new(self.db_ref.clone())
    }

    pub fn groups(&self) -> GroupRepository {
        GroupRepository::new(self.db_ref.clone())
    }

    pub fn templates(&self) -> TemplateRepository {
        TemplateRepository::new(self.db_ref.clone())
    }
}
