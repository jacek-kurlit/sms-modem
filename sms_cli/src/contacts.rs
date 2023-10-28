use crate::{args_parser::ContactsCommands, contacts};
use prettytable::row;
use sms_db::contacts_repo::*;

pub async fn manage_contacts(cmd: ContactsCommands) -> Result<String, String> {
    match cmd {
        ContactsCommands::Add {
            first_name,
            surname_name,
            phone,
            contact_name,
        } => handle_add_contact(first_name, surname_name, phone, contact_name).await,
        ContactsCommands::Delete { contact_name } => handle_delete_contact(contact_name).await,
        ContactsCommands::Get { contact_name } => handle_get_contact(contact_name).await,
        ContactsCommands::List => handle_list_contacts().await,
        ContactsCommands::Update {
            contact_name,
            first_name,
            surname_name,
            phone,
            new_contact_name,
        } => {
            handle_update_contact(
                contact_name,
                first_name,
                surname_name,
                phone,
                new_contact_name,
            )
            .await
        }
    }
}

async fn handle_add_contact(
    first_name: String,
    surname_name: String,
    phone: String,
    contact_name: Option<String>,
) -> Result<String, String> {
    println!(
        "Adding contact with first_name: {}, surname_name: {}, phone: {}, contact name: {:?}",
        first_name, surname_name, phone, contact_name
    );
    contacts::ContactRepository::new()
        .await?
        .add_contact(Contact::new(first_name, surname_name, phone, contact_name))
        .await
        .map(|_| "Contact added".to_string())
}

async fn handle_delete_contact(contact_name: String) -> Result<String, String> {
    println!("Deleting contact with name: {}", contact_name);
    contacts::ContactRepository::new()
        .await?
        .delete_contact(&contact_name)
        .await
        .map(|_| "Contact deleted".to_string())
}

async fn handle_get_contact(contact_name: String) -> Result<String, String> {
    println!("Getting contact with name: {}", contact_name);
    contacts::ContactRepository::new()
        .await?
        .get_contact(&contact_name)
        .await?
        .map(|contact| render_contact_table(vec![contact]))
        .ok_or_else(|| format!("Contact with name: {} not found", contact_name))
}

async fn handle_list_contacts() -> Result<String, String> {
    println!("Getting all contacts");
    let contacts = contacts::ContactRepository::new()
        .await?
        .get_all_contacts()
        .await?;
    Ok(render_contact_table(contacts))
}

async fn handle_update_contact(
    contact_name: String,
    first_name: String,
    surname_name: String,
    phone: String,
    new_contact_name: Option<String>,
) -> Result<String, String> {
    println!(
        "Updating contact with name {} and setting fields to first_name: {}, surname_name: {}, phone: {}, contact name: {:?}",
        contact_name,first_name, surname_name, phone, new_contact_name
    );
    contacts::ContactRepository::new()
        .await?
        .update_contact(
            &contact_name,
            Contact::new(first_name, surname_name, phone, new_contact_name),
        )
        .await
        .map(|_| "Contact updated".to_string())
}

fn render_contact_table(contacts: Vec<Contact>) -> String {
    let mut table = prettytable::Table::new();
    table.add_row(row!["Contact Name", "First Name", "Surname Name", "Phone"]);
    for contact in contacts {
        table.add_row(row![
            contact.contact_name,
            contact.first_name,
            contact.surname_name,
            contact.phone
        ]);
    }
    table.to_string()
}
