use crate::args_parser::{ContactTargetArgs, ContactUpdateArgs, ContactsCommands};
use prettytable::row;
use sms_db::{contacts::*, repository};

pub async fn manage_contacts(cmd: ContactsCommands) -> Result<String, String> {
    match cmd {
        ContactsCommands::Create {
            first_name,
            surname_name,
            phone,
            contact_name,
        } => handle_create_contact(first_name, surname_name, phone, contact_name).await,
        ContactsCommands::Delete(contact_target) => handle_delete_contact(contact_target).await,
        ContactsCommands::Get(contact_target) => handle_get_contact(contact_target).await,
        ContactsCommands::List => handle_list_contacts().await,
        ContactsCommands::Update(update_args) => handle_update_contact(update_args).await,
    }
}

async fn handle_create_contact(
    first_name: String,
    surname_name: String,
    phone: String,
    contact_name: Option<String>,
) -> Result<String, String> {
    println!(
        "Creating contact with first_name: {}, surname_name: {}, phone: {}, contact name: {:?}",
        first_name, surname_name, phone, contact_name
    );
    repository::contacts()
        .create(Contact::new(first_name, surname_name, phone, contact_name))
        .await
        .map(|_| "Contact created".to_string())
}

async fn handle_delete_contact(target_contact: ContactTargetArgs) -> Result<String, String> {
    let contact_name = target_contact.contact_name;
    println!("Deleting contact with name: {}", contact_name);
    let contacts = repository::contacts();
    let contacts_to_delete = contacts
        .find_exactly_one_by_contact_name(&contact_name, target_contact.index)
        .await?;
    contacts
        .delete(&contacts_to_delete.id)
        .await
        .map(|_| "Contact deleted".to_string())
}

async fn handle_get_contact(target_contact: ContactTargetArgs) -> Result<String, String> {
    let contact_name = target_contact.contact_name;
    println!("Getting contact with name: {}", contact_name);
    let contacts = repository::contacts()
        .find_all_or_select_at_index(&contact_name, target_contact.index)
        .await?;
    Ok(render_contact_table(contacts))
}

async fn handle_list_contacts() -> Result<String, String> {
    println!("Getting all contacts");
    let contacts = repository::contacts().get_all().await?;
    Ok(render_contact_table(contacts))
}

async fn handle_update_contact(update_args: ContactUpdateArgs) -> Result<String, String> {
    let ContactUpdateArgs {
        contact_target,
        first_name,
        surname_name,
        phone,
        new_contact_name,
    } = update_args;
    println!(
        "Updating contact with name {} and setting fields to first_name: {}, surname_name: {}, phone: {}, contact name: {:?}",
        contact_target.contact_name,first_name, surname_name, phone, new_contact_name
    );
    let contacts_repo = repository::contacts();
    let contact = contacts_repo
        .find_exactly_one_by_contact_name(&contact_target.contact_name, contact_target.index)
        .await?;
    contacts_repo
        .update(Contact::new_with_id(
            contact.id,
            first_name,
            surname_name,
            phone,
            new_contact_name,
        ))
        .await
        .map(|_| "Contact updated".to_string())
}

fn render_contact_table(contacts: Vec<Contact>) -> String {
    let mut table = prettytable::Table::new();
    table.add_row(row![
        "#",
        "Contact Name",
        "First Name",
        "Surname Name",
        "Phone"
    ]);
    for (index, contact) in contacts.into_iter().enumerate() {
        table.add_row(row![
            index,
            contact.contact_name,
            contact.first_name,
            contact.surname_name,
            contact.phone
        ]);
    }
    table.to_string()
}
