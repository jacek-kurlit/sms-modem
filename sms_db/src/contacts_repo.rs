use surrealdb::{engine::local::SpeeDb, sql::Thing, Surreal};

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

pub async fn add_contact(contact: Contact) -> Result<(), String> {
    println!("Adding contact");
    let db = Surreal::new::<SpeeDb>("/path/to/db/file").await?;
    db.use_ns("main").use_db("sms_db").await?;
    // Create a new person with a random id
    let created: Vec<Record> = db.create("contact").content(contact).await?;
    Ok(())
}
