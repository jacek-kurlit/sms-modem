use prettytable::row;
use sms_db::{groups::Group, RecordEntity, RepositoriesManager};

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
    //TODO: delete all edges to this group
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
    RepositoriesManager::new()
        .await?
        .groups()
        .find_one_by_name(&name)
        .await?
        .map(|group| render_group_table(vec![group]))
        .ok_or_else(|| format!("Group with name: {} not found", name))
}

fn render_group_table(groups: Vec<Group>) -> String {
    let mut table = prettytable::Table::new();
    //FIXME: should we fetch or add some info what contact is assigned to this group?
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
    //FIXME: use graph edges to represent this relation
    let persisted_contact = repository_manager
        .contacts()
        .find_by_contact_name(&contact_name)
        .await?;
    if persisted_contact.is_none() {
        return Err(format!(
            "Cannot add contact {} to group {}. Reason: Contact does not exists",
            contact_name, group_name
        ));
    }
    let group_repository = repository_manager.groups();
    let persited_group = group_repository.find_one_by_name(&group_name).await?;
    if persited_group.is_none() {
        return Err(format!(
            "Cannot add contact {} to group {}. Reason: Group does not exists",
            contact_name, group_name
        ));
    }
    //FIXME: use graph edges to represent this relation
    println!("persited_group: {:?}", persited_group);
    let persited_group = persited_group.unwrap();
    group_repository
        .update(persited_group)
        .await
        .map(|_| "Contact added to group successfully".to_string())
}

async fn handle_group_unassign(contact_name: String, group_name: String) -> Result<String, String> {
    //FIXME: use graph edges to represent this relation
    let group_repository = RepositoriesManager::new().await?.groups();
    let persited_group = group_repository.find_one_by_name(&group_name).await?;
    if persited_group.is_none() {
        return Err(format!(
            "Cannot remove contact {} from group {}. Reason: Group does not exists",
            contact_name, group_name
        ));
    }
    let persited_group = persited_group.unwrap();

    group_repository
        .update(persited_group)
        .await
        .map(|_| "Contact removed from group successfully".to_string())
}
