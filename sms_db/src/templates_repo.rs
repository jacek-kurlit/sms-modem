use serde::{Deserialize, Serialize};
use surrealdb::Surreal;

use crate::{repository, AnyRecord};

#[derive(Debug, Serialize, Deserialize)]
pub struct Template {
    pub alias: String,
    pub text: String,
}

impl Template {
    pub fn new(alias: String, text: String) -> Self {
        Self { alias, text }
    }
}

const TEMPLATE_TABLE: &str = "template";

pub struct TemplateRepository {
    db: Surreal<surrealdb::engine::local::Db>,
}

impl TemplateRepository {
    pub async fn new() -> Result<Self, String> {
        let db = repository::connect_to_db().await?;
        Ok(Self { db })
    }
    pub async fn add_template(&self, template: Template) -> Result<(), String> {
        let id = template.alias.clone();
        let _: Option<AnyRecord> = self
            .db
            .create((TEMPLATE_TABLE, id))
            .content(template)
            .await
            .map_err(|e| format!("Could not create template table {}", e))?;
        Ok(())
    }

    pub async fn delete_template(&self, alias: &str) -> Result<(), String> {
        let _: AnyRecord = self
            .db
            .delete((TEMPLATE_TABLE, alias))
            .await
            .map_err(|e| {
                format!(
                    "Could not delete template with alias: {}, Reason: {}",
                    alias, e
                )
            })?
            .ok_or_else(|| {
                format!(
                    "Could not delete template with alias: {}, Reason: template not found",
                    alias
                )
            })?;
        Ok(())
    }

    pub async fn get_template(&self, alias: &str) -> Result<Option<Template>, String> {
        self.db.select((TEMPLATE_TABLE, alias)).await.map_err(|e| {
            format!(
                "Could not get template with alias: {}, Reason: {}",
                alias, e
            )
        })
    }

    pub async fn get_all_templates(&self) -> Result<Vec<Template>, String> {
        self.db
            .select(TEMPLATE_TABLE)
            .await
            .map_err(|e| format!("Could not get all templates, Reason: {}", e))
    }

    pub async fn update_template(&self, template: Template) -> Result<(), String> {
        self.delete_template(&template.alias).await?;
        self.add_template(template).await?;
        Ok(())
    }
}
