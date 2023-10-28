use clap::Parser;
use sms_cli::{
    args_parser::{self, Commands::Contacts, Commands::Send, Commands::Templates},
    contacts,
};

const SERVICE_URL: &str = "http://192.168.1.1";

#[tokio::main]
async fn main() {
    let args = args_parser::Cli::parse();
    let result = match args.command {
        Contacts(command) => contacts::manage_contacts(command).await,
        Templates(commad) => sms_cli::templates::manage_templates(commad).await,
        Send(send_args) => sms_cli::sms_send::send_sms(send_args, SERVICE_URL).await,
        _ => Err("Command not supported".to_string()),
    };
    display_action_message(result);
}

fn display_action_message(result: Result<String, String>) {
    match result {
        Ok(message) => println!("{}", message),
        Err(e) => println!("Error while handling command, Reason: {:?}", e),
    };
}
