use crate::args_parser::ContactsCommands;
use prettytable::row;
use sms_db::{contacts::*, RecordEntity, RepositoriesManager};

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
    //FIXME: connection between contacts and groups could be resolved by using graph edges
    let contacts = RepositoriesManager::new().await?.contacts();
    let contact_to_delete = contacts
        .find_by_contact_name(&contact_name)
        .await?
        .ok_or_else(|| format!("Contact with name: {} not found", contact_name))?;
    contacts
        .delete(contact_to_delete.id())
        .await
        .map(|_| "Contact deleted".to_string())
}

async fn handle_get_contact(contact_name: String) -> Result<String, String> {
    println!("Getting contact with name: {}", contact_name);
    RepositoriesManager::new()
        .await?
        .contacts()
        .find_by_contact_name(&contact_name)
        .await?
        .ok_or_else(|| format!("Contact with name: {} not found", contact_name))
        .map(|contact| render_contact_table(vec![contact]))
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
    let contacts_repo = RepositoriesManager::new().await?.contacts();
    let contact = contacts_repo
        .find_by_contact_name(&contact_name)
        .await?
        .ok_or_else(|| format!("Contact with name: {} not found", contact_name))?;
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
