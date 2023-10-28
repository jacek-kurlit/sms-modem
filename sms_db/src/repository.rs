use surrealdb::{
    engine::local::{Db, RocksDb},
    Surreal,
};

pub async fn connect_to_db() -> Result<Surreal<Db>, String> {
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
