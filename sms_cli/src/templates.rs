use prettytable::row;
use sms_db::templates_repo::{self, Template};

use crate::args_parser::TemplatesCommands;

pub async fn manage_templates(_cmd: TemplatesCommands) -> Result<String, String> {
    match _cmd {
        TemplatesCommands::Create { alias, text } => handle_create_template(alias, text).await,
        TemplatesCommands::Delete { alias } => handle_delete_template(alias).await,
        TemplatesCommands::Get { alias } => handle_get_template(alias).await,
        TemplatesCommands::Update { alias, text } => handle_update_template(alias, text).await,
        TemplatesCommands::List => handle_list_templates().await,
    }
}

async fn handle_create_template(alias: String, text: String) -> Result<String, String> {
    templates_repo::TemplateRepository::new()
        .await?
        .add_template(Template::new(alias, text))
        .await
        .map(|_| "Template created successfully".to_string())
}

async fn handle_delete_template(alias: String) -> Result<String, String> {
    templates_repo::TemplateRepository::new()
        .await?
        .delete_template(&alias)
        .await
        .map(|_| "Template deleted successfully".to_string())
}

async fn handle_get_template(alias: String) -> Result<String, String> {
    templates_repo::TemplateRepository::new()
        .await?
        .get_template(&alias)
        .await?
        .map(|template| render_templates_table(vec![template]))
        .ok_or_else(|| format!("Template with alias {} not found", alias))
}

async fn handle_list_templates() -> Result<String, String> {
    templates_repo::TemplateRepository::new()
        .await?
        .get_all_templates()
        .await
        .map(render_templates_table)
}

fn render_templates_table(templates: Vec<Template>) -> String {
    let mut table = prettytable::Table::new();
    table.add_row(row!["Alias", "Text"]);
    for template in templates {
        table.add_row(row![template.alias, template.text]);
    }
    table.to_string()
}

async fn handle_update_template(alias: String, text: String) -> Result<String, String> {
    templates_repo::TemplateRepository::new()
        .await?
        .update_template(Template::new(alias, text))
        .await
        .map(|_| "Template updated successfully".to_string())
}
