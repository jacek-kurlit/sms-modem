use std::rc::Rc;

use serde::{Deserialize, Serialize};
use surrealdb::{
    engine::local::Db,
    sql::{Id, Thing},
    Surreal,
};

use crate::AnyRecord;
use std::marker::PhantomData;

pub struct SmsRepository<T> {
    db: Rc<Surreal<Db>>,
    table_name: String,
    phantom: PhantomData<T>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TestData {
    pub id: Option<Thing>,
    pub field: String,
    pub name: String,
}

impl TestData {
    pub fn new(field: String, name: String) -> Self {
        Self {
            id: None,
            field,
            name,
        }
    }
}

impl<T> SmsRepository<T>
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    pub fn new(db_ref: Rc<Surreal<Db>>, table_name: &str) -> Self {
        Self {
            db: db_ref,
            table_name: table_name.to_string(),
            phantom: PhantomData,
        }
    }
    pub async fn create(&self, record: T) -> Result<T, String> {
        let created: T = self
            .db
            //TODO: fnd way o add id or ho do you parse it with record?
            .create(&self.table_name)
            .content(record)
            .await
            .map(|x| x.into_iter().next().expect("Failed to create"))
            .map_err(|e| format!("Could not create {}. Reason {}", self.table_name, e))?;
        Ok(created)
    }

    //TODO: define id!
    pub async fn delete(&self, id: &str) -> Result<(), String> {
        let _: AnyRecord = self
            .db
            .delete((&self.table_name, id))
            .await
            .map_err(|e| {
                format!(
                    "Could not delete record of id '{}' from table: '{}', Reason: {}",
                    id, self.table_name, e
                )
            })?
            .ok_or_else(|| {
                format!(
                    "Could not delete record of id '{}' from table: '{}', Reason: Record not found",
                    id, self.table_name
                )
            })?;
        Ok(())
    }
    pub async fn getv2(&self, id: &Thing) -> Result<Option<T>, String> {
        self.db.select(id).await.map_err(|e| {
            format!(
                "Could not get record with id: '{}' from table '{}', Reason: {}",
                id, self.table_name, e
            )
        })
    }
    //TODO: define id!
    pub async fn get(&self, id: &str) -> Result<Option<T>, String> {
        self.db
            .select((&self.table_name, id.clone()))
            .await
            .map_err(|e| {
                format!(
                    "Could not get record with id: '{}' from table '{}', Reason: {}",
                    id, self.table_name, e
                )
            })
    }

    pub async fn get_all(&self) -> Result<Vec<T>, String> {
        self.db.select(&self.table_name).await.map_err(|e| {
            format!(
                "Could not get all records from table '{}', Reason: {}",
                self.table_name, e
            )
        })
    }

    //TODO: define id!
    pub async fn update(&self, id: &str, record: T) -> Result<(), String> {
        let _: Vec<Option<AnyRecord>> = self
            .db
            .update((&self.table_name, id))
            .content(record)
            .await
            .map_err(|e| {
                format!(
                    "Could not update record with id '{}' in table '{}'. Reason {}",
                    id, self.table_name, e
                )
            })?
            .ok_or_else(|| {
                format!(
                    "Could not update record of id '{}' in table: '{}', Reason: Record not found",
                    id, self.table_name
                )
            })?;
        Ok(())
    }
}
