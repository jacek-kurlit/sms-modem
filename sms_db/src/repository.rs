use std::sync::OnceLock;

use sms_config::config::SmsConfig;
use surrealdb::{
    engine::local::{Db, RocksDb},
    Surreal,
};

use crate::{contacts::Contact, groups::Group, sms_repository::SmsRepository, templates::Template};

pub fn contacts() -> SmsRepository<'static, Contact> {
    SmsRepository::new(crate::repository::get())
}

pub fn groups() -> SmsRepository<'static, Group> {
    SmsRepository::new(crate::repository::get())
}

pub fn templates() -> SmsRepository<'static, Template> {
    SmsRepository::new(crate::repository::get())
}

#[derive(Debug)]
pub enum RepositoryError {
    AlreadyInitialized,
    ConnectionError(String),
}

static DB: OnceLock<Surreal<Db>> = OnceLock::new();

pub async fn init(config: &SmsConfig) -> Result<(), RepositoryError> {
    let db: Surreal<Db> = Surreal::init();
    db.connect::<RocksDb>(&config.db.storage_path)
        .await
        .map_err(|e| {
            RepositoryError::ConnectionError(format!("Cold not connect to db. Reason {e}"))
        })?;
    db.use_ns("main")
        .use_db("sms_db")
        .await
        .map_err(|e| RepositoryError::ConnectionError(format!("Could not use sms db {}", e)))?;
    DB.set(db)
        .map_err(|_| RepositoryError::AlreadyInitialized)?;
    Ok(())
}

pub fn get() -> &'static Surreal<Db> {
    DB.get()
        .expect("Db not initialized. Call init before this method!")
}
