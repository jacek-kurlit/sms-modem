use std::result;

use clap::Parser;
use sms_cli::{
    args_parser::{
        self, Commands::Contacts, Commands::Groups, Commands::Send, Commands::Templates,
    },
    contacts,
};
use sms_db::sms_repository::TestData;

const SERVICE_URL: &str = "http://192.168.1.1";

#[tokio::main]
async fn main() {
    let result = sms_db::RepositoriesManager::new().await;
    if let Ok(result) = result {
        let created = result
            .test()
            .create(TestData::new("1".into(), "lol".into()))
            .await
            .unwrap();
        println!("{:?}", created);
        let get = result
            .test()
            .getv2(&created.id.unwrap())
            .await
            .unwrap()
            .unwrap();
        println!("{:?}", get);
    }
    // let args = args_parser::Cli::parse();
    // let result = match args.command {
    //     Contacts(command) => contacts::manage_contacts(command).await,
    //     Templates(commad) => sms_cli::templates::manage_templates(commad).await,
    //     Groups(command) => sms_cli::groups::manage_groups(command).await,
    //     Send(send_args) => sms_cli::sms_send::send_sms(send_args, SERVICE_URL).await,
    // };
    // display_action_message(result);
}

fn display_action_message(result: Result<String, String>) {
    match result {
        Ok(message) => println!("{}", message),
        Err(e) => println!("Error while handling command, Reason: {:?}", e),
    };
}
