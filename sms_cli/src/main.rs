use clap::Parser;
use sms_cli::{
    args_parser::{self, Commands::Contacts, Commands::Send},
    contacts,
};

const SERVICE_URL: &str = "http://192.168.1.1";

#[tokio::main]
async fn main() {
    let args = args_parser::Cli::parse();
    // println!("{:?}", args);
    match args.command {
        Contacts(command) => {
            contacts::manage_contact(command).await;
        }
        Send(send_args) => {
            sms_cli::sms_send::send_sms(send_args, SERVICE_URL).await;
        }
        _ => println!("Command not supported"),
    };
}
