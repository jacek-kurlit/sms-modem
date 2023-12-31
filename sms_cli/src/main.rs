use clap::Parser;
use sms_cli::{
    args_parser::{
        self,
        Commands::Contacts,
        Commands::Send,
        Commands::Templates,
        Commands::{Groups, Import},
    },
    contacts,
};

#[tokio::main]
async fn main() {
    let args = args_parser::Cli::parse();
    init_dependencies().await;
    let result = match args.command {
        Contacts(command) => contacts::manage_contacts(command).await,
        Templates(commad) => sms_cli::templates::manage_templates(commad).await,
        Groups(command) => sms_cli::groups::manage_groups(command).await,
        Send(send_args) => sms_cli::sms_send::send_sms(send_args, &sms_config::get().sms_api).await,
        Import(import_commands) => sms_cli::replace::manage_imports(import_commands).await,
    };
    display_action_message(result);
}

fn display_action_message(result: Result<String, String>) {
    match result {
        Ok(message) => println!("{}", message),
        Err(e) => println!("Error while handling command, Reason: {:?}", e),
    };
}

async fn init_dependencies() {
    let init_result = sms_config::init();
    if let Err(err) = init_result {
        println!("Could not initialize config. Reason {:?}", err);
        std::process::exit(1);
    }
    let init_result = sms_db::repository::init(sms_config::get()).await;
    if let Err(err) = init_result {
        println!("Could not initialize repository. Reason {:?}", err);
        std::process::exit(1);
    }
}
