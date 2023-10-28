use prettytable::row;
use sms_db::groups_repo::Group;

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
    sms_db::groups_repo::GroupRepository::new()
        .await?
        .create_group(sms_db::groups_repo::Group::new(name))
        .await
        .map(|_| "Group created successfully".to_string())
}

async fn handle_delete_group(name: String) -> Result<String, String> {
    sms_db::groups_repo::GroupRepository::new()
        .await?
        .delete_group(&name)
        .await
        .map(|_| "Group deleted successfully".to_string())
}

async fn handle_get_group(name: String) -> Result<String, String> {
    sms_db::groups_repo::GroupRepository::new()
        .await?
        .get_group(&name)
        .await?
        .map(|group| render_group_table(vec![group]))
        .ok_or_else(|| format!("Group with name: {} not found", name))
}

fn render_group_table(groups: Vec<Group>) -> String {
    let mut table = prettytable::Table::new();
    //TODO: probably embedded table would be better
    table.add_row(row!["Name", "Assigned contacts"]);
    for group in groups {
        table.add_row(row![group.name, group.assigned_contacts.join(", ")]);
    }
    table.to_string()
}

async fn handle_list_groups() -> Result<String, String> {
    sms_db::groups_repo::GroupRepository::new()
        .await?
        .get_all_groups()
        .await
        .map(render_group_table)
}

async fn handle_group_assign(contact_name: String, group_name: String) -> Result<String, String> {
    let persisted_contact = sms_db::contacts_repo::ContactRepository::new()
        .await?
        .get_contact(&contact_name)
        .await?;
    if persisted_contact.is_none() {
        return Err(format!(
            "Cannot add contact {} to group {}. Reason: Contact does not exists",
            contact_name, group_name
        ));
    }
    //FIXME: We cannot have 2 repos working on the same db unless they share connection?
    let group_repository = sms_db::groups_repo::GroupRepository::new().await?;
    let persited_group = group_repository.get_group(&group_name).await?;
    if persited_group.is_none() {
        return Err(format!(
            "Cannot add contact {} to group {}. Reason: Group does not exists",
            contact_name, group_name
        ));
    }
    println!("persited_group: {:?}", persited_group);
    let mut persited_group = persited_group.unwrap();
    persited_group.assigned_contacts.push(contact_name);
    group_repository
        .update_group(persited_group)
        .await
        .map(|_| "Contact added to group successfully".to_string())
}

async fn handle_group_unassign(contact_name: String, group_name: String) -> Result<String, String> {
    let group_repository = sms_db::groups_repo::GroupRepository::new().await?;
    let persited_group = group_repository.get_group(&group_name).await?;
    if persited_group.is_none() {
        return Err(format!(
            "Cannot remove contact {} from group {}. Reason: Group does not exists",
            contact_name, group_name
        ));
    }
    let mut persited_group = persited_group.unwrap();
    if !persited_group.assigned_contacts.contains(&contact_name) {
        return Err(format!(
            "Cannot remove contact {} from group {}. Reason: Contact is not assigned to group",
            contact_name, group_name
        ));
    }
    persited_group
        .assigned_contacts
        .retain(|c| c != &contact_name);
    group_repository
        .update_group(persited_group)
        .await
        .map(|_| "Contact removed from group successfully".to_string())
}
