use prettytable::row;
use sms_db::templates_repo::{self, Template};

use crate::args_parser::TemplatesCommands;

pub async fn manage_templates(cmd: TemplatesCommands) -> Result<String, String> {
    match cmd {
        TemplatesCommands::Create { name, text } => handle_create_template(name, text).await,
        TemplatesCommands::Delete { name } => handle_delete_template(name).await,
        TemplatesCommands::Get { name } => handle_get_template(name).await,
        TemplatesCommands::Update { name, text } => handle_update_template(name, text).await,
        TemplatesCommands::List => handle_list_templates().await,
    }
}

async fn handle_create_template(name: String, text: String) -> Result<String, String> {
    templates_repo::TemplateRepository::new()
        .await?
        .add_template(Template::new(name, text))
        .await
        .map(|_| "Template created successfully".to_string())
}

async fn handle_delete_template(name: String) -> Result<String, String> {
    templates_repo::TemplateRepository::new()
        .await?
        .delete_template(&name)
        .await
        .map(|_| "Template deleted successfully".to_string())
}

async fn handle_get_template(name: String) -> Result<String, String> {
    templates_repo::TemplateRepository::new()
        .await?
        .get_template(&name)
        .await?
        .map(|template| render_templates_table(vec![template]))
        .ok_or_else(|| format!("Template with name {} not found", name))
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
    table.add_row(row!["name", "Text"]);
    for template in templates {
        table.add_row(row![template.name, template.text]);
    }
    table.to_string()
}

async fn handle_update_template(name: String, text: String) -> Result<String, String> {
    templates_repo::TemplateRepository::new()
        .await?
        .update_template(Template::new(name, text))
        .await
        .map(|_| "Template updated successfully".to_string())
}
