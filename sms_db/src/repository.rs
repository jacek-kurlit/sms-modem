use std::rc::Rc;

use surrealdb::{
    engine::local::{Db, RocksDb},
    Surreal,
};

use crate::{contacts::Contact, groups::Group, sms_repository::SmsRepository, templates::Template};

pub struct RepositoriesManager {
    db_ref: Rc<Surreal<surrealdb::engine::local::Db>>,
}

impl RepositoriesManager {
    pub async fn new() -> Result<Self, String> {
        let db_ref = Rc::new(connect_to_db().await?);
        Ok(Self { db_ref })
    }

    pub fn contacts(&self) -> SmsRepository<Contact> {
        SmsRepository::new(self.db_ref.clone())
    }

    pub fn groups(&self) -> SmsRepository<Group> {
        SmsRepository::new(self.db_ref.clone())
    }

    pub fn templates(&self) -> SmsRepository<Template> {
        SmsRepository::new(self.db_ref.clone())
    }
}

async fn connect_to_db() -> Result<Surreal<Db>, String> {
    //TODO: storage location should be moved to configuration
    let db = Surreal::new::<RocksDb>("~/test/rocksdb/sms_modem/test.db")
        .await
        .map_err(|e| format!("Could not connect to db {}", e))?;
    db.use_ns("main")
        .use_db("sms_db")
        .await
        .map_err(|e| format!("Could not use sms db {}", e))?;
    Ok(db)
}
