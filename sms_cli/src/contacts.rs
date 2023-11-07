use crate::args_parser::ContactsCommands;
use prettytable::row;
use sms_db::{contacts_repo::*, RepositoriesManager};

pub async fn manage_contacts(cmd: ContactsCommands) -> Result<String, String> {
    match cmd {
        ContactsCommands::Create {
            first_name,
            surname_name,
            phone,
            contact_name,
        } => handle_create_contact(first_name, surname_name, phone, contact_name).await,
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
    RepositoriesManager::new()
        .await?
        .contacts()
        .create(Contact::new(first_name, surname_name, phone, contact_name))
        .await
        .map(|_| "Contact created".to_string())
}

async fn handle_delete_contact(contact_name: String) -> Result<String, String> {
    println!("Deleting contact with name: {}", contact_name);
    //FIXME: contact could have been assignted to group!
    RepositoriesManager::new()
        .await?
        .contacts()
        .delete(&contact_name)
        .await
        .map(|_| "Contact deleted".to_string())
}

async fn handle_get_contact(contact_name: String) -> Result<String, String> {
    println!("Getting contact with name: {}", contact_name);
    RepositoriesManager::new()
        .await?
        .contacts()
        .get(&contact_name)
        .await?
        .map(|contact| render_contact_table(vec![contact]))
        .ok_or_else(|| format!("Contact with name: {} not found", contact_name))
}

async fn handle_list_contacts() -> Result<String, String> {
    println!("Getting all contacts");
    let contacts = RepositoriesManager::new()
        .await?
        .contacts()
        .get_all()
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
    //FIXME: contact could have been assignted to group!
    // instead of using contact name we should use db ids!
    RepositoriesManager::new()
        .await?
        .contacts()
        .update(
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
