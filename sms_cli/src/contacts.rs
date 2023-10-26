use crate::args_parser::ContactsCommands;
use sms_db::contacts_repo::*;

pub async fn manage_contact(cmd: ContactsCommands) {
    match cmd {
        ContactsCommands::Add {
            first_name,
            surname_name,
            phone,
            alias,
        } => {
            let result = add_user(first_name, surname_name, phone, alias).await;
            display_action_message(result, "Contact added");
        }
        _ => {
            display_action_message(Err("Command not supported".to_string()), "");
        }
    };
}

fn display_action_message(result: Result<(), String>, success_message: &str) {
    match result {
        Ok(_) => println!("{}", success_message),
        Err(e) => println!("Error while handling contacts, Reason: {:?}", e),
    };
}

async fn add_user(
    first_name: String,
    surname_name: String,
    phone: String,
    alias: Option<String>,
) -> Result<(), String> {
    println!(
        "Adding contact with first_name: {}, surname_name: {}, phone: {}, alias: {:?}",
        first_name, surname_name, phone, alias
    );
    add_contact(Contact::new(first_name, surname_name, phone, alias)).await
}
