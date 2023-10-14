use sms_api::SmsError;

use crate::args_parser::SendSmsArgs;

pub async fn send_sms(send_args: SendSmsArgs, service_url: &str) {
    let number = send_args
        .to
        .number
        .expect("Currently only number is supported");
    let message = send_args
        .message
        .plain
        .expect("Currently only plain message is supported");

    let result = send(&message, &[&number], service_url).await;

    match result {
        Ok(_) => {
            println!("Message sent");
        }
        Err(e) => {
            println!("Could not send message, Reason: {:?}", e);
        }
    };
}

async fn send(message: &str, numbers: &[&str], service_url: &str) -> Result<(), SmsError> {
    println!("Sending sms to {:?} with message '{}'", numbers, message);
    sms_api::SmsService::new(service_url)?
        .send_sms(message, numbers)
        .await
}
