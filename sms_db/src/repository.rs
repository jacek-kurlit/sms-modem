use std::rc::Rc;

use sms_config::config::SmsConfig;
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
    //TODO: this shoul be somewhere in static place to reuse it across whole application without
    //need of passing it
    // we could do the same with db connection
    // btw this loading should not be in this module! It should be held by sms_cli!
    let config = sms_config::load_config().unwrap_or_else(|e| {
        println!("Could not load config, Reason: {:?}", e);
        println!("Using default config");
        SmsConfig::default()
    });
    //"~/test/rocksdb/sms_modem/test.db"
    let db = Surreal::new::<RocksDb>(&config.db.storage_path)
        .await
        .map_err(|e| format!("Could not connect to db {}", e))?;
    db.use_ns("main")
        .use_db("sms_db")
        .await
        .map_err(|e| format!("Could not use sms db {}", e))?;
    Ok(db)
}
