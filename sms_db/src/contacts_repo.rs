use serde::{Deserialize, Serialize};
use surrealdb::{engine::local::RocksDb, sql::Thing, Surreal};

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

#[derive(Debug, Serialize)]
pub struct Contact {
    pub first_name: String,
    pub surname_name: String,
    pub phone: String,
    pub alias: String,
}

impl Contact {
    pub fn new(
        first_name: String,
        surname_name: String,
        phone: String,
        alias: Option<String>,
    ) -> Self {
        let alias = alias.unwrap_or_else(|| format!("{} {}", first_name, surname_name));
        Self {
            first_name,
            surname_name,
            phone,
            alias,
        }
    }
}

pub async fn add_contact(contact: Contact) -> Result<(), String> {
    println!("Adding contact");
    //TODO: storage location should be moved to configuration
    // probably db should be treated as a service that can be reused
    let db = Surreal::new::<RocksDb>("~/rocksdb/sms_modem/test.db")
        .await
        .map_err(|e| format!("Could not connect to db {}", e))?;
    db.use_ns("main")
        .use_db("sms_db")
        .await
        .map_err(|e| format!("Could not use sms db {}", e))?;
    // Create a new person with a random id
    let created: Vec<Record> = db
        .create("contact")
        .content(contact)
        .await
        .map_err(|e| format!("Could not create contact table {}", e))?;
    dbg!(created);
    Ok(())
}
