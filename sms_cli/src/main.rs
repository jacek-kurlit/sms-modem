use clap::Parser;
use sms_cli::args_parser::{self, Commands::Send};

#[tokio::main]
async fn main() {
    let args = args_parser::Cli::parse();
    // println!("{:?}", args);
    match args.command {
        Send(send_args) => {
            let result = sms_cli::sms_send::send_sms(send_args).await;
            match result {
                Ok(_) => {
                    println!("Message sent");
                }
                Err(e) => {
                    println!("Could not send message, Reason: {:?}", e);
                }
            }
        }
        _ => {
            println!("Command not supported");
        }
    }
}
