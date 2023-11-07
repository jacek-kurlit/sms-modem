use std::rc::Rc;

use serde::{Deserialize, Serialize};
use surrealdb::{engine::local::Db, Surreal};

use crate::AnyRecord;

#[derive(Debug, Serialize, Deserialize)]
pub struct Template {
    pub name: String,
    pub text: String,
}

impl Template {
    pub fn new(name: String, text: String) -> Self {
        Self { name, text }
    }
}

const TEMPLATE_TABLE: &str = "template";

pub struct TemplateRepository {
    db: Rc<Surreal<Db>>,
}

impl TemplateRepository {
    pub fn new(db: Rc<Surreal<Db>>) -> Self {
        Self { db }
    }

    pub async fn create(&self, template: Template) -> Result<(), String> {
        let id = template.name.clone();
        let _: Option<AnyRecord> = self
            .db
            .create((TEMPLATE_TABLE, id))
            .content(template)
            .await
            .map_err(|e| format!("Could not create template {}", e))?;
        Ok(())
    }

    pub async fn delete(&self, name: &str) -> Result<(), String> {
        let _: AnyRecord = self
            .db
            .delete((TEMPLATE_TABLE, name))
            .await
            .map_err(|e| {
                format!(
                    "Could not delete template with name: {}, Reason: {}",
                    name, e
                )
            })?
            .ok_or_else(|| {
                format!(
                    "Could not delete template with name: {}, Reason: template not found",
                    name
                )
            })?;
        Ok(())
    }

    pub async fn get(&self, name: &str) -> Result<Option<Template>, String> {
        self.db
            .select((TEMPLATE_TABLE, name))
            .await
            .map_err(|e| format!("Could not get template with name: {}, Reason: {}", name, e))
    }

    pub async fn get_all(&self) -> Result<Vec<Template>, String> {
        self.db
            .select(TEMPLATE_TABLE)
            .await
            .map_err(|e| format!("Could not get all templates, Reason: {}", e))
    }

    pub async fn update(&self, template: Template) -> Result<(), String> {
        self.delete(&template.name).await?;
        self.create(template).await?;
        Ok(())
    }
}
