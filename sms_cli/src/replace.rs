use std::path::PathBuf;

use sms_db::{groups::Group, repository};

use crate::args_parser::ImportCommads;

pub async fn manage_imports(import_commands: ImportCommads) -> Result<String, String> {
    match import_commands {
        ImportCommads::ReplaceContacts {
            source_csv,
            group_name,
        } => handle_replace_contacts(source_csv, group_name).await,
    }
}

async fn handle_replace_contacts(
    source_csv: PathBuf,
    group_name: String,
) -> Result<String, String> {
    if !source_csv.exists() {
        return Err(format!(
            "File {} does not exist",
            source_csv.to_string_lossy()
        ));
    }

    //TODO: pare contacts from csv
    // start trancaction
    // delete all old contacts
    // save all new contacts
    // assign all new contacts to group
    // commit transaction
    let group_id = Group::id_from_name(&group_name);
    let group = repository::groups()
        .get(&group_id)
        .await?
        .ok_or_else(|| format!("Group {} not found", group_name))?;

    Ok("".into())
}

