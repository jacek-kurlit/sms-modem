use prettytable::row;
use sms_db::{
    contacts::Contact,
    groups::{Group, GroupDetails},
    sms_repository::SmsRepository,
    RecordEntity, RepositoriesManager,
};

use crate::args_parser::GroupsCommands;

pub async fn manage_groups(cmd: GroupsCommands) -> Result<String, String> {
    match cmd {
        GroupsCommands::Create { name } => handle_create_group(name).await,
        GroupsCommands::Delete { name } => handle_delete_group(name).await,
        GroupsCommands::Get { name } => handle_get_group(name).await,
        GroupsCommands::List => handle_list_groups().await,
        GroupsCommands::Assign {
            contact_name,
            group_name,
        } => handle_group_assign(contact_name, group_name).await,
        GroupsCommands::Unassign {
            contact_name,
            group_name,
        } => handle_group_unassign(contact_name, group_name).await,
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
    let groups_repo = RepositoriesManager::new().await?.groups();
    let persited_group = groups_repo.find_one_by_name(&name).await?.ok_or_else(|| {
        format!(
            "Cannot delete group with name: {}. Reason: Group does not exists",
            name
        )
    })?;
    groups_repo
        .delete(persited_group.id())
        .await
        .map(|_| "Group deleted successfully".to_string())
}

async fn handle_get_group(name: String) -> Result<String, String> {
    let group_repository = RepositoriesManager::new().await?.groups();
    let persited_group = find_group(&group_repository, name).await?;
    let details = group_repository
        .find_group_details(persited_group.id())
        .await?
        .ok_or_else(|| {
            format!(
                "Could not find group details. Reason: Group with id '{}' does not exists",
                persited_group.id()
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

async fn handle_group_assign(contact_name: String, group_name: String) -> Result<String, String> {
    let repository_manager = RepositoriesManager::new().await?;
    let persisted_contact = find_contact(&repository_manager.contacts(), &contact_name).await?;
    let group_repository = repository_manager.groups();
    let persited_group = find_group(&group_repository, group_name).await?;
    group_repository
        .assign_contact(persisted_contact.id(), persited_group.id())
        .await
        .map(|_| "Contact added to group successfully".to_string())
}

async fn find_contact(
    contact_repository: &SmsRepository<Contact>,
    contact_name: &String,
) -> Result<sms_db::contacts::Contact, String> {
    contact_repository
        .find_by_contact_name(contact_name)
        .await?
        .ok_or_else(|| {
            format!(
                "Could not find contact {}. Reason: Contact does not exists",
                contact_name
            )
        })
}

async fn find_group(
    group_repository: &sms_db::sms_repository::SmsRepository<Group>,
    group_name: String,
) -> Result<Group, String> {
    group_repository
        .find_one_by_name(&group_name)
        .await?
        .ok_or_else(|| {
            format!(
                "Could not find group {}. Reason: Group does not exists",
                group_name
            )
        })
}

async fn handle_group_unassign(contact_name: String, group_name: String) -> Result<String, String> {
    let repository_manager = RepositoriesManager::new().await?;
    let group_repository = repository_manager.groups();
    let persited_group = find_group(&group_repository, group_name).await?;
    let persited_contact = find_contact(&repository_manager.contacts(), &contact_name).await?;
    group_repository
        .unassign_contact(persited_contact.id(), persited_group.id())
        .await
        .map(|_| "Contact removed from group successfully".to_string())
}
