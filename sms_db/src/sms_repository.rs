use serde::{Deserialize, Serialize};
use surrealdb::{engine::local::Db, sql::Thing, Surreal};

use std::marker::PhantomData;

pub trait RecordEntity: Serialize + for<'de> Deserialize<'de> {
    fn table_name() -> &'static str;

    fn random_id() -> Thing {
        Thing {
            tb: Self::table_name().into(),
            id: surrealdb::sql::Id::rand(),
        }
    }

    fn id_from_str(id: &str) -> Thing {
        Thing {
            tb: Self::table_name().into(),
            id: surrealdb::sql::Id::String(id.into()),
        }
    }

    fn id(&self) -> &Thing;
}

#[derive(Debug, Deserialize)]
struct AnyRecord {
    #[allow(dead_code)]
    id: Thing,
}

pub struct SmsRepository<'a, T> {
    pub db: &'a Surreal<Db>,
    phantom: PhantomData<T>,
}

impl<'a, T> SmsRepository<'a, T>
where
    T: RecordEntity,
{
    pub fn new(db_ref: &'a Surreal<Db>) -> Self {
        Self {
            db: db_ref,
            phantom: PhantomData,
        }
    }

    pub async fn create(&self, record: T) -> Result<T, String> {
        let created: T = self
            .db
            .create(T::table_name())
            .content(record)
            .await
            .map(|x| x.into_iter().next().expect("Failed to create"))
            .map_err(|e| format!("Could not create {}. Reason {}", T::table_name(), e))?;
        Ok(created)
    }

    pub async fn delete(&self, id: &Thing) -> Result<(), String> {
        let _: AnyRecord = self
            .db
            .delete(id)
            .await
            .map_err(|e| format!("Could not delete record of id '{}', Reason: {}", id, e))?
            .ok_or_else(|| {
                format!(
                    "Could not delete record of id '{}', Reason: Record not found",
                    id
                )
            })?;
        Ok(())
    }

    pub async fn find_one_by_field(
        &self,
        field_name: &str,
        field_value: &str,
    ) -> Result<Option<T>, String> {
        let records = self.find_by_field(field_name, field_value).await?;
        if records.len() > 1 {
            return Err(format!(
                "More than one record with field: '{}' of value '{}' found",
                field_name, field_value
            ));
        }
        Ok(records.into_iter().next())
    }

    pub async fn find_by_field(
        &self,
        field_name: &str,
        field_value: &str,
    ) -> Result<Vec<T>, String> {
        let mut result = self
            .db
            .query("SELECT * FROM type::table($table) WHERE type::field($field_name) = $field_value")
            .bind(("table", T::table_name()))
            .bind(("field_name", field_name))
            .bind(("field_value", field_value))
            .await
            .map_err(|e| {
                format!(
                    "Fail to execute query to find records with field: '{}' of value '{}', Reason: {}",
                    field_name, field_value, e
                )
            })?;

        result.take::<Vec<T>>(0).map_err(|e| {
            format!(
                "Could not find records with field: '{}' of value '{}', Reason: {}",
                field_name, field_value, e
            )
        })
    }

    pub async fn get(&self, id: &Thing) -> Result<Option<T>, String> {
        self.db
            .select(id)
            .await
            .map_err(|e| format!("Could not get record with id: '{}', Reason: {}", id, e))
    }

    pub async fn get_all(&self) -> Result<Vec<T>, String> {
        self.db.select(T::table_name()).await.map_err(|e| {
            format!(
                "Could not get all records from table '{}', Reason: {}",
                T::table_name(),
                e
            )
        })
    }

    pub async fn update(&self, record: T) -> Result<(), String> {
        let id = record.id().clone();
        let _: Option<T> = self
            .db
            .update(record.id())
            .content(record)
            .await
            .map_err(|e| format!("Could not update record with id '{}'. Reason {}", id, e))?
            .ok_or_else(|| {
                format!(
                    "Could not update record of id '{}', Reason: Record not found",
                    id
                )
            })?;
        Ok(())
    }
}
