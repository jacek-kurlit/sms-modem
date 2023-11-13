use prettytable::row;
use sms_db::{
    groups::{Group, GroupDetails},
    RecordEntity, RepositoriesManager,
};

use crate::args_parser::{AssignGroupArgs, GroupsCommands};

pub async fn manage_groups(cmd: GroupsCommands) -> Result<String, String> {
    match cmd {
        GroupsCommands::Create { name } => handle_create_group(name).await,
        GroupsCommands::Delete { name } => handle_delete_group(name).await,
        GroupsCommands::Get { name } => handle_get_group(name).await,
        GroupsCommands::List => handle_list_groups().await,
        GroupsCommands::Assign(assignment_args) => handle_group_assign(assignment_args).await,
        GroupsCommands::Unassign(assignment_args) => handle_group_unassign(assignment_args).await,
    }
}

async fn handle_create_group(name: String) -> Result<String, String> {
    RepositoriesManager::new()
        .await?
        .groups()
        .create(Group::new(name))
        .await
        .map(|_| "Group created successfully".to_string())
}

async fn handle_delete_group(name: String) -> Result<String, String> {
    RepositoriesManager::new()
        .await?
        .groups()
        .delete(&Group::id_from_name(&name))
        .await
        .map(|_| "Group deleted successfully".to_string())
}

async fn handle_get_group(name: String) -> Result<String, String> {
    let group_id = Group::id_from_name(&name);
    let details = RepositoriesManager::new()
        .await?
        .groups()
        .find_group_details(&group_id)
        .await?
        .ok_or_else(|| {
            format!(
                "Could not find group details. Reason: Group with id '{}' does not exists",
                group_id
            )
        })?;

    Ok(render_group_details_table(details))
}

fn render_group_details_table(group_detail: GroupDetails) -> String {
    let mut table = prettytable::Table::new();
    table.add_row(row!["Contact Name"]);
    for contact in group_detail.contacts {
        table.add_row(row![contact.contact_name]);
    }
    table.to_string()
}

fn render_group_table(groups: Vec<Group>) -> String {
    let mut table = prettytable::Table::new();
    table.add_row(row!["Name"]);
    for group in groups {
        table.add_row(row![group.name]);
    }
    table.to_string()
}

async fn handle_list_groups() -> Result<String, String> {
    RepositoriesManager::new()
        .await?
        .groups()
        .get_all()
        .await
        .map(render_group_table)
}

async fn handle_group_assign(assignment_args: AssignGroupArgs) -> Result<String, String> {
    let repository_manager = RepositoriesManager::new().await?;
    let persisted_contact = repository_manager
        .contacts()
        .find_exatcly_one_by_contact_name(
            &assignment_args.contact_target.contact_name,
            assignment_args.contact_target.index,
        )
        .await?;
    repository_manager
        .groups()
        .assign_contact(
            persisted_contact.id(),
            &Group::id_from_name(&assignment_args.group_name),
        )
        .await
        .map(|_| "Contact added to group successfully".to_string())
}

async fn handle_group_unassign(assignment_args: AssignGroupArgs) -> Result<String, String> {
    let repository_manager = RepositoriesManager::new().await?;
    let persited_contact = repository_manager
        .contacts()
        .find_exatcly_one_by_contact_name(
            &assignment_args.contact_target.contact_name,
            assignment_args.contact_target.index,
        )
        .await?;
    repository_manager
        .groups()
        .unassign_contact(
            persited_contact.id(),
            &Group::id_from_name(&assignment_args.group_name),
        )
        .await
        .map(|_| "Contact removed from group successfully".to_string())
}
